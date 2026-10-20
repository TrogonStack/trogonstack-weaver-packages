use generated_semconv::options::InstrumentationOptions;
use generated_semconv::{attribute, meter, metric, scope};
use opentelemetry::KeyValue;
use opentelemetry_sdk::metrics::data::{AggregatedMetrics, MetricData};
use opentelemetry_sdk::metrics::{InMemoryMetricExporter, PeriodicReader, SdkMeterProvider};
use std::time::Duration;

fn provider() -> (SdkMeterProvider, InMemoryMetricExporter) {
    let exporter = InMemoryMetricExporter::default();
    let provider = SdkMeterProvider::builder()
        .with_reader(
            PeriodicReader::builder(exporter.clone())
                .with_interval(Duration::from_secs(3600))
                .build(),
        )
        .build();
    (provider, exporter)
}

#[test]
#[allow(deprecated)]
fn synchronous_metrics_export_types_values_attributes_and_refinements() {
    let (provider, exporter) = provider();
    let meter = meter::Meter::new(
        &provider,
        InstrumentationOptions {
            scope: scope::Scope {
                attributes: vec![KeyValue::new("scope.kind", "worker")],
            },
        },
    );
    metric::MyappTaskDuration::new(&meter).record(
        0.1,
        attribute::MyappTaskId::from("base"),
        [
            metric::MyappTaskDurationAttr::TaskState(attribute::MyappTaskState::Queued),
            metric::MyappTaskDurationAttr::Method(attribute::AuthMethod::Password),
        ],
    );
    metric::MyappTaskRetryDuration::new(&meter).record(
        0.2,
        attribute::MyappTaskId::from("retry"),
        attribute::MyappTaskState::Done,
        [metric::MyappTaskRetryDurationAttr::Method(
            attribute::AuthMethod::Token,
        )],
    );
    metric::WorkerTaskDuration::new(&meter).record(
        0.3,
        attribute::MyappTaskId::from("worker"),
        [],
    );
    metric::MyappTaskStarted::new(&meter).add(
        7,
        [metric::MyappTaskStartedAttr::TaskId(
            attribute::MyappTaskId::from("started"),
        )],
    );
    metric::MyappTaskActive::new(&meter).add(-2, attribute::MyappTaskState::Queued);
    metric::MyappQueueDepth::new(&meter).record(5);
    metric::AuthAttempts::new(&meter).add(
        3,
        attribute::AuthMethod::Password,
        attribute::AuthSuccess::from(true),
    );

    provider.force_flush().unwrap();
    let exported = exporter.get_finished_metrics().unwrap();
    let scope = exported[0].scope_metrics().next().unwrap();
    assert_eq!(scope.scope().version(), None);
    assert!(scope
        .scope()
        .attributes()
        .any(|attribute| attribute.key.as_str() == "scope.kind"
            && attribute.value.as_str() == "worker"));
    assert_eq!(scope.scope().name(), "generated-semconv");
    assert_eq!(
        scope.scope().schema_url(),
        Some(generated_semconv::SCHEMA_URL)
    );
    let find = |name: &str| {
        scope
            .metrics()
            .find(|metric| metric.name() == name)
            .unwrap()
    };

    let duration = find("myapp.task.duration");
    assert_eq!(
        duration.description(),
        "Time a task took from start to finish."
    );
    assert_eq!(duration.unit(), "s");
    let AggregatedMetrics::F64(MetricData::Histogram(histogram)) = duration.data() else {
        panic!("duration must be an f64 histogram");
    };
    assert_eq!(histogram.data_points().count(), 3);
    for (id, value, expected_attrs) in [
        (
            "base",
            0.1,
            vec![
                KeyValue::new("myapp.task.id", "base"),
                KeyValue::new("myapp.task.state", "queued"),
                KeyValue::new("auth.method", "password"),
            ],
        ),
        (
            "retry",
            0.2,
            vec![
                KeyValue::new("myapp.task.id", "retry"),
                KeyValue::new("myapp.task.state", "done"),
                KeyValue::new("auth.method", "token"),
            ],
        ),
        (
            "worker",
            0.3,
            vec![KeyValue::new("myapp.task.id", "worker")],
        ),
    ] {
        let point = histogram
            .data_points()
            .find(|point| {
                point
                    .attributes()
                    .any(|attr| *attr == KeyValue::new("myapp.task.id", id))
            })
            .unwrap();
        assert_eq!(point.count(), 1);
        assert_eq!(point.sum(), value);
        assert_eq!(
            point.bounds().collect::<Vec<_>>(),
            [0.0, 0.005, 0.01, 0.1, 1.0, 10.0]
        );
        assert_eq!(point.attributes().count(), expected_attrs.len());
        for expected in expected_attrs {
            assert!(point.attributes().any(|actual| *actual == expected));
        }
    }

    let started = find("myapp.task.started");
    assert_eq!(started.unit(), "{task}");
    let AggregatedMetrics::U64(MetricData::Sum(sum)) = started.data() else {
        panic!("counter must be an unsigned sum");
    };
    let point = sum.data_points().next().unwrap();
    assert_eq!(point.value(), 7);
    assert_eq!(
        point.attributes().collect::<Vec<_>>(),
        [&KeyValue::new("myapp.task.id", "started")]
    );

    let AggregatedMetrics::I64(MetricData::Sum(sum)) = find("myapp.task.active").data() else {
        panic!("up-down counter must preserve signed values");
    };
    let point = sum.data_points().next().unwrap();
    assert_eq!(point.value(), -2);
    assert_eq!(
        point.attributes().collect::<Vec<_>>(),
        [&KeyValue::new("myapp.task.state", "queued")]
    );

    let AggregatedMetrics::I64(MetricData::Gauge(gauge)) = find("myapp.queue.depth").data() else {
        panic!("queue depth must be a signed gauge");
    };
    let point = gauge.data_points().next().unwrap();
    assert_eq!(point.value(), 5);
    assert_eq!(point.attributes().count(), 0);

    let attempts = find("auth.attempts");
    assert_eq!(attempts.unit(), "{attempt}");
    let AggregatedMetrics::U64(MetricData::Sum(sum)) = attempts.data() else {
        panic!("attempts must be an unsigned sum");
    };
    let point = sum.data_points().next().unwrap();
    assert_eq!(point.value(), 3);
    assert!(point
        .attributes()
        .any(|attr| *attr == KeyValue::new("auth.method", "password")));
    assert!(point
        .attributes()
        .any(|attr| *attr == KeyValue::new("auth.success", true)));
    provider.shutdown().unwrap();
}

