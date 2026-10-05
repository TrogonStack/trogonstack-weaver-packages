use generated_semconv::options::InstrumentationOptions;
use generated_semconv::{logger, probeevent};
use opentelemetry::logs::Severity;
use opentelemetry_sdk::logs::{InMemoryLogExporter, SdkLoggerProvider};

#[test]
fn events_without_attributes_export_cleanly() {
    let exporter = InMemoryLogExporter::default();
    let provider = SdkLoggerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let logger = logger::Logger::new(&provider, InstrumentationOptions::default());
    probeevent::emit_tick(
        &opentelemetry::Context::new(),
        &logger,
        [probeevent::TickOption::Severity(Severity::Debug)],
    );
    provider.force_flush().unwrap();
    let records = exporter.get_emitted_logs().unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].instrumentation.name(), "generated-semconv");
    assert_eq!(records[0].instrumentation.version(), None);
    assert_eq!(records[0].record.event_name(), Some("probe.tick"));
    assert_eq!(records[0].record.severity_number(), Some(Severity::Debug));
    assert_eq!(records[0].record.attributes_iter().count(), 0);
    assert_eq!(
        records[0].instrumentation.schema_url(),
        Some(generated_semconv::SCHEMA_URL)
    );
}
