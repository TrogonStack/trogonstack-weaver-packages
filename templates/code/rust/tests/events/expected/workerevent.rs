#![allow(deprecated)]
use ::opentelemetry::logs::LogRecord;
use ::opentelemetry::trace::TraceContextExt;

#[derive(Clone, Debug)]
pub enum HostStartedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
}
/// A worker host started taking tasks.
pub fn r#emit_host_started<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, options: impl IntoIterator<Item = HostStartedOption>) {
    let _context_guard = context.clone().attach();
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let attributes: Vec<::opentelemetry::KeyValue> = vec![];
    for option in options {
        match option {
            HostStartedOption::Severity(value) => severity = value,
            HostStartedOption::Timestamp(value) => timestamp = Some(value),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some("worker.host.started")) { return; }
    let mut record = logger.inner().create_log_record();
    record.set_event_name("worker.host.started");
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    for attribute in attributes { record.add_attribute(attribute.key, super::logger::log_value(attribute.value)); }
    logger.inner().emit(record);
}
#[derive(Clone, Debug)]
pub enum TaskFinishedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
    BoolValues(super::myappattr::BoolValuesAttr),
    DoubleValues(super::myappattr::DoubleValuesAttr),
    IntValues(super::myappattr::IntValuesAttr),
    StringValues(super::myappattr::StringValuesAttr),
    /// Recommended: When the task ran more than once.
    TaskAttempt(super::myappattr::TaskAttemptAttr),
}
/// A task reached a final state on a worker.
///
/// Emitted once per task, after its last attempt.
pub fn r#emit_task_finished<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#myapp_task_state: super::myappattr::TaskStateAttr, r#worker_host_name: super::workerattr::HostNameAttr, options: impl IntoIterator<Item = TaskFinishedOption>) {
    let _context_guard = context.clone().attach();
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes: Vec<::opentelemetry::KeyValue> = vec![::opentelemetry::KeyValue::from(r#myapp_task_state), ::opentelemetry::KeyValue::from(r#worker_host_name)];
    for option in options {
        match option {
            TaskFinishedOption::Severity(value) => severity = value,
            TaskFinishedOption::Timestamp(value) => timestamp = Some(value),
            TaskFinishedOption::BoolValues(value) => attributes.push(::opentelemetry::KeyValue::from(value)),
            TaskFinishedOption::DoubleValues(value) => attributes.push(::opentelemetry::KeyValue::from(value)),
            TaskFinishedOption::IntValues(value) => attributes.push(::opentelemetry::KeyValue::from(value)),
            TaskFinishedOption::StringValues(value) => attributes.push(::opentelemetry::KeyValue::from(value)),
            TaskFinishedOption::TaskAttempt(value) => attributes.push(::opentelemetry::KeyValue::from(value)),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some("worker.task.finished")) { return; }
    let mut record = logger.inner().create_log_record();
    record.set_event_name("worker.task.finished");
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    for attribute in attributes { record.add_attribute(attribute.key, super::logger::log_value(attribute.value)); }
    logger.inner().emit(record);
}
