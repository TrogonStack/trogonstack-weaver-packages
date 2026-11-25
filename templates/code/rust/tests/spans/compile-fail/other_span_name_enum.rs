use generated_semconv::{attribute, trace, tracer};
fn main() {
    trace::start_myapp_task_retry(
        &opentelemetry::Context::new(),
        &tracer::Tracer::default(),
        trace::MyappTaskRunName::ByState,
        attribute::MyappTaskState::Queued,
        [],
    );
}
