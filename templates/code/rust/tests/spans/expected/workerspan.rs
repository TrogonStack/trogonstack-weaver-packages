#![allow(deprecated)]
use ::opentelemetry::trace::{TraceContextExt, Tracer};
use ::std::fmt::Write;

#[derive(Clone, Debug)]
pub enum TaskDispatchStartAttr {
    /// Recommended: When the task came from a named queue.
    QueueName(super::myappattr::QueueNameAttr),
    TaskId(super::myappattr::TaskIdAttr),
}
impl From<TaskDispatchStartAttr> for ::opentelemetry::KeyValue {
    fn from(value: TaskDispatchStartAttr) -> Self {
        match value {
            TaskDispatchStartAttr::QueueName(value) => ::opentelemetry::KeyValue::from(value),
            TaskDispatchStartAttr::TaskId(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
#[derive(Clone, Debug)]
pub enum TaskDispatchAttr {
    QueueName(super::myappattr::QueueNameAttr),
}
impl From<TaskDispatchAttr> for TaskDispatchStartAttr {
    fn from(value: TaskDispatchAttr) -> Self {
        match value {
            TaskDispatchAttr::QueueName(value) => Self::QueueName(value),
        }
    }
}
/// The scheduler handing a task to a worker.
///
/// Ends when the worker accepts the task.
pub struct TaskDispatchSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> { span: S }
impl<S: ::opentelemetry::trace::Span> TaskDispatchSpan<S> {
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
    pub fn set_attributes(&mut self, attributes: impl IntoIterator<Item = TaskDispatchAttr>) {
        for attribute in attributes { ::opentelemetry::trace::Span::set_attribute(&mut self.span, ::opentelemetry::KeyValue::from(TaskDispatchStartAttr::from(attribute))); }
    }
}
impl Default for TaskDispatchSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.task.dispatch") }
    }
}

pub fn r#start_task_dispatch<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#myapp_task_attempt: super::myappattr::TaskAttemptAttr, r#worker_host_name: super::workerattr::HostNameAttr, r#worker_host_port: super::workerattr::HostPortAttr, options: impl IntoIterator<Item = TaskDispatchStartAttr>) -> TaskDispatchSpan<T::Span> {
    let mut name = String::with_capacity("dispatch ".len() + ::std::convert::AsRef::<str>::as_ref(&r#worker_host_name).len() + ":".len() + 20);
    name.push_str("dispatch ");
    write!(&mut name, "{}", r#worker_host_name).expect("writing a span name to String cannot fail");
    name.push(':');
    write!(&mut name, "{}", r#worker_host_port).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 3] = [::opentelemetry::KeyValue::from(r#myapp_task_attempt), ::opentelemetry::KeyValue::from(r#worker_host_name), ::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Client)
        .with_attributes(attributes.into_iter().chain(options.into_iter().map(::opentelemetry::KeyValue::from)))
        .start_with_context(tracer.inner(), context);
    TaskDispatchSpan { span }
}
