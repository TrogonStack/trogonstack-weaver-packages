#![allow(deprecated)]
use ::opentelemetry::logs::LogRecord;
use ::opentelemetry::trace::TraceContextExt;

/// A request to an upstream server failed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EdgeUpstreamFailed;

impl EdgeUpstreamFailed {
    pub const NAME: &'static str = "edge.upstream.failed";
}

#[derive(Clone, Debug)]
pub enum EdgeUpstreamFailedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
    HttpRequestMethodList(Vec<::opentelemetry::StringValue>),
    ServerAddress(::opentelemetry::StringValue),
}
/// A request to an upstream server failed.
pub fn r#emit_edge_upstream_failed<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#error_type: ::opentelemetry::StringValue, options: impl IntoIterator<Item = EdgeUpstreamFailedOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes = ::smallvec::SmallVec::<[EdgeUpstreamFailedOption; 2]>::new();
    for option in options {
        match option {
            EdgeUpstreamFailedOption::Severity(value) => severity = value,
            EdgeUpstreamFailedOption::Timestamp(value) => timestamp = Some(value),
            attribute => attributes.push(attribute),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some(EdgeUpstreamFailed::NAME)) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name(EdgeUpstreamFailed::NAME);
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    record.add_attribute("error.type", r#error_type);
    for option in attributes {
        match option {
            EdgeUpstreamFailedOption::HttpRequestMethodList(value) => record.add_attribute("http.request.method_list", value.into_iter().collect::<::opentelemetry::logs::AnyValue>()),
            EdgeUpstreamFailedOption::ServerAddress(value) => record.add_attribute("server.address", value),
            EdgeUpstreamFailedOption::Severity(_) | EdgeUpstreamFailedOption::Timestamp(_) => continue,
        }
    }
    logger.inner().emit(record);
}
