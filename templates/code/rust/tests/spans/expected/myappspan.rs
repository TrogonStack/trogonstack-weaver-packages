#![allow(deprecated)]
use ::opentelemetry::trace::{TraceContextExt, Tracer};
use ::std::fmt::Write;

/// Synthetic escaped name.
pub struct NameApostropheSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> NameApostropheSpan<S> {
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
}

impl Default for NameApostropheSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.name.apostrophe") }
    }
}

pub fn r#start_name_apostrophe<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#worker_host_port: super::workerattr::HostPortAttr) -> NameApostropheSpan<T::Span> {
    let mut name = String::with_capacity("\u{0027}".len() + 20);
    name.push('\u{0027}');
    write!(&mut name, "{}", r#worker_host_port).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    NameApostropheSpan { span }
}
/// A span named from array values.
pub struct NameArraysSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> NameArraysSpan<S> {
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
}

impl Default for NameArraysSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.name.arrays") }
    }
}

pub fn r#start_name_arrays<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#myapp_array_flags: super::myappattr::ArrayFlagsAttr, r#myapp_array_indices: super::myappattr::ArrayIndicesAttr, r#myapp_array_samples: super::myappattr::ArraySamplesAttr, r#myapp_array_tags: super::myappattr::ArrayTagsAttr) -> NameArraysSpan<T::Span> {
    let mut name = String::with_capacity(::std::convert::AsRef::<[::opentelemetry::StringValue]>::as_ref(&r#myapp_array_tags).iter().map(|value| value.as_str().len() + 3).sum::<usize>() + 2 + ":".len() + ::std::convert::AsRef::<[i64]>::as_ref(&r#myapp_array_indices).len() * 21 + 2 + ":".len() + ::std::convert::AsRef::<[f64]>::as_ref(&r#myapp_array_samples).len() * 328 + 2 + ":".len() + ::std::convert::AsRef::<[bool]>::as_ref(&r#myapp_array_flags).len() * 6 + 2);
    write!(&mut name, "{}", r#myapp_array_tags).expect("writing a span name to String cannot fail");
    name.push(':');
    write!(&mut name, "{}", r#myapp_array_indices).expect("writing a span name to String cannot fail");
    name.push(':');
    write!(&mut name, "{}", r#myapp_array_samples).expect("writing a span name to String cannot fail");
    name.push(':');
    write!(&mut name, "{}", r#myapp_array_flags).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 4] = [::opentelemetry::KeyValue::from(r#myapp_array_flags), ::opentelemetry::KeyValue::from(r#myapp_array_indices), ::opentelemetry::KeyValue::from(r#myapp_array_samples), ::opentelemetry::KeyValue::from(r#myapp_array_tags)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    NameArraysSpan { span }
}
/// Synthetic escaped name.
pub struct NameBackslashSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> NameBackslashSpan<S> {
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
}

impl Default for NameBackslashSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.name.backslash") }
    }
}

pub fn r#start_name_backslash<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#worker_host_port: super::workerattr::HostPortAttr) -> NameBackslashSpan<T::Span> {
    let mut name = String::with_capacity("\\".len() + 20);
    name.push('\\');
    write!(&mut name, "{}", r#worker_host_port).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    NameBackslashSpan { span }
}
/// Synthetic escaped name.
pub struct NameControlSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> NameControlSpan<S> {
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
}

impl Default for NameControlSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.name.control") }
    }
}

pub fn r#start_name_control<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#worker_host_port: super::workerattr::HostPortAttr) -> NameControlSpan<T::Span> {
    let mut name = String::with_capacity("\u{0008}".len() + 20);
    name.push('\u{0008}');
    write!(&mut name, "{}", r#worker_host_port).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    NameControlSpan { span }
}
/// A span named from a floating point value.
pub struct NameFloatSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> NameFloatSpan<S> {
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
}

impl Default for NameFloatSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.name.float") }
    }
}

pub fn r#start_name_float<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#myapp_number_value: super::myappattr::NumberValueAttr) -> NameFloatSpan<T::Span> {
    let mut name = String::with_capacity(328);
    write!(&mut name, "{}", r#myapp_number_value).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#myapp_number_value)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    NameFloatSpan { span }
}
#[derive(Clone, Debug, Default)]
pub struct NameStaticName(::std::borrow::Cow<'static, str>);

impl From<&'static str> for NameStaticName {
    fn from(value: &'static str) -> Self { Self(::std::borrow::Cow::Borrowed(value)) }
}

impl From<String> for NameStaticName {
    fn from(value: String) -> Self { Self(::std::borrow::Cow::Owned(value)) }
}

impl From<::std::borrow::Cow<'static, str>> for NameStaticName {
    fn from(value: ::std::borrow::Cow<'static, str>) -> Self { Self(value) }
}
/// A span with an unconstrained caller supplied name.
pub struct NameStaticSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> NameStaticSpan<S> {
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
}

impl Default for NameStaticSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.name.static") }
    }
}

