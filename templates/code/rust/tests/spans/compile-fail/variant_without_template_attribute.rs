use generated_semconv::{attribute, trace, tracer};
fn main() {
    trace::start_myapp_task_run(
        &opentelemetry::Context::new(),
        &tracer::Tracer::default(),
        trace::MyappTaskRunName::ByTaskId {},
        attribute::MyappTaskState::Queued,
        [],
    );
}
