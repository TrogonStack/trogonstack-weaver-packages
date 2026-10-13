use generated_semconv::attribute::{TaskId, TaskState};
use opentelemetry::{KeyValue, Value};

#[test]
fn vendor_prefix_changes_types_but_preserves_telemetry_keys() {
    assert_eq!(
        ::opentelemetry::KeyValue::from(TaskId::from("task")),
        KeyValue::new("myapp.task.id", "task")
    );
    assert_eq!(
        ::opentelemetry::KeyValue::from(TaskState::Queued).value,
        Value::String("queued".into())
    );
}
