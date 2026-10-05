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
        KeyValue::from(myappattr::TaskTagsAttr::from(vec![String::from("tag")])).value,
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
fn string_attributes_accept_explicit_owned_and_inferred_conversions() {
    let owned = String::from("owned");
    let borrowed: myappattr::TaskIdAttr = {
        let source = String::from("borrowed");
        source.into()
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

#[test]
fn static_and_native_strings_preserve_their_storage() {
    const STATIC: &str = "static-task";
    let attribute = myappattr::TaskIdAttr::from(STATIC);
    assert_eq!(attribute.as_ref().as_ptr(), STATIC.as_ptr());
    let Value::String(text) = KeyValue::from(attribute).value else {
        panic!("expected string");
    };
    assert_eq!(text.as_str().as_ptr(), STATIC.as_ptr());
    let Value::String(text) = KeyValue::from(myappattr::TaskStateAttr::Running).value else {
        panic!("expected string");
    };
    assert_eq!(text.as_str(), myappattr::TaskStateAttr::Running.as_ref());

    let shared: std::sync::Arc<str> = std::sync::Arc::from("shared-task");
    let native = opentelemetry::StringValue::from(shared.clone());
    let attribute = myappattr::TaskIdAttr::from(native);
    assert_eq!(attribute.as_ref().as_ptr(), shared.as_ptr());
    let Value::String(text) = KeyValue::from(attribute).value else {
        panic!("expected string");
    };
    assert_eq!(text.as_str().as_ptr(), shared.as_ptr());
    assert_eq!(std::sync::Arc::strong_count(&shared), 2);

    let native = vec![opentelemetry::StringValue::from("tag")];
    let original = native.as_ptr();
    let Value::Array(Array::String(values)) =
        KeyValue::from(myappattr::TaskTagsAttr::from(native)).value
    else {
        panic!("expected string array");
    };
    assert_eq!(values.as_ptr(), original);
}

#[test]
fn copy_scalars_and_borrowed_display_preserve_telemetry_formatting() {
    fn assert_copy<T: Copy>() {}
    assert_copy::<myappattr::TaskRetriesAttr>();
    assert_copy::<myappattr::TaskProgressAttr>();
    assert_copy::<myappattr::TaskCancelledAttr>();
    assert_copy::<myappattr::TaskStateAttr>();
    let retries = myappattr::TaskRetriesAttr::from(7);
    let _: KeyValue = retries.into();
    assert_eq!(retries.to_string(), "7");
    assert_eq!(myappattr::TaskCancelledAttr::from(true).to_string(), "true");
    assert_eq!(myappattr::TaskProgressAttr::from(0.5).to_string(), "0.5");
    assert_eq!(myappattr::TaskStateAttr::Running.to_string(), "running");
    assert_eq!(authattr::LevelAttr::High.to_string(), "2");
    assert_eq!(myappattr::TaskSampleRateAttr::Tenth.to_string(), "0.1");
    let string = myappattr::TaskIdAttr::from(String::from("dynamic"));
    assert_eq!(string.to_string(), string.as_ref());
    let array = myappattr::TaskTagsAttr::from(vec![String::from("a"), String::from("b")]);
    assert_eq!(array.to_string(), KeyValue::from(array).value.to_string());
    let array = myappattr::TaskShardIdsAttr::from(vec![1, 2]);
    assert_eq!(array.to_string(), "[1,2]");
    let array = myappattr::TaskWeightsAttr::from(vec![0.25, 0.5]);
    assert_eq!(array.to_string(), "[0.25,0.5]");
    let array = myappattr::TaskFlagsAttr::from(vec![true, false]);
    assert_eq!(array.to_string(), "[true,false]");
}
