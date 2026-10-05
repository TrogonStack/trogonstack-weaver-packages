use generated_semconv::{
    billingattr, billingevent, billingmetric, billingspan, logger, meter,
    options::InstrumentationOptions, scope::Scope, tracer, SCHEMA_URL,
};
use opentelemetry::{InstrumentationScope, KeyValue};
use opentelemetry_sdk::logs::{InMemoryLogExporter, SdkLoggerProvider};
use opentelemetry_sdk::metrics::{InMemoryMetricExporter, PeriodicReader, SdkMeterProvider};
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};
use std::time::Duration;

fn options() -> InstrumentationOptions {
    InstrumentationOptions {
        scope: Scope {
            attributes: vec![KeyValue::new("scope.kind", "configured")],
        },
    }
}

fn configured_identity(scope: &InstrumentationScope) {
    assert_eq!(scope.name(), "synthetic-billing");
    assert_eq!(scope.version(), Some("2.3.4"));
    assert_eq!(scope.schema_url(), Some(SCHEMA_URL));
    assert_eq!(
        scope.attributes().collect::<Vec<_>>(),
        [&KeyValue::new("scope.kind", "configured")]
    );
}

#[test]
fn configured_identity_is_shared_by_metrics_spans_and_events() {
    let metric_exporter = InMemoryMetricExporter::default();
    let metric_provider = SdkMeterProvider::builder()
        .with_reader(
            PeriodicReader::builder(metric_exporter.clone())
                .with_interval(Duration::from_secs(3600))
                .build(),
        )
        .build();
    billingmetric::InvoiceAmountHistogram::new(&meter::Meter::new(&metric_provider, options()))
        .record(12.5, billingattr::InvoiceIdAttr::from("invoice-1"), []);
    metric_provider.force_flush().unwrap();
    let metrics = metric_exporter.get_finished_metrics().unwrap();
    configured_identity(metrics[0].scope_metrics().next().unwrap().scope());

    let span_exporter = InMemorySpanExporter::default();
    let span_provider = SdkTracerProvider::builder()
        .with_simple_exporter(span_exporter.clone())
        .build();
    let tracer = tracer::Tracer::new(&span_provider, options());
    let context = opentelemetry::Context::new();
    let mut span = billingspan::start_invoice_issue(
        &context,
        &tracer,
        billingattr::InvoiceIdAttr::from("invoice-1"),
    );
    span.end();
    span_provider.force_flush().unwrap();
    let spans = span_exporter.get_finished_spans().unwrap();
    assert_eq!(spans[0].name, "invoice invoice-1");
    configured_identity(&spans[0].instrumentation_scope);

    let log_exporter = InMemoryLogExporter::default();
    let log_provider = SdkLoggerProvider::builder()
        .with_simple_exporter(log_exporter.clone())
        .build();
    billingevent::emit_invoice_issued(
        &context,
        &logger::Logger::new(&log_provider, options()),
        billingattr::InvoiceIdAttr::from("invoice-1"),
        [],
    );
    log_provider.force_flush().unwrap();
    let logs = log_exporter.get_emitted_logs().unwrap();
    assert_eq!(
        logs[0].record.event_name(),
        Some("acme.billing.invoice.issued")
    );
    configured_identity(&logs[0].instrumentation);
}
