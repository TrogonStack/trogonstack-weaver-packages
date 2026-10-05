use generated_semconv::{logger, myappattr, myappevent, myappmetric, myappspan, tracer};
use opentelemetry::KeyValue;

#[test]
fn typed_dependencies_and_plain_imported_values_work_together() {
    let error = upstream::errorattr::TypeAttr::Timeout;
    assert_eq!(
        KeyValue::from(error),
        KeyValue::new("error.type", "timeout")
    );
    myappmetric::TaskFailedCounter::default().add(
        1,
        error,
        myappattr::TaskIdAttr::from("task-1"),
        [myappmetric::TaskFailedCounterAttr::ServerPort(443)],
    );
    let context = opentelemetry::Context::new();
    myappevent::emit_task_errored(
        &context,
        &logger::Logger::default(),
        error,
        [myappevent::TaskErroredOption::ServerPort(443)],
    );
    let mut span = myappspan::start_task_dispatch(
        &context,
        &tracer::Tracer::default(),
        error,
        myappattr::TaskIdAttr::from("task-1"),
        [myappspan::TaskDispatchStartAttr::ServerPort(443)],
    );
    span.end();
}
