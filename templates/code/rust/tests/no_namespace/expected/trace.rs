#![allow(deprecated)]
use ::opentelemetry::trace::{TraceContextExt as _, Tracer as _};

#[derive(Clone, Debug)]
pub enum HeartbeatStartAttr {
    Source(super::attribute::HeartbeatSource),
}

impl From<HeartbeatStartAttr> for ::opentelemetry::KeyValue {
    fn from(value: HeartbeatStartAttr) -> Self {
        match value {
            HeartbeatStartAttr::Source(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
#[derive(Clone, Debug)]
pub enum HeartbeatAttr {
    Source(super::attribute::HeartbeatSource),
}

impl From<HeartbeatAttr> for HeartbeatStartAttr {
    fn from(value: HeartbeatAttr) -> Self {
        match value {
            HeartbeatAttr::Source(value) => Self::Source(value),
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct HeartbeatName(::std::borrow::Cow<'static, str>);

impl From<&'static str> for HeartbeatName {
    fn from(value: &'static str) -> Self { Self(::std::borrow::Cow::Borrowed(value)) }
}

impl From<String> for HeartbeatName {
    fn from(value: String) -> Self { Self(::std::borrow::Cow::Owned(value)) }
}

impl From<::std::borrow::Cow<'static, str>> for HeartbeatName {
    fn from(value: ::std::borrow::Cow<'static, str>) -> Self { Self(value) }
}
/// A periodic liveness signal.
pub struct Heartbeat<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> Heartbeat<S> {
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
    pub fn update_name(&mut self, name: HeartbeatName) {
        let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed(Heartbeat::NAME) } else { name.0 };
        ::opentelemetry::trace::Span::update_name(&mut self.span, name);
    }
    pub fn add_event(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>, attributes: impl IntoIterator<Item = HeartbeatAttr>) {
        ::opentelemetry::trace::Span::add_event(&mut self.span, name, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(HeartbeatStartAttr::from(attribute))).collect());
    }
    pub fn add_event_with_timestamp(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>, timestamp: ::std::time::SystemTime, attributes: impl IntoIterator<Item = HeartbeatAttr>) {
        ::opentelemetry::trace::Span::add_event_with_timestamp(&mut self.span, name, timestamp, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(HeartbeatStartAttr::from(attribute))).collect());
    }
    pub fn add_link(&mut self, span_context: ::opentelemetry::trace::SpanContext, attributes: impl IntoIterator<Item = HeartbeatAttr>) {
        ::opentelemetry::trace::Span::add_link(&mut self.span, span_context, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(HeartbeatStartAttr::from(attribute))).collect());
    }
    pub fn set_attributes(&mut self, attributes: impl IntoIterator<Item = HeartbeatAttr>) {
        for attribute in attributes { ::opentelemetry::trace::Span::set_attribute(&mut self.span, ::opentelemetry::KeyValue::from(HeartbeatStartAttr::from(attribute))); }
    }
}

impl Heartbeat {
    pub const NAME: &'static str = "heartbeat";
}

impl Default for Heartbeat {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_heartbeat<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, name: HeartbeatName, r#heartbeat_sequence: super::attribute::HeartbeatSequence, options: impl IntoIterator<Item = HeartbeatStartAttr>) -> Heartbeat<T::Span> {
    let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed(Heartbeat::NAME) } else { name.0 };
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#heartbeat_sequence)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes.into_iter().chain(options.into_iter().map(::opentelemetry::KeyValue::from)))
        .start_with_context(tracer.inner(), context);
    Heartbeat { span }
}
