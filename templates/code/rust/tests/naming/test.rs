#![allow(non_camel_case_types, non_snake_case)]
use generated_semconv::{capsattr, capsmetric, meter};
use opentelemetry::KeyValue;

#[test]
fn names_preserve_capitalization_and_digits_in_attributes_and_signals() {
    let identifier = capsattr::Http2STATUSApiIDAttr::from("value");
    assert_eq!(
        KeyValue::from(identifier.clone()),
        KeyValue::new("caps.http2_STATUS.api_ID", "value")
    );
    let mode = capsattr::Mode2STATEAttr::HTTP2ReadySTATE;
    let metric = capsmetric::Http2STATUSCountCounter::new(&meter::Meter::default());
    metric.add(1, identifier, mode);
}
