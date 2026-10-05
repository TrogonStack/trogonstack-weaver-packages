use generated_semconv::taskattr::{IdAttr, StateAttr};
use opentelemetry::{KeyValue, Value};

#[test]
fn vendor_prefix_changes_types_but_preserves_telemetry_keys() {
    assert_eq!(
        IdAttr::new("task").key_value(),
        KeyValue::new("myapp.task.id", "task")
    );
    assert_eq!(
        StateAttr::Queued.key_value().value,
        Value::String("queued".into())
    );
}
