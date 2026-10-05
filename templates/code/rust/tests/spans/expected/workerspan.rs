#![allow(deprecated)]
#[derive(Clone, Debug)]
pub enum TaskDispatchStartAttr {
/// Recommended: When the task came from a named queue.
    QueueName(super::myappattr::QueueNameAttr),

    TaskId(super::myappattr::TaskIdAttr),
}
impl TaskDispatchStartAttr {
    fn key_value(self) -> ::opentelemetry::KeyValue {
        match self {
            Self::QueueName(value) => value.key_value(),
            Self::TaskId(value) => value.key_value(),
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
pub struct TaskDispatchSpan { span: ::opentelemetry::global::BoxedSpan }
impl TaskDispatchSpan {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context {
        use ::opentelemetry::trace::TraceContextExt;
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut ::opentelemetry::global::BoxedSpan { &mut self.span }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
    pub fn set_attributes(&mut self, attributes: impl IntoIterator<Item = TaskDispatchAttr>) {
        for attribute in attributes { ::opentelemetry::trace::Span::set_attribute(&mut self.span, TaskDispatchStartAttr::from(attribute).key_value()); }
    }
}
impl Default for TaskDispatchSpan {
    fn default() -> Self {
        use ::opentelemetry::trace::Tracer;
        Self { span: super::tracer::Tracer::default().inner().start("myapp.task.dispatch") }
    }
}

pub fn r#start_task_dispatch(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer, r#myapp_task_attempt: super::myappattr::TaskAttemptAttr, r#worker_host_name: super::workerattr::HostNameAttr, r#worker_host_port: super::workerattr::HostPortAttr, options: impl IntoIterator<Item = TaskDispatchStartAttr>) -> TaskDispatchSpan {
    use ::opentelemetry::trace::Tracer;
    let mut name = String::new();
    name.push_str("dispatch ");
    name.push_str(&r#worker_host_name.clone().key_value().value.to_string());
    name.push(':');
    name.push_str(&r#worker_host_port.clone().key_value().value.to_string());
    let mut attributes = vec![r#myapp_task_attempt.key_value(), r#worker_host_name.key_value(), r#worker_host_port.key_value()];
    attributes.extend(options.into_iter().map(TaskDispatchStartAttr::key_value));
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Client)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    TaskDispatchSpan { span }
}
