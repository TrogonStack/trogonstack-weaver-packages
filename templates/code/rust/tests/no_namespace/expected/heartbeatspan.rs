#![allow(deprecated)]
use ::opentelemetry::trace::{TraceContextExt, Tracer};

#[derive(Clone, Debug)]
pub enum StartAttr {
    Source(super::heartbeatattr::SourceAttr),
}
impl From<StartAttr> for ::opentelemetry::KeyValue {
    fn from(value: StartAttr) -> Self {
        match value {
            StartAttr::Source(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
#[derive(Clone, Debug)]
pub enum Attr {
    Source(super::heartbeatattr::SourceAttr),
}
impl From<Attr> for StartAttr {
    fn from(value: Attr) -> Self {
        match value {
            Attr::Source(value) => Self::Source(value),
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Name(::std::borrow::Cow<'static, str>);
impl From<&'static str> for Name {
    fn from(value: &'static str) -> Self { Self(::std::borrow::Cow::Borrowed(value)) }
}
impl From<String> for Name {
    fn from(value: String) -> Self { Self(::std::borrow::Cow::Owned(value)) }
}
impl From<::std::borrow::Cow<'static, str>> for Name {
    fn from(value: ::std::borrow::Cow<'static, str>) -> Self { Self(value) }
}
/// A periodic liveness signal.
pub struct Span<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> { span: S }
impl<S: ::opentelemetry::trace::Span> Span<S> {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context
    where S: Send + Sync + 'static,
    {
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut S { &mut self.span }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
    pub fn set_attributes(&mut self, attributes: impl IntoIterator<Item = Attr>) {
        for attribute in attributes { ::opentelemetry::trace::Span::set_attribute(&mut self.span, ::opentelemetry::KeyValue::from(StartAttr::from(attribute))); }
    }
}
impl Default for Span {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("heartbeat") }
    }
}

pub fn r#start<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, name: Name, r#heartbeat_sequence: super::heartbeatattr::SequenceAttr, options: impl IntoIterator<Item = StartAttr>) -> Span<T::Span> {
    let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed("heartbeat") } else { name.0 };
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#heartbeat_sequence)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes.into_iter().chain(options.into_iter().map(::opentelemetry::KeyValue::from)))
        .start_with_context(tracer.inner(), context);
    Span { span }
}
