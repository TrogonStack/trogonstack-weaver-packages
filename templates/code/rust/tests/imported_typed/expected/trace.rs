#![allow(deprecated)]
use ::opentelemetry::trace::{TraceContextExt as _, Tracer as _};

mod span_name {
    use ::std::fmt::Write as _;

    pub(super) enum Part {
        Text(&'static str),
        Attribute(&'static str),
        Enum(&'static str),
    }

    pub(super) fn select(attributes: &[::opentelemetry::KeyValue], fallback: &'static str, templates: &[&[Part]]) -> ::std::borrow::Cow<'static, str> {
        for template in templates {
            let Some(capacity) = capacity(attributes, template) else { continue };
            let mut name = String::with_capacity(capacity);
            for part in *template {
                match part {
                    Part::Text(text) => name.push_str(text),
                    Part::Attribute(key) | Part::Enum(key) => {
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
                Part::Enum(key) => match value(attributes, key)? {
                    ::opentelemetry::Value::String(text) if text.as_str() == "_OTHER" => return None,
                    value => length(value)?,
                },
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
    pub fn span_context(&self) -> &::opentelemetry::trace::SpanContext { ::opentelemetry::trace::Span::span_context(&self.span) }
    pub fn is_recording(&self) -> bool { ::opentelemetry::trace::Span::is_recording(&self.span) }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
    pub fn add_event(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>, attributes: impl IntoIterator<Item = MyappTaskDispatchAttr>) {
        ::opentelemetry::trace::Span::add_event(&mut self.span, name, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(MyappTaskDispatchStartAttr::from(attribute))).collect());
    }
    pub fn add_event_with_timestamp(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>, timestamp: ::std::time::SystemTime, attributes: impl IntoIterator<Item = MyappTaskDispatchAttr>) {
        ::opentelemetry::trace::Span::add_event_with_timestamp(&mut self.span, name, timestamp, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(MyappTaskDispatchStartAttr::from(attribute))).collect());
    }
    pub fn add_link(&mut self, span_context: ::opentelemetry::trace::SpanContext, attributes: impl IntoIterator<Item = MyappTaskDispatchAttr>) {
        ::opentelemetry::trace::Span::add_link(&mut self.span, span_context, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(MyappTaskDispatchStartAttr::from(attribute))).collect());
    }
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

/// The span is named from the first of these templates whose attributes are all set, not empty, and not `_OTHER`: `task {myapp.task.id} failed with {error.type}`. Otherwise it is named `myapp.task.dispatch`.
pub fn r#start_myapp_task_dispatch<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#error_type: upstream::attribute::ErrorType, r#myapp_task_id: super::attribute::MyappTaskId, options: impl IntoIterator<Item = MyappTaskDispatchStartAttr>) -> MyappTaskDispatch<T::Span> {
    let attributes: ::std::vec::Vec<::opentelemetry::KeyValue> = [::opentelemetry::KeyValue::from(r#error_type), ::opentelemetry::KeyValue::from(r#myapp_task_id)].into_iter().chain(options.into_iter().map(::opentelemetry::KeyValue::from)).collect();
    let name = span_name::select(&attributes, "myapp.task.dispatch", &[
        &[span_name::Part::Text("task "), span_name::Part::Attribute("myapp.task.id"), span_name::Part::Text(" failed with "), span_name::Part::Enum("error.type")],
    ]);
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    MyappTaskDispatch { span }
}