#[test]
#[allow(deprecated)]
fn observable_callbacks_export_values_and_typed_attributes() {
    let (provider, exporter) = provider();
    let meter = meter::Meter::new(&provider, InstrumentationOptions::default());
    let _started = metric::MyappTaskStartedObservable::new(&meter, |observer| {
        observer.observe(
            11,
            [metric::MyappTaskStartedAttr::TaskId(
                attribute::MyappTaskId::from("observed"),
            )],
        );
    });
    let _active = metric::MyappTaskActiveObservable::new(&meter, |observer| {
        observer.observe(-4, attribute::MyappTaskState::Queued);
    });
    let _depth = metric::MyappQueueDepthObservable::new(&meter, |observer| {
        observer.observe(9);
    });
    let _attempts = metric::AuthAttemptsObservable::new(&meter, |observer| {
        observer.observe(
            13,
            attribute::AuthMethod::Token,
            attribute::AuthSuccess::from(false),
        );
    });
    provider.force_flush().unwrap();
    let exported = exporter.get_finished_metrics().unwrap();
    let scope = exported[0].scope_metrics().next().unwrap();
    assert_eq!(scope.scope().name(), "generated-semconv");
    assert_eq!(scope.scope().version(), None);
    assert_eq!(
        scope.scope().schema_url(),
        Some(generated_semconv::SCHEMA_URL)
    );
    let find = |name: &str| {
        scope
            .metrics()
            .find(|metric| metric.name() == name)
            .unwrap()
    };
    let AggregatedMetrics::U64(MetricData::Sum(started)) = find("myapp.task.started").data() else {
        panic!("observable counter must export an unsigned sum");
    };
    let point = started.data_points().next().unwrap();
    assert_eq!(point.value(), 11);
    assert!(point
        .attributes()
        .any(|attr| *attr == KeyValue::new("myapp.task.id", "observed")));
    let AggregatedMetrics::I64(MetricData::Sum(active)) = find("myapp.task.active").data() else {
        panic!("observable up-down counter must preserve signed values");
    };
    assert_eq!(active.data_points().next().unwrap().value(), -4);
    let AggregatedMetrics::I64(MetricData::Gauge(depth)) = find("myapp.queue.depth").data() else {
        panic!("observable gauge must export a gauge");
    };
    assert_eq!(depth.data_points().next().unwrap().value(), 9);
    let AggregatedMetrics::U64(MetricData::Sum(attempts)) = find("auth.attempts").data() else {
        panic!("observable counter must export an unsigned sum");
    };
    let point = attempts.data_points().next().unwrap();
    assert_eq!(point.value(), 13);
    assert!(point
        .attributes()
        .any(|attr| *attr == KeyValue::new("auth.method", "token")));
    assert!(point
        .attributes()
        .any(|attr| *attr == KeyValue::new("auth.success", false)));
    provider.shutdown().unwrap();
}

