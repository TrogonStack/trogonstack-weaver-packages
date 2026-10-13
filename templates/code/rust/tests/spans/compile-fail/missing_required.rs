use generated_semconv::{trace, tracer};
fn main() {
    trace::start_myapp_task_dispatch(
        &opentelemetry::Context::new(),
        &tracer::Tracer::default(),
        [],
    );
}
