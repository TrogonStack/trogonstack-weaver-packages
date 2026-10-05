use generated_semconv::{authattr, myappattr, SCHEMA_URL};
use opentelemetry::{Array, KeyValue, Value};

#[test]
fn typed_attributes_preserve_keys_values_and_schema() {
    assert_eq!(SCHEMA_URL, "https://example.com/schemas/attributes/1.0.0");
    let value: KeyValue = myappattr::TaskIdAttr::new("task").into();
    assert_eq!(value, KeyValue::new("myapp.task.id", "task"));
    assert_eq!(
        myappattr::TaskRetriesAttr::new(4).key_value(),
        KeyValue::new("myapp.task.retries", 4_i64)
    );
    assert_eq!(
        myappattr::TaskCancelledAttr::new(true).key_value().value,
        Value::Bool(true)
    );
    assert_eq!(
        myappattr::TaskProgressAttr::new(0.5).key_value().value,
        Value::F64(0.5)
    );
    assert_eq!(
        myappattr::TaskTagsAttr::new(vec!["tag".into()])
            .key_value()
            .value,
        Value::Array(Array::String(vec!["tag".into()]))
    );
    assert_eq!(
        myappattr::TaskShardIdsAttr::new(vec![1, 2])
            .key_value()
            .value,
        Value::Array(Array::I64(vec![1, 2]))
    );
    assert_eq!(
        myappattr::TaskWeightsAttr::new(vec![0.25])
            .key_value()
            .value,
        Value::Array(Array::F64(vec![0.25]))
    );
    assert_eq!(
        myappattr::TaskFlagsAttr::new(vec![false]).key_value().value,
        Value::Array(Array::Bool(vec![false]))
    );
    assert_eq!(
        myappattr::TaskStateAttr::Running.key_value().value,
        Value::String("running".into())
    );
    assert_eq!(myappattr::TaskEscapedAttr::Quoted.key_value().value, Value::String("quote\" slash\\ newline\n tab\t backspace\u{0008} formfeed\u{000c} null\u{0000} snowman☃".into()));
    assert_eq!(authattr::LevelAttr::High.key_value().value, Value::I64(2));
    assert_eq!(
        myappattr::TaskPreemptibleAttr::Allowed.key_value().value,
        Value::Bool(true)
    );
    assert_eq!(
        myappattr::TaskSampleRateAttr::All.key_value().value,
        Value::F64(1.0)
    );
}
