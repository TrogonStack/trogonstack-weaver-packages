#![allow(deprecated)]
use ::opentelemetry::logs::LogRecord;
use ::opentelemetry::trace::TraceContextExt;

/// A task failed with an error.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MyappTaskErrored;

impl MyappTaskErrored {
    pub const NAME: &'static str = "myapp.task.errored";
}

#[derive(Clone, Debug)]
pub enum MyappTaskErroredOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
    ServerPort(i64),
}
/// A task failed with an error.
pub fn r#emit_myapp_task_errored<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#error_type: upstream::attribute::ErrorType, options: impl IntoIterator<Item = MyappTaskErroredOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes = ::smallvec::SmallVec::<[MyappTaskErroredOption; 1]>::new();
    for option in options {
        match option {
            MyappTaskErroredOption::Severity(value) => severity = value,
            MyappTaskErroredOption::Timestamp(value) => timestamp = Some(value),
            attribute => attributes.push(attribute),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some(MyappTaskErrored::NAME)) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name(MyappTaskErrored::NAME);
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    record.add_attribute(upstream::attribute::ErrorType::KEY, r#error_type);
    for option in attributes {
        match option {
            MyappTaskErroredOption::ServerPort(value) => record.add_attribute("server.port", value),
            MyappTaskErroredOption::Severity(_) | MyappTaskErroredOption::Timestamp(_) => continue,
        }
    }
    logger.inner().emit(record);
}
