use generated_semconv::options::InstrumentationOptions;
use generated_semconv::{logger, meter, probeattr, probeevent, probemetric, scope};
use opentelemetry::logs::{AnyValue, LogRecord, Logger, LoggerProvider, Severity};
use opentelemetry::metrics::{
    AsyncInstrument, AsyncInstrumentBuilder, Callback, Counter, InstrumentBuilder,
    InstrumentProvider, Meter, MeterProvider, ObservableCounter, SyncInstrument,
};
use opentelemetry::{Context, InstrumentationScope, Key, KeyValue};
use std::alloc::{GlobalAlloc, Layout, System};
use std::borrow::Cow;
use std::cell::Cell;
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

thread_local! {
    static ALLOCATIONS: Cell<Option<usize>> = const { Cell::new(None) };
}

struct AllocationCounter;

unsafe impl GlobalAlloc for AllocationCounter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _ = ALLOCATIONS.try_with(|count| {
            if let Some(value) = count.get() {
                count.set(Some(value + 1));
            }
        });
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let _ = ALLOCATIONS.try_with(|count| {
            if let Some(value) = count.get() {
                count.set(Some(value + 1));
            }
        });
        unsafe { System.realloc(pointer, layout, new_size) }
    }
}

#[global_allocator]
static ALLOCATOR: AllocationCounter = AllocationCounter;

fn allocations(work: impl FnOnce()) -> usize {
    ALLOCATIONS.with(|count| count.set(Some(0)));
    work();
    ALLOCATIONS.with(|count| count.replace(None).unwrap())
}

#[derive(Default)]
struct MetricState {
    attribute_count: AtomicUsize,
    sum: AtomicI64,
    order: AtomicU64,
    callbacks: Mutex<Vec<Callback<u64>>>,
}

struct MetricProvider(Arc<MetricState>);

impl MeterProvider for MetricProvider {
    fn meter_with_scope(&self, _: InstrumentationScope) -> Meter {
        Meter::new(Arc::new(Self(self.0.clone())))
    }
}

impl InstrumentProvider for MetricProvider {
    fn u64_counter(&self, _: InstrumentBuilder<'_, Counter<u64>>) -> Counter<u64> {
        Counter::new(Arc::new(Self(self.0.clone())))
    }
    fn u64_observable_counter(
        &self,
        builder: AsyncInstrumentBuilder<'_, ObservableCounter<u64>, u64>,
    ) -> ObservableCounter<u64> {
        self.0.callbacks.lock().unwrap().extend(builder.callbacks);
        ObservableCounter::new()
    }
}

impl AsyncInstrument<u64> for MetricProvider {
    fn observe(&self, value: u64, attributes: &[KeyValue]) {
        self.measure(value, attributes);
    }
}

struct BorrowedMeterProvider {
    meter: Meter,
    scope: Mutex<Option<InstrumentationScope>>,
}

impl MeterProvider for BorrowedMeterProvider {
    fn meter_with_scope(&self, scope: InstrumentationScope) -> Meter {
        *self.scope.lock().unwrap() = Some(scope);
        self.meter.clone()
    }
}

impl SyncInstrument<u64> for MetricProvider {
    fn measure(&self, _: u64, attributes: &[KeyValue]) {
        black_box(attributes);
        self.0
            .attribute_count
            .store(attributes.len(), Ordering::Relaxed);
        self.0.sum.store(
            attributes
                .iter()
                .map(|attr| match attr.value {
                    opentelemetry::Value::I64(value) => value,
                    _ => 0,
                })
                .sum(),
            Ordering::Relaxed,
        );
        self.0.order.store(
            attributes
                .iter()
                .fold(0_u64, |hash, attr| match attr.value {
                    opentelemetry::Value::I64(value) => {
                        hash.wrapping_mul(31).wrapping_add(value as u64)
                    }
                    _ => hash,
                }),
            Ordering::Relaxed,
        );
    }
}

#[derive(Default)]
struct LogState {
    enabled: AtomicBool,
    attribute_count: AtomicUsize,
    attribute_sum: AtomicI64,
    severity: AtomicUsize,
    context_attached_during_check: AtomicBool,
    order: AtomicU64,
    timestamp: AtomicU64,
    scope: Mutex<Option<InstrumentationScope>>,
}

struct ExplicitContext;
struct LogProvider(Arc<LogState>);
struct FakeLogger(Arc<LogState>);

impl LoggerProvider for LogProvider {
    type Logger = FakeLogger;

