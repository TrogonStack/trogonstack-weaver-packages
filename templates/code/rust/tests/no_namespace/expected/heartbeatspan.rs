#![allow(deprecated)]
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
pub struct Name(String);
impl Name {
    pub fn new(value: impl Into<String>) -> Self { Self(value.into()) }
}
/// A periodic liveness signal.
pub struct Span { span: ::opentelemetry::global::BoxedSpan }
impl Span {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context {
        use ::opentelemetry::trace::TraceContextExt;
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut ::opentelemetry::global::BoxedSpan { &mut self.span }
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
        use ::opentelemetry::trace::Tracer;
        Self { span: super::tracer::Tracer::default().inner().start("heartbeat") }
    }
}

pub fn r#start_(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer, name: Name, r#heartbeat_sequence: super::heartbeatattr::SequenceAttr, options: impl IntoIterator<Item = StartAttr>) -> Span {
    use ::opentelemetry::trace::Tracer;
    let name = if name.0.is_empty() { "heartbeat".to_owned() } else { name.0 };
    let mut attributes = vec![::opentelemetry::KeyValue::from(r#heartbeat_sequence)];
    attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    Span { span }
}
