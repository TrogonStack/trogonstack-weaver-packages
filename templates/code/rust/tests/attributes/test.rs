use generated_semconv::{authattr, myappattr, SCHEMA_URL};
use opentelemetry::{Array, KeyValue, Value};

#[test]
fn typed_attributes_preserve_keys_values_and_schema() {
    assert_eq!(SCHEMA_URL, "https://example.com/schemas/attributes/1.0.0");
    let value: KeyValue = myappattr::TaskIdAttr::from("task").into();
    assert_eq!(value, KeyValue::new("myapp.task.id", "task"));
    assert_eq!(
        KeyValue::from(myappattr::TaskRetriesAttr::from(4)),
        KeyValue::new("myapp.task.retries", 4_i64)
    );
    assert_eq!(
        KeyValue::from(myappattr::TaskCancelledAttr::from(true)).value,
        Value::Bool(true)
    );
    assert_eq!(
        KeyValue::from(myappattr::TaskProgressAttr::from(0.5)).value,
        Value::F64(0.5)
    );
    assert_eq!(
        KeyValue::from(myappattr::TaskTagsAttr::from(vec!["tag".into()])).value,
        Value::Array(Array::String(vec!["tag".into()]))
    );
    assert_eq!(
        KeyValue::from(myappattr::TaskShardIdsAttr::from(vec![1, 2])).value,
        Value::Array(Array::I64(vec![1, 2]))
    );
    assert_eq!(
        KeyValue::from(myappattr::TaskWeightsAttr::from(vec![0.25])).value,
        Value::Array(Array::F64(vec![0.25]))
    );
    assert_eq!(
        KeyValue::from(myappattr::TaskFlagsAttr::from(vec![false])).value,
        Value::Array(Array::Bool(vec![false]))
    );
    assert_eq!(
        KeyValue::from(myappattr::TaskStateAttr::Running).value,
        Value::String("running".into())
    );
    assert_eq!(KeyValue::from(myappattr::TaskEscapedAttr::Quoted).value, Value::String("quote\" slash\\ newline\n tab\t backspace\u{0008} formfeed\u{000c} null\u{0000} snowman☃".into()));
    assert_eq!(
        KeyValue::from(authattr::LevelAttr::High).value,
        Value::I64(2)
    );
    assert_eq!(
        KeyValue::from(myappattr::TaskPreemptibleAttr::Allowed).value,
        Value::Bool(true)
    );
    assert_eq!(
        KeyValue::from(myappattr::TaskSampleRateAttr::All).value,
        Value::F64(1.0)
    );
}

#[test]
fn string_attributes_accept_borrowed_owned_and_inferred_conversions() {
    let owned = String::from("owned");
    let borrowed: myappattr::TaskIdAttr = {
        let source = String::from("borrowed");
        source.as_str().into()
    };
    assert_eq!(
        KeyValue::from(myappattr::TaskIdAttr::from(owned)),
        KeyValue::new("myapp.task.id", "owned")
    );
    assert_eq!(
        KeyValue::from(borrowed),
        KeyValue::new("myapp.task.id", "borrowed")
    );
    let inferred: myappattr::TaskIdAttr = "inferred".into();
    assert_eq!(
        KeyValue::from(inferred),
        KeyValue::new("myapp.task.id", "inferred")
    );
    let retries: myappattr::TaskRetriesAttr = 7_i64.into();
    assert_eq!(KeyValue::from(retries).value, Value::I64(7));
    let tags: myappattr::TaskTagsAttr = vec![String::from("tag")].into();
    assert_eq!(
        KeyValue::from(tags).value,
        Value::Array(Array::String(vec!["tag".into()]))
    );
}
