#![allow(deprecated)]
use ::opentelemetry::logs::LogRecord;
use ::opentelemetry::trace::TraceContextExt;

#[derive(Clone, Debug)]
pub enum RecordedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
    Optional(super::probeattr::OptionalAttr),
    Values(super::probeattr::ValuesAttr),
}
/// A measurement was recorded.
pub fn r#emit_recorded<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#probe_required: super::probeattr::RequiredAttr, options: impl IntoIterator<Item = RecordedOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes = ::smallvec::SmallVec::<[RecordedOption; 2]>::new();
    for option in options {
        match option {
            RecordedOption::Severity(value) => severity = value,
            RecordedOption::Timestamp(value) => timestamp = Some(value),
            attribute => attributes.push(attribute),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some("probe.recorded")) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name("probe.recorded");
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    record.add_attribute(super::probeattr::RequiredAttr::KEY, r#probe_required);
    for option in attributes {
        match option {
            RecordedOption::Optional(value) => record.add_attribute(super::probeattr::OptionalAttr::KEY, value),
            RecordedOption::Values(value) => record.add_attribute(super::probeattr::ValuesAttr::KEY, value),
            RecordedOption::Severity(_) | RecordedOption::Timestamp(_) => continue,
        }
    }
    logger.inner().emit(record);
}