pub fn r#start_name_static<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, name: NameStaticName) -> NameStaticSpan<T::Span> {
    let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed("myapp.name.static") } else { name.0 };
    let attributes: [::opentelemetry::KeyValue; 0] = [];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    NameStaticSpan { span }
}
/// Synthetic escaped name.
pub struct NameUnicodeSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> NameUnicodeSpan<S> {
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
}

impl Default for NameUnicodeSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.name.unicode") }
    }
}

pub fn r#start_name_unicode<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#worker_host_port: super::workerattr::HostPortAttr) -> NameUnicodeSpan<T::Span> {
    let mut name = String::with_capacity("é".len() + 20);
    name.push('é');
    write!(&mut name, "{}", r#worker_host_port).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    NameUnicodeSpan { span }
}
#[derive(Clone, Debug, Default)]
pub struct QueueDrainName(::std::borrow::Cow<'static, str>);

impl From<&'static str> for QueueDrainName {
    fn from(value: &'static str) -> Self { Self(::std::borrow::Cow::Borrowed(value)) }
}

impl From<String> for QueueDrainName {
    fn from(value: String) -> Self { Self(::std::borrow::Cow::Owned(value)) }
}

impl From<::std::borrow::Cow<'static, str>> for QueueDrainName {
    fn from(value: ::std::borrow::Cow<'static, str>) -> Self { Self(value) }
}
/// Removing every task from a queue.
/// Opt-in: record only when the user requests it.
#[deprecated(note = "Drain queues by dispatching their tasks instead.")]
pub struct QueueDrainSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> QueueDrainSpan<S> {
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
}

impl Default for QueueDrainSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.queue.drain") }
    }
}
#[deprecated(note = "Drain queues by dispatching their tasks instead.")]
pub fn r#start_queue_drain<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, name: QueueDrainName) -> QueueDrainSpan<T::Span> {
    let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed("myapp.queue.drain") } else { name.0 };
    let attributes: [::opentelemetry::KeyValue; 0] = [];
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
pub struct TaskDispatchSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

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

pub fn r#start_task_dispatch<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#worker_host_name: super::workerattr::HostNameAttr, r#worker_host_port: super::workerattr::HostPortAttr, options: impl IntoIterator<Item = TaskDispatchStartAttr>) -> TaskDispatchSpan<T::Span> {
    let mut name = String::with_capacity("dispatch ".len() + ::std::convert::AsRef::<str>::as_ref(&r#worker_host_name).len() + ":".len() + 20);
    name.push_str("dispatch ");
    write!(&mut name, "{}", r#worker_host_name).expect("writing a span name to String cannot fail");
    name.push(':');
    write!(&mut name, "{}", r#worker_host_port).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 2] = [::opentelemetry::KeyValue::from(r#worker_host_name), ::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Client)
        .with_attributes(attributes.into_iter().chain(options.into_iter().map(::opentelemetry::KeyValue::from)))
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
pub struct TaskRunName(::std::borrow::Cow<'static, str>);

impl From<&'static str> for TaskRunName {
    fn from(value: &'static str) -> Self { Self(::std::borrow::Cow::Borrowed(value)) }
}

impl From<String> for TaskRunName {
    fn from(value: String) -> Self { Self(::std::borrow::Cow::Owned(value)) }
}

impl From<::std::borrow::Cow<'static, str>> for TaskRunName {
    fn from(value: ::std::borrow::Cow<'static, str>) -> Self { Self(value) }
}
/// One run of a task.
pub struct TaskRunSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> TaskRunSpan<S> {
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
    pub fn set_attributes(&mut self, attributes: impl IntoIterator<Item = TaskRunAttr>) {
        for attribute in attributes { ::opentelemetry::trace::Span::set_attribute(&mut self.span, ::opentelemetry::KeyValue::from(TaskRunStartAttr::from(attribute))); }
    }
}

impl Default for TaskRunSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("myapp.task.run") }
    }
}

pub fn r#start_task_run<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, name: TaskRunName, r#myapp_task_state: super::myappattr::TaskStateAttr, options: impl IntoIterator<Item = TaskRunStartAttr>) -> TaskRunSpan<T::Span> {
    let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed("myapp.task.run") } else { name.0 };
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#myapp_task_state)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes.into_iter().chain(options.into_iter().map(::opentelemetry::KeyValue::from)))
        .start_with_context(tracer.inner(), context);
    TaskRunSpan { span }
}
