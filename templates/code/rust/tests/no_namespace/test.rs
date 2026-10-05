use generated_semconv::{
    exceptionattr, exceptionevent, heartbeatattr, heartbeatspan, logger,
    options::InstrumentationOptions, tracer,
};
use opentelemetry::logs::{AnyValue, Severity};
use opentelemetry_sdk::logs::{InMemoryLogExporter, SdkLoggerProvider};
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};

#[test]
fn empty_short_names_expose_bare_functions_and_unambiguous_options() {
    let context = opentelemetry::Context::new();
    let logs = InMemoryLogExporter::default();
    let logger_provider = SdkLoggerProvider::builder()
        .with_simple_exporter(logs.clone())
        .build();
    let logger = logger::Logger::new(&logger_provider, InstrumentationOptions::default());
    exceptionevent::emit(
        &context,
        &logger,
        exceptionattr::TypeAttr::from("ValueError"),
        [
            exceptionevent::EventOption::Message(exceptionattr::MessageAttr::from("failed")),
            exceptionevent::EventOption::Severity(Severity::Warn),
        ],
    );
    logger_provider.force_flush().unwrap();
    let records = logs.get_emitted_logs().unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].record.event_name(), Some("exception"));
    assert_eq!(records[0].record.severity_number(), Some(Severity::Warn));
    assert!(records[0].record.attributes_iter().any(|(key, value)| {
        key.as_str() == "exception.message" && value == &AnyValue::String("failed".into())
    }));

    let spans = InMemorySpanExporter::default();
    let tracer_provider = SdkTracerProvider::builder()
        .with_simple_exporter(spans.clone())
        .build();
    let tracer = tracer::Tracer::new(&tracer_provider, InstrumentationOptions::default());
    let mut span = heartbeatspan::start(
        &context,
        &tracer,
        heartbeatspan::Name::default(),
        heartbeatattr::SequenceAttr::from(7),
        [heartbeatspan::StartAttr::Source(
            heartbeatattr::SourceAttr::from("worker"),
        )],
    );
    span.end();
    tracer_provider.force_flush().unwrap();
    let spans = spans.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, "heartbeat");
    assert!(spans[0]
        .attributes
        .contains(&opentelemetry::KeyValue::new("heartbeat.sequence", 7)));
    assert!(spans[0]
        .attributes
        .contains(&opentelemetry::KeyValue::new("heartbeat.source", "worker")));
}
