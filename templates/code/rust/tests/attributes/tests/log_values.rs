use generated_semconv::{attribute};
use opentelemetry::logs::AnyValue;

#[test]
fn typed_attributes_convert_directly_to_log_values() {
    let retries: AnyValue = attribute::MyappTaskRetries::from(4).into();
    assert_eq!(retries, AnyValue::Int(4));
    assert_eq!(
        AnyValue::from(attribute::MyappTaskProgress::from(0.5)),
        AnyValue::Double(0.5)
    );
    assert_eq!(
        AnyValue::from(attribute::MyappTaskCancelled::from(true)),
        AnyValue::Boolean(true)
    );
    assert_eq!(
        AnyValue::from(attribute::MyappTaskState::Running),
        AnyValue::String("running".into())
    );
    assert_eq!(AnyValue::from(attribute::AuthLevel::High), AnyValue::Int(2));
    assert_eq!(
        AnyValue::from(attribute::MyappTaskPreemptible::Allowed),
        AnyValue::Boolean(true)
    );
    assert_eq!(
        AnyValue::from(attribute::MyappTaskSampleRate::Tenth),
        AnyValue::Double(0.1)
    );
}

#[test]
fn log_conversion_preserves_string_storage() {
    for attribute in [
        attribute::MyappTaskId::from("static-task"),
        attribute::MyappTaskId::from(String::from("owned-task")),
        attribute::MyappTaskId::from(opentelemetry::StringValue::from(
            std::sync::Arc::<str>::from("shared-task"),
        )),
    ] {
        let pointer = attribute.as_ref().as_ptr();
        let AnyValue::String(value) = AnyValue::from(attribute) else {
            panic!("expected string log value");
        };
        assert_eq!(value.as_str().as_ptr(), pointer);
    }
}

#[test]
fn typed_arrays_export_log_lists_with_order_and_duplicates() {
    assert_eq!(
        AnyValue::from(attribute::MyappTaskShardIds::from(vec![1, 1, 2])),
        AnyValue::ListAny(Box::new(vec![
            AnyValue::Int(1),
            AnyValue::Int(1),
            AnyValue::Int(2),
        ]))
    );
    assert_eq!(
        AnyValue::from(attribute::MyappTaskWeights::from(vec![0.25, 0.75])),
        AnyValue::ListAny(Box::new(vec![
            AnyValue::Double(0.25),
            AnyValue::Double(0.75),
        ]))
    );
    assert_eq!(
        AnyValue::from(attribute::MyappTaskFlags::from(vec![true, false])),
        AnyValue::ListAny(Box::new(vec![
            AnyValue::Boolean(true),
            AnyValue::Boolean(false),
        ]))
    );
    let tag = opentelemetry::StringValue::from(String::from("owned-tag"));
    let pointer = tag.as_str().as_ptr();
    let AnyValue::ListAny(values) = AnyValue::from(attribute::MyappTaskTags::from(vec![tag])) else {
        panic!("expected string log list");
    };
    let AnyValue::String(value) = &values[0] else {
        panic!("expected string list element");
    };
    assert_eq!(value.as_str(), "owned-tag");
    assert_eq!(value.as_str().as_ptr(), pointer);
    assert_eq!(
        AnyValue::from(attribute::MyappTaskTags::from(Vec::<
            opentelemetry::StringValue,
        >::new())),
        AnyValue::ListAny(Box::default())
    );
}
