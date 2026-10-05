fn main() {
    let source = String::from("dynamic");
    let _ = generated_semconv::myappattr::TaskIdAttr::from(source.as_str());
}
