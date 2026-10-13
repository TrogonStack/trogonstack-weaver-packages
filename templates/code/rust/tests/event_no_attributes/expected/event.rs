#![allow(deprecated)]
use ::opentelemetry::logs::LogRecord;
use ::opentelemetry::trace::TraceContextExt;

/// A synthetic tick occurred.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProbeTick;

impl ProbeTick {
    pub const NAME: &'static str = "probe.tick";
}

#[derive(Clone, Debug)]
pub enum ProbeTickOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
}
/// A synthetic tick occurred.
pub fn r#emit_probe_tick<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, options: impl IntoIterator<Item = ProbeTickOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    for option in options {
        match option {
            ProbeTickOption::Severity(value) => severity = value,
            ProbeTickOption::Timestamp(value) => timestamp = Some(value),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some(ProbeTick::NAME)) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name(ProbeTick::NAME);
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    logger.inner().emit(record);
}
