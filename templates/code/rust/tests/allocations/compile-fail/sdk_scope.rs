use generated_semconv::options::InstrumentationOptions;

fn main() {
    let _ = InstrumentationOptions {
        scope: opentelemetry::InstrumentationScope::builder("caller-identity")
            .with_version("1.2.3")
            .build(),
    };
}
