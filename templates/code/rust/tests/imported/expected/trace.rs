#![allow(deprecated)]
use ::opentelemetry::trace::{TraceContextExt as _, Tracer as _};
use ::std::fmt::Write as _;

/// A request named from attempted methods.
pub struct EdgeUpstreamMethods<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> EdgeUpstreamMethods<S> {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context
    where S: Send + Sync + 'static,
    {
        parent.with_span(self.span)
    }
    pub fn span_context(&self) -> &::opentelemetry::trace::SpanContext { ::opentelemetry::trace::Span::span_context(&self.span) }
    pub fn is_recording(&self) -> bool { ::opentelemetry::trace::Span::is_recording(&self.span) }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
    pub fn add_event(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>) {
        ::opentelemetry::trace::Span::add_event(&mut self.span, name, ::std::vec::Vec::new());
    }
    pub fn add_event_with_timestamp(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>, timestamp: ::std::time::SystemTime) {
        ::opentelemetry::trace::Span::add_event_with_timestamp(&mut self.span, name, timestamp, ::std::vec::Vec::new());
    }
    pub fn add_link(&mut self, span_context: ::opentelemetry::trace::SpanContext) {
        ::opentelemetry::trace::Span::add_link(&mut self.span, span_context, ::std::vec::Vec::new());
    }
}

impl EdgeUpstreamMethods {
    pub const NAME: &'static str = "edge.upstream.methods";
}

impl Default for EdgeUpstreamMethods {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_edge_upstream_methods<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#http_request_method_list: Vec<::opentelemetry::StringValue>) -> EdgeUpstreamMethods<T::Span> {
    let mut name = String::with_capacity(r#http_request_method_list.iter().map(|value| value.as_str().len() + 3).sum::<usize>() + 2);
    name.push('[');
    for (index, value) in r#http_request_method_list.iter().enumerate() {
        if index > 0 { name.push(','); }
        write!(&mut name, "\"{}\"", value).expect("writing a span name to String cannot fail");
    }
    name.push(']');
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::new("http.request.method_list", ::opentelemetry::Value::Array(::opentelemetry::Array::String(r#http_request_method_list)))];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Client)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    EdgeUpstreamMethods { span }
}
#[derive(Clone, Debug)]
pub enum EdgeUpstreamRequestStartAttr {
    /// Conditionally required: If the request failed.
    ErrorType(::opentelemetry::StringValue),
    HttpRequestMethodList(Vec<::opentelemetry::StringValue>),
}

impl From<EdgeUpstreamRequestStartAttr> for ::opentelemetry::KeyValue {
    fn from(value: EdgeUpstreamRequestStartAttr) -> Self {
        match value {
            EdgeUpstreamRequestStartAttr::ErrorType(value) => ::opentelemetry::KeyValue::new("error.type", value),
            EdgeUpstreamRequestStartAttr::HttpRequestMethodList(value) => ::opentelemetry::KeyValue::new("http.request.method_list", ::opentelemetry::Value::Array(::opentelemetry::Array::String(value))),
        }
    }
}
#[derive(Clone, Debug)]
pub enum EdgeUpstreamRequestAttr {
    HttpRequestMethodList(Vec<::opentelemetry::StringValue>),
}

impl From<EdgeUpstreamRequestAttr> for EdgeUpstreamRequestStartAttr {
    fn from(value: EdgeUpstreamRequestAttr) -> Self {
        match value {
            EdgeUpstreamRequestAttr::HttpRequestMethodList(value) => Self::HttpRequestMethodList(value),
        }
    }
}
/// A request to an upstream server.
pub struct EdgeUpstreamRequest<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> EdgeUpstreamRequest<S> {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context
    where S: Send + Sync + 'static,
    {
        parent.with_span(self.span)
    }
    pub fn span_context(&self) -> &::opentelemetry::trace::SpanContext { ::opentelemetry::trace::Span::span_context(&self.span) }
    pub fn is_recording(&self) -> bool { ::opentelemetry::trace::Span::is_recording(&self.span) }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
    pub fn add_event(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>, attributes: impl IntoIterator<Item = EdgeUpstreamRequestAttr>) {
        ::opentelemetry::trace::Span::add_event(&mut self.span, name, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(EdgeUpstreamRequestStartAttr::from(attribute))).collect());
    }
    pub fn add_event_with_timestamp(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>, timestamp: ::std::time::SystemTime, attributes: impl IntoIterator<Item = EdgeUpstreamRequestAttr>) {
        ::opentelemetry::trace::Span::add_event_with_timestamp(&mut self.span, name, timestamp, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(EdgeUpstreamRequestStartAttr::from(attribute))).collect());
    }
    pub fn add_link(&mut self, span_context: ::opentelemetry::trace::SpanContext, attributes: impl IntoIterator<Item = EdgeUpstreamRequestAttr>) {
        ::opentelemetry::trace::Span::add_link(&mut self.span, span_context, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(EdgeUpstreamRequestStartAttr::from(attribute))).collect());
    }
    pub fn set_attributes(&mut self, attributes: impl IntoIterator<Item = EdgeUpstreamRequestAttr>) {
        for attribute in attributes { ::opentelemetry::trace::Span::set_attribute(&mut self.span, ::opentelemetry::KeyValue::from(EdgeUpstreamRequestStartAttr::from(attribute))); }
    }
}

impl EdgeUpstreamRequest {
    pub const NAME: &'static str = "edge.upstream.request";
}

impl Default for EdgeUpstreamRequest {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_edge_upstream_request<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#server_address: ::opentelemetry::StringValue, r#server_port: i64, options: impl IntoIterator<Item = EdgeUpstreamRequestStartAttr>) -> EdgeUpstreamRequest<T::Span> {
    let mut name = String::with_capacity(r#server_address.as_str().len() + ":".len() + 20);
    write!(&mut name, "{}", r#server_address).expect("writing a span name to String cannot fail");
    name.push(':');
    write!(&mut name, "{}", r#server_port).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 2] = [::opentelemetry::KeyValue::new("server.address", r#server_address), ::opentelemetry::KeyValue::new("server.port", r#server_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Client)
        .with_attributes(attributes.into_iter().chain(options.into_iter().map(::opentelemetry::KeyValue::from)))
        .start_with_context(tracer.inner(), context);
    EdgeUpstreamRequest { span }
}
