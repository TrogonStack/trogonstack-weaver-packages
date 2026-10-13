use generated_semconv::attribute::{DemoKept, DemoMode};
use opentelemetry::KeyValue;
#[test]
fn explicit_false_keeps_attributes_and_enum_members() {
    assert_eq!(
        KeyValue::from(DemoKept::from("value")),
        KeyValue::new("demo.kept", "value")
    );
    assert_eq!(
        KeyValue::from(DemoMode::Kept),
        KeyValue::new("demo.mode", "kept")
    );
}
