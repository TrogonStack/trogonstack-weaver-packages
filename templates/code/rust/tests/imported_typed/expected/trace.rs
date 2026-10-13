#![allow(deprecated)]
use ::opentelemetry::trace::{TraceContextExt as _, Tracer as _};
use ::std::fmt::Write as _;

#[derive(Clone, Debug)]
pub enum MyappTaskDispatchStartAttr {
    ServerPort(i64),
}

impl From<MyappTaskDispatchStartAttr> for ::opentelemetry::KeyValue {
    fn from(value: MyappTaskDispatchStartAttr) -> Self {
        match value {
            MyappTaskDispatchStartAttr::ServerPort(value) => ::opentelemetry::KeyValue::new("server.port", value),
        }
    }
}
#[derive(Clone, Debug)]
pub enum MyappTaskDispatchAttr {
    ServerPort(i64),
}

impl From<MyappTaskDispatchAttr> for MyappTaskDispatchStartAttr {
    fn from(value: MyappTaskDispatchAttr) -> Self {
        match value {
            MyappTaskDispatchAttr::ServerPort(value) => Self::ServerPort(value),
        }
    }
}
/// Dispatch of a task.
pub struct MyappTaskDispatch<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> MyappTaskDispatch<S> {
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
    pub fn set_attributes(&mut self, attributes: impl IntoIterator<Item = MyappTaskDispatchAttr>) {
        for attribute in attributes { ::opentelemetry::trace::Span::set_attribute(&mut self.span, ::opentelemetry::KeyValue::from(MyappTaskDispatchStartAttr::from(attribute))); }
    }
}

impl MyappTaskDispatch {
    pub const NAME: &'static str = "myapp.task.dispatch";
}

impl Default for MyappTaskDispatch {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_myapp_task_dispatch<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#error_type: upstream::attribute::ErrorType, r#myapp_task_id: super::attribute::MyappTaskId, options: impl IntoIterator<Item = MyappTaskDispatchStartAttr>) -> MyappTaskDispatch<T::Span> {
    let mut name = String::with_capacity("task ".len() + ::std::convert::AsRef::<str>::as_ref(&r#myapp_task_id).len() + " failed with ".len() + ::std::convert::AsRef::<str>::as_ref(&r#error_type).len());
    name.push_str("task ");
    write!(&mut name, "{}", r#myapp_task_id).expect("writing a span name to String cannot fail");
    name.push_str(" failed with ");
    write!(&mut name, "{}", r#error_type).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 2] = [::opentelemetry::KeyValue::from(r#error_type), ::opentelemetry::KeyValue::from(r#myapp_task_id)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes.into_iter().chain(options.into_iter().map(::opentelemetry::KeyValue::from)))
        .start_with_context(tracer.inner(), context);
    MyappTaskDispatch { span }
}
