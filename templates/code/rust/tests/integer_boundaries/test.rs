use generated_semconv::demoattr::BoundaryAttr;
use opentelemetry::Value;
#[test]
fn integer_enum_preserves_signed_64_bit_limits() {
    assert_eq!(
        ::opentelemetry::KeyValue::from(BoundaryAttr::Minimum).value,
        Value::I64(i64::MIN)
    );
    assert_eq!(
        ::opentelemetry::KeyValue::from(BoundaryAttr::Maximum).value,
        Value::I64(i64::MAX)
    );
}
