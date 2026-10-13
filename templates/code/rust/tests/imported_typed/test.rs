use generated_semconv::{attribute, event, logger, metric, trace, tracer};
use opentelemetry::KeyValue;

#[test]
fn typed_dependencies_and_plain_imported_values_work_together() {
    let error = upstream::attribute::ErrorType::Timeout;
    assert_eq!(
        KeyValue::from(error),
        KeyValue::new("error.type", "timeout")
    );
    metric::MyappTaskFailed::default().add(
        1,
        error,
        attribute::MyappTaskId::from("task-1"),
        [metric::MyappTaskFailedAttr::ServerPort(443)],
    );
    let context = opentelemetry::Context::new();
    event::emit_myapp_task_errored(
        &context,
        &logger::Logger::default(),
        error,
        [event::MyappTaskErroredOption::ServerPort(443)],
    );
    let mut span = trace::start_myapp_task_dispatch(
        &context,
        &tracer::Tracer::default(),
        error,
        attribute::MyappTaskId::from("task-1"),
        [trace::MyappTaskDispatchStartAttr::ServerPort(443)],
    );
    span.end();
}
