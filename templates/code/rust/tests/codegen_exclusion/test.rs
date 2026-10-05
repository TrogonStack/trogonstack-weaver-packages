use generated_semconv::demoattr::{KeptAttr, ModeAttr};
use opentelemetry::KeyValue;
#[test]
fn explicit_false_keeps_attributes_and_enum_members() {
    assert_eq!(
        KeptAttr::from("value").key_value(),
        KeyValue::new("demo.kept", "value")
    );
    assert_eq!(
        ModeAttr::Kept.key_value(),
        KeyValue::new("demo.mode", "kept")
    );
}