#[test]
fn alternate_numeric_types_export_sync_and_observable_measurements() {
    let (provider, exporter) = provider();
    let sync_meter = meter::Meter::new(
        &provider,
        InstrumentationOptions {
            scope: scope::Scope {
                attributes: vec![KeyValue::new("measurement.mode", "sync")],
            },
        },
    );
    metric::MyappTaskFloatStarted::new(&sync_meter).add(0.5);
    metric::MyappTaskFloatActive::new(&sync_meter).add(-1.25);
    metric::MyappTaskFloatDepth::new(&sync_meter).record(2.5);
    metric::MyappTaskPayload::new(&sync_meter).record(512);
    let async_meter = meter::Meter::new(
        &provider,
        InstrumentationOptions {
            scope: scope::Scope {
                attributes: vec![KeyValue::new("measurement.mode", "async")],
            },
        },
    );
    let _started = metric::MyappTaskFloatStartedObservable::new(&async_meter, |observer| {
        observer.observe(0.75);
    });
    let _active =
        metric::MyappTaskFloatActiveObservable::new(&async_meter, |observer| {
            observer.observe(-2.5);
        });
    let _depth = metric::MyappTaskFloatDepthObservable::new(&async_meter, |observer| {
        observer.observe(3.75);
    });
    provider.force_flush().unwrap();
    let exported = exporter.get_finished_metrics().unwrap();
    for (mode, started_value, active_value, depth_value) in
        [("sync", 0.5, -1.25, 2.5), ("async", 0.75, -2.5, 3.75)]
    {
        let scope = exported[0]
            .scope_metrics()
            .find(|scope| {
                scope
                    .scope()
                    .attributes()
                    .any(|attribute| attribute == &KeyValue::new("measurement.mode", mode))
            })
            .unwrap();
        assert_eq!(scope.scope().name(), "generated-semconv");
        assert_eq!(scope.scope().version(), None);
        assert_eq!(
            scope.scope().schema_url(),
            Some(generated_semconv::SCHEMA_URL)
        );
        let find = |metric_name: &str| {
            scope
                .metrics()
                .find(|metric| metric.name() == metric_name)
                .unwrap()
        };
        let AggregatedMetrics::F64(MetricData::Sum(started)) =
            find("myapp.task.float.started").data()
        else {
            panic!("double counter must export a floating sum");
        };
        assert_eq!(started.data_points().next().unwrap().value(), started_value);
        let AggregatedMetrics::F64(MetricData::Sum(active)) =
            find("myapp.task.float.active").data()
        else {
            panic!("double up-down counter must export a floating sum");
        };
        assert_eq!(active.data_points().next().unwrap().value(), active_value);
        let AggregatedMetrics::F64(MetricData::Gauge(depth)) =
            find("myapp.task.float.depth").data()
        else {
            panic!("double gauge must export a floating gauge");
        };
        assert_eq!(depth.data_points().next().unwrap().value(), depth_value);
        if mode == "sync" {
            let payload = find("myapp.task.payload");
            assert_eq!(payload.unit(), "By");
            let AggregatedMetrics::U64(MetricData::Histogram(histogram)) = payload.data() else {
                panic!("integer histogram must preserve unsigned measurements");
            };
            let point = histogram.data_points().next().unwrap();
            assert_eq!(point.count(), 1);
            assert_eq!(point.sum(), 512);
            assert_eq!(point.bounds().collect::<Vec<_>>(), [128.0, 256.0, 1024.0]);
        }
    }
    provider.shutdown().unwrap();
}

