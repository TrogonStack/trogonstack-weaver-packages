use generated_semconv::{myappspan, tracer};
fn main() {
    myappspan::start_task_dispatch(
        &opentelemetry::Context::new(),
        &tracer::Tracer::default(),
        [],
    );
}
