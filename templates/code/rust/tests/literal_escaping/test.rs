use generated_semconv::demoattr::LiteralAttr;
use generated_semconv::options::InstrumentationOptions;
use generated_semconv::{demometric::LiteralCountCounter, meter::Meter};
use opentelemetry::Value;
use opentelemetry_sdk::metrics::{InMemoryMetricExporter, PeriodicReader, SdkMeterProvider};
#[test]
fn literal_backslashes_and_actual_control_characters_are_distinct() {
    assert_eq!(
        ::opentelemetry::KeyValue::from(LiteralAttr::Literal).value,
        Value::String(r"backspace\b formfeed\f unicode\u1234 doubled\\b tripled\\\u0000".into())
    );
    assert_eq!(
        ::opentelemetry::KeyValue::from(LiteralAttr::Controls).value,
        Value::String("backspace\u{0008} formfeed\u{000c} unicode\u{0001} slash\\b".into())
    );
}

#[test]
fn signal_metadata_preserves_literal_backslash_sequences() {
    let exporter = InMemoryMetricExporter::default();
    let provider = SdkMeterProvider::builder()
        .with_reader(PeriodicReader::builder(exporter.clone()).build())
        .build();
    LiteralCountCounter::new(&Meter::new(&provider, InstrumentationOptions::default()))
        .add(1, LiteralAttr::Literal);
    provider.force_flush().unwrap();
    let exported = exporter.get_finished_metrics().unwrap();
    let scope = exported[0].scope_metrics().next().unwrap();
    assert_eq!(
        scope.scope().name(),
        r#"literal \b \f \u1234 quote" double\\slash"#
    );
    assert_eq!(
        scope.scope().version(),
        Some("controls \u{0008} \u{000c} \u{0001} literal\\b quote\"")
    );
    assert_eq!(
        scope.scope().schema_url(),
        Some(generated_semconv::SCHEMA_URL)
    );
    let metric = scope.metrics().next().unwrap();
    assert_eq!(metric.description(), r"literal \b \f \u1234");
    assert_eq!(metric.unit(), r"literal\b\u1234");
}
