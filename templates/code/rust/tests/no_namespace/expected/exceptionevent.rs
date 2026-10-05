#![allow(deprecated)]
#[derive(Clone, Debug)]
pub enum Option {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),

    Message(super::exceptionattr::MessageAttr),
}
/// A noteworthy exception occurred.
pub fn r#emit_<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#exception_type: super::exceptionattr::TypeAttr, options: impl IntoIterator<Item = Option>) {
    use ::opentelemetry::logs::LogRecord;
    use ::opentelemetry::trace::TraceContextExt;
    let _context_guard = context.clone().attach();
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes: Vec<::opentelemetry::KeyValue> = vec![r#exception_type.key_value()];
    for option in options {
        match option {
            Option::Severity(value) => severity = value,
            Option::Timestamp(value) => timestamp = Some(value),
            Option::Message(value) => attributes.push(value.key_value()),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some("exception")) { return; }
    let mut record = logger.inner().create_log_record();
    record.set_event_name("exception");
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    for attribute in attributes { record.add_attribute(attribute.key, super::logger::log_value(attribute.value)); }
    logger.inner().emit(record);
}