const _: &str = metric::MyappTaskDuration::NAME;

#[test]
fn instruments_expose_registry_name_as_constant() {
    assert_eq!(metric::MyappTaskDuration::NAME, "myapp.task.duration");
    assert_eq!(metric::AuthAttemptsObservable::NAME, metric::AuthAttempts::NAME);
}

#[test]
fn instruments_export_registry_unit_and_description() {
    let (provider, exporter) = provider();
    let meter = meter::Meter::new(&provider, InstrumentationOptions::default());
    metric::MyappTaskDuration::new(&meter).record(
        0.1,
        attribute::MyappTaskId::from("metadata"),
        [],
    );
    let _observable = metric::AuthAttemptsObservable::new(&meter, |observer| {
        observer.observe(
            1,
            attribute::AuthMethod::Password,
            attribute::AuthSuccess::from(true),
        );
    });

    provider.force_flush().unwrap();
    let exported = exporter.get_finished_metrics().unwrap();
    let scope = exported[0].scope_metrics().next().unwrap();
    let find = |metric_name: &str| {
        scope
            .metrics()
            .find(|metric| metric.name() == metric_name)
            .unwrap()
    };
    let duration = find(metric::MyappTaskDuration::NAME);
    assert_eq!(duration.unit(), "s");
    assert_eq!(
        duration.description(),
        "Time a task took from start to finish."
    );
    let observable = find(metric::AuthAttemptsObservable::NAME);
    assert_eq!(observable.unit(), "{attempt}");
    assert_eq!(
        observable.description(),
        "Number of authentication attempts."
    );
    provider.shutdown().unwrap();
}

#[test]
fn meter_accepts_the_global_provider_trait_object() {
    let (provider, exporter) = provider();
    opentelemetry::global::set_meter_provider(provider.clone());
    let meter = meter::Meter::new(
        &*opentelemetry::global::meter_provider(),
        InstrumentationOptions::default(),
    );
    metric::MyappTaskStarted::new(&meter).add(
        1,
        [metric::MyappTaskStartedAttr::TaskId(
            attribute::MyappTaskId::from("global"),
        )],
    );

    provider.force_flush().unwrap();
    let exported = exporter.get_finished_metrics().unwrap();
    let scope = exported[0].scope_metrics().next().unwrap();
    assert_eq!(scope.scope().name(), "generated-semconv");
    assert_eq!(
        scope.scope().schema_url(),
        Some(generated_semconv::SCHEMA_URL)
    );
    assert_eq!(
        scope.metrics().next().unwrap().name(),
        metric::MyappTaskStarted::NAME
    );
}
