use generated_semconv::{authattr, authmetric, meter, myappattr, myappmetric, workermetric};
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
    let meter = meter::Meter::new_with_scope(
        &provider,
        opentelemetry::InstrumentationScope::builder("synthetic-worker")
            .with_version("1.2.3")
            .with_schema_url("https://example.com/other/1.0.0")
            .with_attributes([opentelemetry::KeyValue::new("scope.kind", "worker")])
            .build(),
    );
    myappmetric::TaskDurationHistogram::new(&meter).record(
        0.1,
        myappattr::TaskIdAttr::new("base"),
        [
            myappmetric::TaskDurationHistogramAttr::TaskState(myappattr::TaskStateAttr::Queued),
            myappmetric::TaskDurationHistogramAttr::Method(authattr::MethodAttr::Password),
        ],
    );
    myappmetric::TaskRetryDurationHistogram::new(&meter).record(
        0.2,
        myappattr::TaskIdAttr::new("retry"),
        myappattr::TaskStateAttr::Done,
        [myappmetric::TaskRetryDurationHistogramAttr::Method(
            authattr::MethodAttr::Token,
        )],
    );
    workermetric::TaskDurationHistogram::new(&meter).record(
        0.3,
        myappattr::TaskIdAttr::new("worker"),
        [],
    );
    myappmetric::TaskStartedCounter::new(&meter).add(
        7,
        [myappmetric::TaskStartedCounterAttr::TaskId(
            myappattr::TaskIdAttr::new("started"),
        )],
    );
    myappmetric::TaskActiveUpDownCounter::new(&meter).add(-2, myappattr::TaskStateAttr::Queued);
    myappmetric::QueueDepthGauge::new(&meter).record(5);
    authmetric::AttemptsCounter::new(&meter).add(
        3,
        authattr::MethodAttr::Password,
        authattr::SuccessAttr::new(true),
    );

    provider.force_flush().unwrap();
    let exported = exporter.get_finished_metrics().unwrap();
    let scope = exported[0].scope_metrics().next().unwrap();
    assert_eq!(scope.scope().version(), Some("1.2.3"));
    assert!(scope
        .scope()
        .attributes()
        .any(|attribute| attribute.key.as_str() == "scope.kind"
            && attribute.value.as_str() == "worker"));
    assert_eq!(scope.scope().name(), "synthetic-worker");
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
    let meter = meter::Meter::new(&provider, "synthetic-observer");
    let _started = myappmetric::TaskStartedObservableCounter::new(&meter, |observer| {
        observer.observe(
            11,
            [myappmetric::TaskStartedCounterAttr::TaskId(
                myappattr::TaskIdAttr::new("observed"),
            )],
        );
    });
    let _active = myappmetric::TaskActiveObservableUpDownCounter::new(&meter, |observer| {
        observer.observe(-4, myappattr::TaskStateAttr::Queued);
    });
    let _depth = myappmetric::QueueDepthObservableGauge::new(&meter, |observer| {
        observer.observe(9);
    });
    let _attempts = authmetric::AttemptsObservableCounter::new(&meter, |observer| {
        observer.observe(
            13,
            authattr::MethodAttr::Token,
            authattr::SuccessAttr::new(false),
        );
    });
    provider.force_flush().unwrap();
    let exported = exporter.get_finished_metrics().unwrap();
    let scope = exported[0].scope_metrics().next().unwrap();
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
    let sync_meter = meter::Meter::new(&provider, "synthetic-float-sync");
    myappmetric::TaskFloatStartedCounter::new(&sync_meter).add(0.5);
    myappmetric::TaskFloatActiveUpDownCounter::new(&sync_meter).add(-1.25);
    myappmetric::TaskFloatDepthGauge::new(&sync_meter).record(2.5);
    myappmetric::TaskPayloadHistogram::new(&sync_meter).record(512);
    let async_meter = meter::Meter::new(&provider, "synthetic-float-async");
    let _started = myappmetric::TaskFloatStartedObservableCounter::new(&async_meter, |observer| {
        observer.observe(0.75);
    });
    let _active =
        myappmetric::TaskFloatActiveObservableUpDownCounter::new(&async_meter, |observer| {
            observer.observe(-2.5);
        });
    let _depth = myappmetric::TaskFloatDepthObservableGauge::new(&async_meter, |observer| {
        observer.observe(3.75);
    });
    provider.force_flush().unwrap();
    let exported = exporter.get_finished_metrics().unwrap();
    for (name, started_value, active_value, depth_value) in [
        ("synthetic-float-sync", 0.5, -1.25, 2.5),
        ("synthetic-float-async", 0.75, -2.5, 3.75),
    ] {
        let scope = exported[0]
            .scope_metrics()
            .find(|scope| scope.scope().name() == name)
            .unwrap();
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
        if name == "synthetic-float-sync" {
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
