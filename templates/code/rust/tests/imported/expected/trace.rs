#![allow(deprecated)]
use ::opentelemetry::trace::{TraceContextExt as _, Tracer as _};

mod span_name {
    use ::std::fmt::Write as _;

    pub(super) enum Part {
        Text(&'static str),
        Attribute(&'static str),
    }

    pub(super) fn select(attributes: &[::opentelemetry::KeyValue], fallback: &'static str, templates: &[&[Part]]) -> ::std::borrow::Cow<'static, str> {
        for template in templates {
            let Some(capacity) = capacity(attributes, template) else { continue };
            let mut name = String::with_capacity(capacity);
            for part in *template {
                match part {
                    Part::Text(text) => name.push_str(text),
                    Part::Attribute(key) => {
                        if let Some(value) = value(attributes, key) {
                            write!(&mut name, "{value}").expect("writing a span name to String cannot fail");
                        }
                    }
                }
            }
            return ::std::borrow::Cow::Owned(name);
        }
        ::std::borrow::Cow::Borrowed(fallback)
    }

    fn value<'a>(attributes: &'a [::opentelemetry::KeyValue], key: &str) -> Option<&'a ::opentelemetry::Value> {
        attributes.iter().rev().find(|attribute| attribute.key.as_str() == key).map(|attribute| &attribute.value)
    }

    fn capacity(attributes: &[::opentelemetry::KeyValue], template: &[Part]) -> Option<usize> {
        template.iter().try_fold(0, |capacity, part| {
            let length = match part {
                Part::Text(text) => text.len(),
                Part::Attribute(key) => length(value(attributes, key)?)?,
            };
            Some(capacity + length)
        })
    }

    fn length(value: &::opentelemetry::Value) -> Option<usize> {
        use ::opentelemetry::{Array, Value};
        match value {
            Value::String(text) if text.as_str().is_empty() => None,
            Value::String(text) => Some(text.as_str().len()),
            Value::I64(_) => Some(20),
            Value::F64(_) => Some(328),
            Value::Bool(_) => Some(5),
            Value::Array(Array::String(values)) if values.is_empty() => None,
            Value::Array(Array::String(values)) => Some(values.iter().map(|value| value.as_str().len() + 3).sum::<usize>() + 2),
            Value::Array(Array::I64(values)) => (!values.is_empty()).then(|| values.len() * 21 + 2),
            Value::Array(Array::F64(values)) => (!values.is_empty()).then(|| values.len() * 328 + 2),
            Value::Array(Array::Bool(values)) => (!values.is_empty()).then(|| values.len() * 6 + 2),
            _ => Some(0),
        }
    }
}

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

/// The span is named from the first of these templates whose attributes are all set and not empty: `{http.request.method_list}`. Otherwise it is named `edge.upstream.methods`.
pub fn r#start_edge_upstream_methods<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#http_request_method_list: Vec<::opentelemetry::StringValue>) -> EdgeUpstreamMethods<T::Span> {
    let attributes: ::std::vec::Vec<::opentelemetry::KeyValue> = [::opentelemetry::KeyValue::new("http.request.method_list", ::opentelemetry::Value::Array(::opentelemetry::Array::String(r#http_request_method_list)))].into_iter().collect();
    let name = span_name::select(&attributes, "edge.upstream.methods", &[
        &[span_name::Part::Attribute("http.request.method_list")],
    ]);
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

/// The span is named from the first of these templates whose attributes are all set and not empty: `{server.address}:{server.port}`. Otherwise it is named `edge.upstream.request`.
pub fn r#start_edge_upstream_request<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#server_address: ::opentelemetry::StringValue, r#server_port: i64, options: impl IntoIterator<Item = EdgeUpstreamRequestStartAttr>) -> EdgeUpstreamRequest<T::Span> {
    let attributes: ::std::vec::Vec<::opentelemetry::KeyValue> = [::opentelemetry::KeyValue::new("server.address", r#server_address), ::opentelemetry::KeyValue::new("server.port", r#server_port)].into_iter().chain(options.into_iter().map(::opentelemetry::KeyValue::from)).collect();
    let name = span_name::select(&attributes, "edge.upstream.request", &[
        &[span_name::Part::Attribute("server.address"), span_name::Part::Text(":"), span_name::Part::Attribute("server.port")],
    ]);
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Client)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    EdgeUpstreamRequest { span }
}
