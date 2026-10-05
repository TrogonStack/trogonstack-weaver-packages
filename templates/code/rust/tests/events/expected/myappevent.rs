#![allow(deprecated)]
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
    /// Opt-in: the convention records it only when a user asks for it.
    HostName(super::workerattr::HostNameAttr),
}
/// A task reached a final state.
///
/// Emitted once per task, after its last attempt.
pub fn r#emit_task_finished<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#myapp_task_state: super::myappattr::TaskStateAttr, options: impl IntoIterator<Item = TaskFinishedOption>) {
    use ::opentelemetry::logs::LogRecord;
    use ::opentelemetry::trace::TraceContextExt;
    let _context_guard = context.clone().attach();
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes: Vec<::opentelemetry::KeyValue> = vec![r#myapp_task_state.key_value()];
    for option in options {
        match option {
            TaskFinishedOption::Severity(value) => severity = value,
            TaskFinishedOption::Timestamp(value) => timestamp = Some(value),
            TaskFinishedOption::BoolValues(value) => attributes.push(value.key_value()),
            TaskFinishedOption::DoubleValues(value) => attributes.push(value.key_value()),
            TaskFinishedOption::IntValues(value) => attributes.push(value.key_value()),
            TaskFinishedOption::StringValues(value) => attributes.push(value.key_value()),
            TaskFinishedOption::TaskAttempt(value) => attributes.push(value.key_value()),
            TaskFinishedOption::HostName(value) => attributes.push(value.key_value()),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some("myapp.task.finished")) { return; }
    let mut record = logger.inner().create_log_record();
    record.set_event_name("myapp.task.finished");
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
pub enum TaskRetriedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
}
/// A task is about to run again.
/// Opt-in: record only when the user requests it.
#[deprecated(note = "Read the attempt from `myapp.task.finished` instead.")]
pub fn r#emit_task_retried<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#myapp_task_attempt: super::myappattr::TaskAttemptAttr, options: impl IntoIterator<Item = TaskRetriedOption>) {
    use ::opentelemetry::logs::LogRecord;
    use ::opentelemetry::trace::TraceContextExt;
    let _context_guard = context.clone().attach();
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let attributes: Vec<::opentelemetry::KeyValue> = vec![r#myapp_task_attempt.key_value()];
    for option in options {
        match option {
            TaskRetriedOption::Severity(value) => severity = value,
            TaskRetriedOption::Timestamp(value) => timestamp = Some(value),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some("myapp.task.retried")) { return; }
    let mut record = logger.inner().create_log_record();
    record.set_event_name("myapp.task.retried");
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    for attribute in attributes { record.add_attribute(attribute.key, super::logger::log_value(attribute.value)); }
    logger.inner().emit(record);
}
