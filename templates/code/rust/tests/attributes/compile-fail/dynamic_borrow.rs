fn main() {
    let source = String::from("dynamic");
    let _ = generated_semconv::attribute::MyappTaskId::from(source.as_str());
}
