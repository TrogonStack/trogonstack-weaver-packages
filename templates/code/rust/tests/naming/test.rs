#![allow(non_camel_case_types, non_snake_case)]
use generated_semconv::{attribute, meter, metric};
use opentelemetry::KeyValue;

#[test]
fn names_preserve_capitalization_and_digits_in_attributes_and_signals() {
    let identifier = attribute::CapsHttp2STATUSApiID::from("value");
    assert_eq!(
        KeyValue::from(identifier.clone()),
        KeyValue::new("caps.http2_STATUS.api_ID", "value")
    );
    let mode = attribute::CapsMode2STATE::HTTP2ReadySTATE;
    let metric = metric::CapsHttp2STATUSCount::new(&meter::Meter::default());
    metric.add(1, identifier, mode);
}
