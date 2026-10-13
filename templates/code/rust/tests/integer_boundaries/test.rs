use generated_semconv::attribute::DemoBoundary;
use opentelemetry::Value;
#[test]
fn integer_enum_preserves_signed_64_bit_limits() {
    assert_eq!(
        ::opentelemetry::KeyValue::from(DemoBoundary::Minimum).value,
        Value::I64(i64::MIN)
    );
    assert_eq!(
        ::opentelemetry::KeyValue::from(DemoBoundary::Maximum).value,
        Value::I64(i64::MAX)
    );
}
