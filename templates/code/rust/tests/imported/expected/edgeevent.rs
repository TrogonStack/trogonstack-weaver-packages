#![allow(deprecated)]
use ::opentelemetry::logs::LogRecord;
use ::opentelemetry::trace::TraceContextExt;

#[derive(Clone, Debug)]
pub enum UpstreamFailedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
    HttpRequestMethodList(Vec<::opentelemetry::StringValue>),
    ServerAddress(::opentelemetry::StringValue),
}
/// A request to an upstream server failed.
pub fn r#emit_upstream_failed<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#error_type: ::opentelemetry::StringValue, options: impl IntoIterator<Item = UpstreamFailedOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes = ::smallvec::SmallVec::<[UpstreamFailedOption; 2]>::new();
    for option in options {
        match option {
            UpstreamFailedOption::Severity(value) => severity = value,
            UpstreamFailedOption::Timestamp(value) => timestamp = Some(value),
            attribute => attributes.push(attribute),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some("edge.upstream.failed")) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name("edge.upstream.failed");
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    record.add_attribute("error.type", r#error_type);
    for option in attributes {
        match option {
            UpstreamFailedOption::HttpRequestMethodList(value) => record.add_attribute("http.request.method_list", value.into_iter().collect::<::opentelemetry::logs::AnyValue>()),
            UpstreamFailedOption::ServerAddress(value) => record.add_attribute("server.address", value),
            UpstreamFailedOption::Severity(_) | UpstreamFailedOption::Timestamp(_) => continue,
        }
    }
    logger.inner().emit(record);
}
