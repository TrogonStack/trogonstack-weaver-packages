use generated_semconv::options::InstrumentationOptions;
use generated_semconv::{
    logger,
    attribute,
    event,
    scope,
    SCHEMA_URL,
};
use opentelemetry::logs::{AnyValue, Severity};
use opentelemetry::trace::{SpanContext, SpanId, TraceContextExt, TraceFlags, TraceId, TraceState};
use opentelemetry_sdk::logs::{InMemoryLogExporter, SdkLoggerProvider};

#[test]
fn typed_events_have_schema_timestamps_severity_attributes_and_trace_context() {
    let exporter = InMemoryLogExporter::default();
    let provider = SdkLoggerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let logger = logger::Logger::new(
        &provider,
        InstrumentationOptions {
            scope: scope::Scope {
                attributes: vec![opentelemetry::KeyValue::new("scope.kind", "test")],
            },
        },
    );
    let span = SpanContext::new(
        TraceId::from(7),
        SpanId::from(9),
        TraceFlags::SAMPLED,
        false,
        TraceState::default(),
    );
    let context = opentelemetry::Context::new().with_remote_span_context(span.clone());
    let timestamp = std::time::UNIX_EPOCH + std::time::Duration::from_secs(100);
    event::emit_myapp_task_finished(
        &context,
        &logger,
        attribute::MyappTaskState::Done,
        [
            event::MyappTaskFinishedOption::TaskAttempt(attribute::MyappTaskAttempt::from(2)),
            event::MyappTaskFinishedOption::HostName(attribute::WorkerHostName::from("worker-1")),
            event::MyappTaskFinishedOption::Severity(Severity::Warn),
            event::MyappTaskFinishedOption::Timestamp(timestamp),
            event::MyappTaskFinishedOption::BoolValues(attribute::MyappBoolValues::from(vec![true])),
            event::MyappTaskFinishedOption::IntValues(attribute::MyappIntValues::from(vec![1])),
            event::MyappTaskFinishedOption::DoubleValues(attribute::MyappDoubleValues::from(vec![
                1.5,
            ])),
            event::MyappTaskFinishedOption::StringValues(attribute::MyappStringValues::from(vec![
                opentelemetry::StringValue::from("fetch"),
            ])),
        ],
    );
    event::emit_worker_task_finished(
        &context,
        &logger,
        attribute::MyappTaskState::Done,
        attribute::WorkerHostName::from("worker-1"),
        [],
    );
    provider.force_flush().unwrap();
    let logs = exporter.get_emitted_logs().unwrap();
    assert_eq!(logs.len(), 2);
    let log = &logs[0];
    assert_eq!(log.record.event_name(), Some("myapp.task.finished"));
    assert_eq!(log.record.severity_number(), Some(Severity::Warn));
    assert_eq!(log.record.timestamp(), Some(timestamp));
    assert!(log.record.observed_timestamp().is_some());
    assert!(log.record.body().is_none());
    assert_eq!(log.instrumentation.schema_url(), Some(SCHEMA_URL));
    assert_eq!(log.instrumentation.name(), "generated-semconv");
    assert_eq!(log.instrumentation.version(), None);
    assert!(log
        .instrumentation
        .attributes()
        .any(|kv| kv == &opentelemetry::KeyValue::new("scope.kind", "test")));
    assert_eq!(
        log.record.trace_context().unwrap().trace_id,
        span.trace_id()
    );
    assert_eq!(log.record.trace_context().unwrap().span_id, span.span_id());
    assert!(log
        .record
        .attributes_iter()
        .any(|(key, value)| key.as_str() == "myapp.task.attempt" && value == &AnyValue::Int(2)));
    for (key, expected) in [
        ("myapp.bool.values", AnyValue::Boolean(true)),
        ("myapp.int.values", AnyValue::Int(1)),
        ("myapp.double.values", AnyValue::Double(1.5)),
        ("myapp.string.values", AnyValue::String("fetch".into())),
    ] {
        assert!(log
            .record
            .attributes_iter()
            .any(|(attr_key, value)| attr_key.as_str() == key
                && value == &AnyValue::ListAny(Box::new(vec![expected.clone()]))));
    }
    assert_eq!(logs[1].record.event_name(), Some("worker.task.finished"));
    assert_eq!(logs[1].record.severity_number(), Some(Severity::Info));
    assert!(logs[1].record.timestamp().is_none());
    event::emit_myapp_task_finished(
        &context,
        &logger::Logger::default(),
        attribute::MyappTaskState::Done,
        [],
    );
}

struct DisabledProvider;
struct DisabledLogger;
impl opentelemetry::logs::LoggerProvider for DisabledProvider {
    type Logger = DisabledLogger;
    fn logger_with_scope(&self, _: opentelemetry::InstrumentationScope) -> DisabledLogger {
        DisabledLogger
    }
}
impl opentelemetry::logs::Logger for DisabledLogger {
    type LogRecord = opentelemetry_sdk::logs::SdkLogRecord;
    fn create_log_record(&self) -> Self::LogRecord {
        panic!("disabled events must not create a record")
    }
    fn emit(&self, _: Self::LogRecord) {
        panic!("disabled events must not emit a record")
    }
    fn event_enabled(&self, severity: Severity, target: &str, name: Option<&str>) -> bool {
        assert_eq!(severity, Severity::Error);
        assert_eq!(target, "");
        assert_eq!(name, Some("myapp.task.finished"));
        false
    }
}
#[test]
fn disabled_events_are_checked_before_record_creation() {
    event::emit_myapp_task_finished(
        &opentelemetry::Context::new(),
        &logger::Logger::new(&DisabledProvider, InstrumentationOptions::default()),
        attribute::MyappTaskState::Done,
        [event::MyappTaskFinishedOption::Severity(Severity::Error)],
    );
}

const _: &str = event::MyappTaskFinished::NAME;

#[test]
fn events_expose_their_name_as_a_constant() {
    assert_eq!(event::MyappTaskFinished::NAME, "myapp.task.finished");
}
