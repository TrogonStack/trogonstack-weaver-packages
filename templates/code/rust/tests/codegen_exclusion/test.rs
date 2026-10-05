use generated_semconv::demoattr::{KeptAttr, ModeAttr};
use opentelemetry::KeyValue;
#[test]
fn explicit_false_keeps_attributes_and_enum_members() {
    assert_eq!(
        KeyValue::from(KeptAttr::from("value")),
        KeyValue::new("demo.kept", "value")
    );
    assert_eq!(
        KeyValue::from(ModeAttr::Kept),
        KeyValue::new("demo.mode", "kept")
    );
}
