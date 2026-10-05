#![allow(deprecated)]
use ::opentelemetry::logs::LogRecord;
use ::opentelemetry::trace::TraceContextExt;

#[derive(Clone, Debug)]
pub enum TaskErroredOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
    ServerPort(i64),
}
/// A task failed with an error.
pub fn r#emit_task_errored<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#error_type: upstream::errorattr::TypeAttr, options: impl IntoIterator<Item = TaskErroredOption>) {
    let _context_guard = context.clone().attach();
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes: Vec<::opentelemetry::KeyValue> = vec![::opentelemetry::KeyValue::from(r#error_type)];
    for option in options {
        match option {
            TaskErroredOption::Severity(value) => severity = value,
            TaskErroredOption::Timestamp(value) => timestamp = Some(value),
            TaskErroredOption::ServerPort(value) => attributes.push(::opentelemetry::KeyValue::new("server.port", value)),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some("myapp.task.errored")) { return; }
    let mut record = logger.inner().create_log_record();
    record.set_event_name("myapp.task.errored");
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    for attribute in attributes { record.add_attribute(attribute.key, super::logger::log_value(attribute.value)); }
    logger.inner().emit(record);
}
