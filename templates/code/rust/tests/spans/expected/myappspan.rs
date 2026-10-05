#![allow(deprecated)]
use ::opentelemetry::trace::{TraceContextExt, Tracer};

/// Synthetic escaped name.
pub struct NameApostropheSpan { span: ::opentelemetry::global::BoxedSpan }
impl NameApostropheSpan {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context {
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut ::opentelemetry::global::BoxedSpan { &mut self.span }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
}
impl Default for NameApostropheSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.name.apostrophe") }
    }
}

pub fn r#start_name_apostrophe(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer, r#worker_host_port: super::workerattr::HostPortAttr) -> NameApostropheSpan {
    let mut name = String::new();
    name.push('\u{0027}');
    name.push_str(&::opentelemetry::KeyValue::from(r#worker_host_port.clone()).value.to_string());
    let attributes = vec![::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    NameApostropheSpan { span }
}
/// Synthetic escaped name.
pub struct NameBackslashSpan { span: ::opentelemetry::global::BoxedSpan }
impl NameBackslashSpan {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context {
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut ::opentelemetry::global::BoxedSpan { &mut self.span }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
}
impl Default for NameBackslashSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.name.backslash") }
    }
}

pub fn r#start_name_backslash(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer, r#worker_host_port: super::workerattr::HostPortAttr) -> NameBackslashSpan {
    let mut name = String::new();
    name.push('\\');
    name.push_str(&::opentelemetry::KeyValue::from(r#worker_host_port.clone()).value.to_string());
    let attributes = vec![::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    NameBackslashSpan { span }
}
/// Synthetic escaped name.
pub struct NameControlSpan { span: ::opentelemetry::global::BoxedSpan }
impl NameControlSpan {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context {
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut ::opentelemetry::global::BoxedSpan { &mut self.span }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
}
impl Default for NameControlSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.name.control") }
    }
}

pub fn r#start_name_control(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer, r#worker_host_port: super::workerattr::HostPortAttr) -> NameControlSpan {
    let mut name = String::new();
    name.push('\u{0008}');
    name.push_str(&::opentelemetry::KeyValue::from(r#worker_host_port.clone()).value.to_string());
    let attributes = vec![::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    NameControlSpan { span }
}
/// Synthetic escaped name.
pub struct NameUnicodeSpan { span: ::opentelemetry::global::BoxedSpan }
impl NameUnicodeSpan {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context {
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut ::opentelemetry::global::BoxedSpan { &mut self.span }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
}
impl Default for NameUnicodeSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.name.unicode") }
    }
}

pub fn r#start_name_unicode(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer, r#worker_host_port: super::workerattr::HostPortAttr) -> NameUnicodeSpan {
    let mut name = String::new();
    name.push('é');
    name.push_str(&::opentelemetry::KeyValue::from(r#worker_host_port.clone()).value.to_string());
    let attributes = vec![::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    NameUnicodeSpan { span }
}
#[derive(Clone, Debug, Default)]
pub struct QueueDrainName(String);
impl QueueDrainName {
    pub fn new(value: impl Into<String>) -> Self { Self(value.into()) }
}
/// Removing every task from a queue.
/// Opt-in: record only when the user requests it.
#[deprecated(note = "Drain queues by dispatching their tasks instead.")]
pub struct QueueDrainSpan { span: ::opentelemetry::global::BoxedSpan }
impl QueueDrainSpan {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context {
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut ::opentelemetry::global::BoxedSpan { &mut self.span }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
}
impl Default for QueueDrainSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.queue.drain") }
    }
}
#[deprecated(note = "Drain queues by dispatching their tasks instead.")]
pub fn r#start_queue_drain(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer, name: QueueDrainName) -> QueueDrainSpan {
    let name = if name.0.is_empty() { "myapp.queue.drain".to_owned() } else { name.0 };
    let attributes = vec![];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Consumer)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    QueueDrainSpan { span }
}
#[derive(Clone, Debug)]
pub enum TaskDispatchStartAttr {
    /// Recommended: When the task came from a named queue.
    QueueName(super::myappattr::QueueNameAttr),
    /// Opt-in: the convention records it only when a user asks for it.
    TaskAttempt(super::myappattr::TaskAttemptAttr),
    TaskId(super::myappattr::TaskIdAttr),
}
impl From<TaskDispatchStartAttr> for ::opentelemetry::KeyValue {
    fn from(value: TaskDispatchStartAttr) -> Self {
        match value {
            TaskDispatchStartAttr::QueueName(value) => ::opentelemetry::KeyValue::from(value),
            TaskDispatchStartAttr::TaskAttempt(value) => ::opentelemetry::KeyValue::from(value),
            TaskDispatchStartAttr::TaskId(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
#[derive(Clone, Debug)]
pub enum TaskDispatchAttr {
    QueueName(super::myappattr::QueueNameAttr),
    TaskAttempt(super::myappattr::TaskAttemptAttr),
}
impl From<TaskDispatchAttr> for TaskDispatchStartAttr {
    fn from(value: TaskDispatchAttr) -> Self {
        match value {
            TaskDispatchAttr::QueueName(value) => Self::QueueName(value),
            TaskDispatchAttr::TaskAttempt(value) => Self::TaskAttempt(value),
        }
    }
}
/// The scheduler handing a task to a worker.
///
/// Ends when the worker accepts the task.
pub struct TaskDispatchSpan { span: ::opentelemetry::global::BoxedSpan }
impl TaskDispatchSpan {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context {
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut ::opentelemetry::global::BoxedSpan { &mut self.span }
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

pub fn r#start_task_dispatch(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer, r#worker_host_name: super::workerattr::HostNameAttr, r#worker_host_port: super::workerattr::HostPortAttr, options: impl IntoIterator<Item = TaskDispatchStartAttr>) -> TaskDispatchSpan {
    let mut name = String::new();
    name.push_str("dispatch ");
    name.push_str(&::opentelemetry::KeyValue::from(r#worker_host_name.clone()).value.to_string());
    name.push(':');
    name.push_str(&::opentelemetry::KeyValue::from(r#worker_host_port.clone()).value.to_string());
    let mut attributes = vec![::opentelemetry::KeyValue::from(r#worker_host_name), ::opentelemetry::KeyValue::from(r#worker_host_port)];
    attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Client)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    TaskDispatchSpan { span }
}
#[derive(Clone, Debug)]
pub enum TaskRunStartAttr {
    TaskId(super::myappattr::TaskIdAttr),
}
impl From<TaskRunStartAttr> for ::opentelemetry::KeyValue {
    fn from(value: TaskRunStartAttr) -> Self {
        match value {
            TaskRunStartAttr::TaskId(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
#[derive(Clone, Debug)]
pub enum TaskRunAttr {
    TaskId(super::myappattr::TaskIdAttr),
}
impl From<TaskRunAttr> for TaskRunStartAttr {
    fn from(value: TaskRunAttr) -> Self {
        match value {
            TaskRunAttr::TaskId(value) => Self::TaskId(value),
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct TaskRunName(String);
impl TaskRunName {
    pub fn new(value: impl Into<String>) -> Self { Self(value.into()) }
}
/// One run of a task.
pub struct TaskRunSpan { span: ::opentelemetry::global::BoxedSpan }
impl TaskRunSpan {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context {
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut ::opentelemetry::global::BoxedSpan { &mut self.span }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
    pub fn set_attributes(&mut self, attributes: impl IntoIterator<Item = TaskRunAttr>) {
        for attribute in attributes { ::opentelemetry::trace::Span::set_attribute(&mut self.span, ::opentelemetry::KeyValue::from(TaskRunStartAttr::from(attribute))); }
    }
}
impl Default for TaskRunSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.task.run") }
    }
}

pub fn r#start_task_run(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer, name: TaskRunName, r#myapp_task_state: super::myappattr::TaskStateAttr, options: impl IntoIterator<Item = TaskRunStartAttr>) -> TaskRunSpan {
    let name = if name.0.is_empty() { "myapp.task.run".to_owned() } else { name.0 };
    let mut attributes = vec![::opentelemetry::KeyValue::from(r#myapp_task_state)];
    attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    TaskRunSpan { span }
}
