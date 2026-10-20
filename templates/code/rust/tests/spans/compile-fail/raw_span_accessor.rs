use generated_semconv::trace;
fn main() {
    let mut span = trace::MyappTaskDispatch::default();
    span.span().set_attribute(opentelemetry::KeyValue::new("any", "value"));
}
