#![allow(deprecated)]
use ::opentelemetry::logs::LogRecord;
use ::opentelemetry::trace::TraceContextExt;

/// A measurement was recorded.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProbeRecorded;

impl ProbeRecorded {
    pub const NAME: &'static str = "probe.recorded";
}

#[derive(Clone, Debug)]
pub enum ProbeRecordedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
    Optional(super::attribute::ProbeOptional),
    Values(super::attribute::ProbeValues),
}
/// A measurement was recorded.
pub fn r#emit_probe_recorded<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#probe_required: super::attribute::ProbeRequired, options: impl IntoIterator<Item = ProbeRecordedOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes = ::smallvec::SmallVec::<[ProbeRecordedOption; 2]>::new();
    for option in options {
        match option {
            ProbeRecordedOption::Severity(value) => severity = value,
            ProbeRecordedOption::Timestamp(value) => timestamp = Some(value),
            attribute => attributes.push(attribute),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some(ProbeRecorded::NAME)) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name(ProbeRecorded::NAME);
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    record.add_attribute(super::attribute::ProbeRequired::KEY, r#probe_required);
    for option in attributes {
        match option {
            ProbeRecordedOption::Optional(value) => record.add_attribute(super::attribute::ProbeOptional::KEY, value),
            ProbeRecordedOption::Values(value) => record.add_attribute(super::attribute::ProbeValues::KEY, value),
            ProbeRecordedOption::Severity(_) | ProbeRecordedOption::Timestamp(_) => continue,
        }
    }
    logger.inner().emit(record);
}
