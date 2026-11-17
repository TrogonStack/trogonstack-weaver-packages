use generated_semconv::{attribute, trace, tracer};
fn main() {
    trace::start_myapp_task_run_by_task_id(
        &opentelemetry::Context::new(),
        &tracer::Tracer::default(),
        attribute::MyappTaskState::Queued,
    );
}