    fn logger_with_scope(&self, scope: InstrumentationScope) -> Self::Logger {
        *self.0.scope.lock().unwrap() = Some(scope);
        FakeLogger(self.0.clone())
    }
}

#[derive(Default)]
struct FakeRecord {
    attributes: usize,
    sum: i64,
    severity: usize,
    order: u64,
    timestamp: u64,
}

impl LogRecord for FakeRecord {
    fn set_event_name(&mut self, _: &'static str) {}
    fn set_target<T: Into<Cow<'static, str>>>(&mut self, _: T) {}
    fn set_timestamp(&mut self, value: SystemTime) {
        self.timestamp = value.duration_since(UNIX_EPOCH).unwrap().as_secs();
    }
    fn set_observed_timestamp(&mut self, _: SystemTime) {}
    fn set_severity_text(&mut self, _: &'static str) {}
    fn set_severity_number(&mut self, value: Severity) {
        self.severity = value as usize;
    }
    fn set_body(&mut self, _: AnyValue) {}
    fn add_attributes<I, K, V>(&mut self, values: I)
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<Key>,
        V: Into<AnyValue>,
    {
        for (key, value) in values {
            self.add_attribute(key, value);
        }
    }
    fn add_attribute<K: Into<Key>, V: Into<AnyValue>>(&mut self, key: K, value: V) {
        black_box(key.into());
        self.attributes += 1;
        if let AnyValue::Int(value) = value.into() {
            self.sum += value;
            self.order = self.order.wrapping_mul(31).wrapping_add(value as u64);
        }
    }
}

impl Logger for FakeLogger {
    type LogRecord = FakeRecord;
    fn create_log_record(&self) -> FakeRecord {
        FakeRecord::default()
    }
    fn emit(&self, record: FakeRecord) {
        self.0
            .attribute_count
            .store(record.attributes, Ordering::Relaxed);
        self.0.attribute_sum.store(record.sum, Ordering::Relaxed);
        self.0.severity.store(record.severity, Ordering::Relaxed);
        self.0.order.store(record.order, Ordering::Relaxed);
        self.0.timestamp.store(record.timestamp, Ordering::Relaxed);
    }
    fn event_enabled(&self, _: Severity, _: &str, _: Option<&str>) -> bool {
        self.0.context_attached_during_check.store(
            Context::current().get::<ExplicitContext>().is_some(),
            Ordering::Relaxed,
        );
        self.0.enabled.load(Ordering::Relaxed)
    }
}

#[test]
fn metrics_use_no_generated_heap_for_required_or_typical_optional_attributes() {
    let state = Arc::new(MetricState::default());
    let meter = meter::Meter::new(
        &MetricProvider(state.clone()),
        InstrumentationOptions {
            scope: "allocation-test".into(),
        },
    );
    let required = probemetric::RequiredCounter::new(&meter);
    let optional = probemetric::OptionalCounter::new(&meter);
    let required_count = allocations(|| required.add(1, probeattr::RequiredAttr::from(3)));
    assert_eq!(
        required_count, 0,
        "required metric attribute buffer allocated"
    );
    let optional_count = allocations(|| {
        optional.add(
            1,
            probeattr::RequiredAttr::from(3),
            [probemetric::OptionalCounterAttr::Optional(
                probeattr::OptionalAttr::from(5),
            )],
        )
    });
    assert_eq!(
        optional_count, 0,
        "ordinary optional metric attributes allocated"
    );
    assert_eq!(state.attribute_count.load(Ordering::Relaxed), 2);
    assert_eq!(state.sum.load(Ordering::Relaxed), 8);
    optional.add(
        1,
        probeattr::RequiredAttr::from(3),
        (0..20).map(|value| {
            probemetric::OptionalCounterAttr::Optional(probeattr::OptionalAttr::from(value))
        }),
    );
    assert_eq!(state.attribute_count.load(Ordering::Relaxed), 21);
    assert_eq!(state.sum.load(Ordering::Relaxed), 193);
    assert_eq!(
        state.order.load(Ordering::Relaxed),
        (0..20_u64).fold(3_u64, |hash, value| hash
            .wrapping_mul(31)
            .wrapping_add(value))
    );
    println!("generated metric allocations: required={required_count}, optional={optional_count}");
}

#[test]
fn observable_measurements_use_stack_attribute_storage() {
    let state = Arc::new(MetricState::default());
    let provider = MetricProvider(state.clone());
    let meter = meter::Meter::new(
        &provider,
        InstrumentationOptions {
            scope: "allocation-test".into(),
        },
    );
    let _required = probemetric::RequiredObservableCounter::new(&meter, |observer| {
        observer.observe(1, probeattr::RequiredAttr::from(3));
    });
    let _optional = probemetric::OptionalObservableCounter::new(&meter, |observer| {
        observer.observe(
            1,
            probeattr::RequiredAttr::from(3),
            [probemetric::OptionalCounterAttr::Optional(
                probeattr::OptionalAttr::from(5),
            )],
        );
    });
    let callbacks = state.callbacks.lock().unwrap();
    let count = allocations(|| {
        for callback in callbacks.iter() {
            callback(&provider);
        }
    });
    assert_eq!(count, 0);
    assert_eq!(state.attribute_count.load(Ordering::Relaxed), 2);
    assert_eq!(state.sum.load(Ordering::Relaxed), 8);
    println!("generated observable metric allocations: {count}");
}

#[test]
fn static_meter_and_logger_scope_names_remain_borrowed() {
    let metric_provider = BorrowedMeterProvider {
        meter: Meter::new(Arc::new(MetricProvider(Arc::new(MetricState::default())))),
        scope: Mutex::new(None),
    };
    drop(metric_provider.scope.lock().unwrap());
    let metric_count = allocations(|| {
        black_box(meter::Meter::new(
            &metric_provider,
            InstrumentationOptions {
                scope: "allocation-test".into(),
            },
        ));
    });
    let log_provider = LogProvider(Arc::new(LogState::default()));
    drop(log_provider.0.scope.lock().unwrap());
    let log_count = allocations(|| {
        black_box(logger::Logger::new(
            &log_provider,
            InstrumentationOptions {
                scope: "allocation-test".into(),
            },
        ));
    });
    assert_eq!(metric_count, 0);
    assert_eq!(log_count, 0);
    println!("generated static scope allocations: meter={metric_count}, logger={log_count}");
}

#[test]
fn configured_scopes_move_metadata_without_rebuilding_storage() {
    let metric_provider = BorrowedMeterProvider {
        meter: Meter::new(Arc::new(MetricProvider(Arc::new(MetricState::default())))),
        scope: Mutex::new(None),
    };
    let log_provider = LogProvider(Arc::new(LogState::default()));
    drop(metric_provider.scope.lock().unwrap());
    drop(log_provider.0.scope.lock().unwrap());
    let scope = || scope::Scope {
        name: String::from("allocation-test").into(),
        version: Some(String::from("1.2.3").into()),
        attributes: vec![KeyValue::new("scope.identity", 7_i64)],
    };
    let metric_scope = scope();
    let log_scope = scope();
    let original = |scope: &scope::Scope| {
        (
            scope.name.as_ptr(),
            scope.version.as_ref().unwrap().as_ptr(),
            scope.attributes.as_ptr(),
        )
    };
    let original_metric = original(&metric_scope);
    let original_log = original(&log_scope);
    let metric_count = allocations(|| {
        black_box(meter::Meter::new(
            &metric_provider,
            InstrumentationOptions {
                scope: metric_scope,
            },
        ));
    });
    let log_count = allocations(|| {
        black_box(logger::Logger::new(
            &log_provider,
            InstrumentationOptions { scope: log_scope },
        ));
    });
    assert_eq!(metric_count, 0);
    assert_eq!(log_count, 0);
    for (stored, original) in [
        (metric_provider.scope.lock().unwrap(), original_metric),
        (log_provider.0.scope.lock().unwrap(), original_log),
    ] {
        let scope = stored.as_ref().unwrap();
        assert_eq!(scope.name(), "allocation-test");
        assert_eq!(scope.version(), Some("1.2.3"));
        assert_eq!(scope.schema_url(), Some(generated_semconv::SCHEMA_URL));
        assert_eq!(scope.name().as_ptr(), original.0);
        assert_eq!(scope.version().unwrap().as_ptr(), original.1);
        assert_eq!(
            scope.attributes().next().unwrap() as *const KeyValue,
            original.2
        );
        assert_eq!(
            scope.attributes().collect::<Vec<_>>(),
            [&KeyValue::new("scope.identity", 7_i64)]
        );
    }
    println!("generated configured scope allocations: meter={metric_count}, logger={log_count}");
}

#[test]
fn scope_convenience_inputs_are_nested_in_instrumentation_options() {
    let metric_provider = BorrowedMeterProvider {
        meter: Meter::new(Arc::new(MetricProvider(Arc::new(MetricState::default())))),
        scope: Mutex::new(None),
    };
    for options in [
        scope::Scope::from("static"),
        scope::Scope::from(String::from("owned")),
        scope::Scope::from(Cow::Borrowed("cow-borrowed")),
        scope::Scope::from(Cow::Owned(String::from("cow-owned"))),
    ]
    .into_iter()
    .map(|scope| InstrumentationOptions { scope })
    .chain(std::iter::once(InstrumentationOptions::default()))
    {
        let name = options.scope.name.clone();
        let _meter = meter::Meter::new(&metric_provider, options);
        let stored = metric_provider.scope.lock().unwrap();
        let stored = stored.as_ref().unwrap();
        assert_eq!(stored.name(), name);
        assert_eq!(stored.version(), None);
        assert_eq!(stored.attributes().count(), 0);
        assert_eq!(stored.schema_url(), Some(generated_semconv::SCHEMA_URL));
    }
}

#[test]
fn enabled_events_use_no_generated_heap_and_preserve_duplicate_attributes() {
    let state = Arc::new(LogState::default());
    state.enabled.store(true, Ordering::Relaxed);
    let logger = logger::Logger::new(
        &LogProvider(state.clone()),
        InstrumentationOptions {
            scope: "allocation-test".into(),
        },
    );
    let context = Context::new();
    probeevent::emit_recorded(&context, &logger, probeattr::RequiredAttr::from(3), []);
    let count = allocations(|| {
        probeevent::emit_recorded(
            &context,
            &logger,
            probeattr::RequiredAttr::from(3),
            [probeevent::RecordedOption::Optional(
                probeattr::OptionalAttr::from(5),
            )],
        )
    });
    assert_eq!(count, 0, "ordinary enabled event attributes allocated");
    assert_eq!(state.attribute_count.load(Ordering::Relaxed), 2);
    assert_eq!(state.attribute_sum.load(Ordering::Relaxed), 8);
    probeevent::emit_recorded(
        &context,
        &logger,
        probeattr::RequiredAttr::from(3),
        (0..20).map(|value| {
            probeevent::RecordedOption::Optional(probeattr::OptionalAttr::from(value))
        }),
    );
    assert_eq!(state.attribute_count.load(Ordering::Relaxed), 21);
    assert_eq!(state.attribute_sum.load(Ordering::Relaxed), 193);
    assert_eq!(
        state.order.load(Ordering::Relaxed),
        (0..20_u64).fold(3_u64, |hash, value| hash
            .wrapping_mul(31)
            .wrapping_add(value))
    );
    let configuration_count = allocations(|| {
        probeevent::emit_recorded(
            &context,
            &logger,
            probeattr::RequiredAttr::from(3),
            [
                probeevent::RecordedOption::Severity(Severity::Warn),
                probeevent::RecordedOption::Timestamp(UNIX_EPOCH + Duration::from_secs(10)),
                probeevent::RecordedOption::Severity(Severity::Error),
                probeevent::RecordedOption::Timestamp(UNIX_EPOCH + Duration::from_secs(20)),
            ],
        )
    });
    assert_eq!(configuration_count, 0);
    assert_eq!(
        state.severity.load(Ordering::Relaxed),
        Severity::Error as usize
    );
    assert_eq!(state.timestamp.load(Ordering::Relaxed), 20);
    println!("generated enabled event allocations: attributes={count}, configuration={configuration_count}");
}

#[test]
fn disabled_events_defer_context_attachment_and_attribute_conversion() {
    let state = Arc::new(LogState::default());
    let logger = logger::Logger::new(
        &LogProvider(state.clone()),
        InstrumentationOptions {
            scope: "allocation-test".into(),
        },
    );
    let context = Context::new().with_value(ExplicitContext);
    let values = probeattr::ValuesAttr::from(vec![1, 2, 3]);
    probeevent::emit_recorded(
        &Context::new(),
        &logger,
        probeattr::RequiredAttr::from(0),
        [],
    );
    let count = allocations(|| {
        probeevent::emit_recorded(
            &context,
            &logger,
            probeattr::RequiredAttr::from(3),
            [probeevent::RecordedOption::Values(values)],
        )
    });
    assert_eq!(count, 0, "disabled event attributes allocated");
    assert!(!state.context_attached_during_check.load(Ordering::Relaxed));
    assert_eq!(state.attribute_count.load(Ordering::Relaxed), 0);
    println!("generated disabled event allocations: {count}");
}
