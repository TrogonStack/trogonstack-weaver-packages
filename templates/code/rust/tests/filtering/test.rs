use generated_semconv::taskattr::{IdAttr, StateAttr};
use opentelemetry::{KeyValue, Value};

#[test]
fn vendor_prefix_changes_types_but_preserves_telemetry_keys() {
    assert_eq!(
        ::opentelemetry::KeyValue::from(IdAttr::from("task")),
        KeyValue::new("myapp.task.id", "task")
    );
    assert_eq!(
        ::opentelemetry::KeyValue::from(StateAttr::Queued).value,
        Value::String("queued".into())
    );
}
