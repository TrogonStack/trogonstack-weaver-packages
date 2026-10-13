#![allow(deprecated)]
use ::opentelemetry::logs::LogRecord;
use ::opentelemetry::trace::TraceContextExt;

/// A noteworthy exception occurred.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Exception;

impl Exception {
    pub const NAME: &'static str = "exception";
}

#[derive(Clone, Debug)]
pub enum ExceptionOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
    Message(super::attribute::ExceptionMessage),
}
/// A noteworthy exception occurred.
pub fn r#emit_exception<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#exception_type: super::attribute::ExceptionType, options: impl IntoIterator<Item = ExceptionOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes = ::smallvec::SmallVec::<[ExceptionOption; 1]>::new();
    for option in options {
        match option {
            ExceptionOption::Severity(value) => severity = value,
            ExceptionOption::Timestamp(value) => timestamp = Some(value),
            attribute => attributes.push(attribute),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some(Exception::NAME)) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name(Exception::NAME);
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    record.add_attribute(super::attribute::ExceptionType::KEY, r#exception_type);
    for option in attributes {
        match option {
            ExceptionOption::Message(value) => record.add_attribute(super::attribute::ExceptionMessage::KEY, value),
            ExceptionOption::Severity(_) | ExceptionOption::Timestamp(_) => continue,
        }
    }
    logger.inner().emit(record);
}
