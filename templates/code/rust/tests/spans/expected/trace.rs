#![allow(deprecated)]
use ::opentelemetry::trace::{TraceContextExt as _, Tracer as _};
use ::std::fmt::Write as _;

/// Synthetic escaped name.
pub struct MyappNameApostrophe<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> MyappNameApostrophe<S> {
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

impl MyappNameApostrophe {
    pub const NAME: &'static str = "myapp.name.apostrophe";
}

impl Default for MyappNameApostrophe {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_myapp_name_apostrophe<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#worker_host_port: super::attribute::WorkerHostPort) -> MyappNameApostrophe<T::Span> {
    let mut name = String::with_capacity("\u{0027}".len() + 20);
    name.push('\u{0027}');
    write!(&mut name, "{}", r#worker_host_port).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    MyappNameApostrophe { span }
}
/// A span named from array values.
pub struct MyappNameArrays<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> MyappNameArrays<S> {
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

impl MyappNameArrays {
    pub const NAME: &'static str = "myapp.name.arrays";
}

impl Default for MyappNameArrays {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_myapp_name_arrays<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#myapp_array_flags: super::attribute::MyappArrayFlags, r#myapp_array_indices: super::attribute::MyappArrayIndices, r#myapp_array_samples: super::attribute::MyappArraySamples, r#myapp_array_tags: super::attribute::MyappArrayTags) -> MyappNameArrays<T::Span> {
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
    MyappNameArrays { span }
}
/// Synthetic escaped name.
pub struct MyappNameBackslash<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> MyappNameBackslash<S> {
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

impl MyappNameBackslash {
    pub const NAME: &'static str = "myapp.name.backslash";
}

impl Default for MyappNameBackslash {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_myapp_name_backslash<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#worker_host_port: super::attribute::WorkerHostPort) -> MyappNameBackslash<T::Span> {
    let mut name = String::with_capacity("\\".len() + 20);
    name.push('\\');
    write!(&mut name, "{}", r#worker_host_port).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    MyappNameBackslash { span }
}
/// Synthetic escaped name.
pub struct MyappNameControl<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> MyappNameControl<S> {
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

impl MyappNameControl {
    pub const NAME: &'static str = "myapp.name.control";
}

impl Default for MyappNameControl {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_myapp_name_control<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#worker_host_port: super::attribute::WorkerHostPort) -> MyappNameControl<T::Span> {
    let mut name = String::with_capacity("\u{0008}".len() + 20);
    name.push('\u{0008}');
    write!(&mut name, "{}", r#worker_host_port).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    MyappNameControl { span }
}
/// A span named from a floating point value.
pub struct MyappNameFloat<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> MyappNameFloat<S> {
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

impl MyappNameFloat {
    pub const NAME: &'static str = "myapp.name.float";
}

impl Default for MyappNameFloat {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_myapp_name_float<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#myapp_number_value: super::attribute::MyappNumberValue) -> MyappNameFloat<T::Span> {
    let mut name = String::with_capacity(328);
    write!(&mut name, "{}", r#myapp_number_value).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#myapp_number_value)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    MyappNameFloat { span }
}
#[derive(Clone, Debug, Default)]
pub struct MyappNameStaticName(::std::borrow::Cow<'static, str>);

impl From<&'static str> for MyappNameStaticName {
    fn from(value: &'static str) -> Self { Self(::std::borrow::Cow::Borrowed(value)) }
}

impl From<String> for MyappNameStaticName {
    fn from(value: String) -> Self { Self(::std::borrow::Cow::Owned(value)) }
}

impl From<::std::borrow::Cow<'static, str>> for MyappNameStaticName {
    fn from(value: ::std::borrow::Cow<'static, str>) -> Self { Self(value) }
}
/// A span with an unconstrained caller supplied name.
pub struct MyappNameStatic<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> MyappNameStatic<S> {
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
    pub fn update_name(&mut self, name: MyappNameStaticName) {
        let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed(MyappNameStatic::NAME) } else { name.0 };
        ::opentelemetry::trace::Span::update_name(&mut self.span, name);
    }
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

impl MyappNameStatic {
    pub const NAME: &'static str = "myapp.name.static";
}

impl Default for MyappNameStatic {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_myapp_name_static<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, name: MyappNameStaticName) -> MyappNameStatic<T::Span> {
    let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed(MyappNameStatic::NAME) } else { name.0 };
    let attributes: [::opentelemetry::KeyValue; 0] = [];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    MyappNameStatic { span }
}
/// Synthetic escaped name.
pub struct MyappNameUnicode<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> MyappNameUnicode<S> {
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

impl MyappNameUnicode {
    pub const NAME: &'static str = "myapp.name.unicode";
}

impl Default for MyappNameUnicode {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_myapp_name_unicode<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#worker_host_port: super::attribute::WorkerHostPort) -> MyappNameUnicode<T::Span> {
    let mut name = String::with_capacity("é".len() + 20);
    name.push('é');
    write!(&mut name, "{}", r#worker_host_port).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#worker_host_port)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    MyappNameUnicode { span }
}
#[derive(Clone, Debug, Default)]
pub struct MyappQueueDrainName(::std::borrow::Cow<'static, str>);

impl From<&'static str> for MyappQueueDrainName {
    fn from(value: &'static str) -> Self { Self(::std::borrow::Cow::Borrowed(value)) }
}

impl From<String> for MyappQueueDrainName {
    fn from(value: String) -> Self { Self(::std::borrow::Cow::Owned(value)) }
}

impl From<::std::borrow::Cow<'static, str>> for MyappQueueDrainName {
    fn from(value: ::std::borrow::Cow<'static, str>) -> Self { Self(value) }
}
/// Removing every task from a queue.
/// Opt-in: record only when the user requests it.
#[deprecated(note = "Drain queues by dispatching their tasks instead.")]
pub struct MyappQueueDrain<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> MyappQueueDrain<S> {
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
    pub fn update_name(&mut self, name: MyappQueueDrainName) {
        let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed(MyappQueueDrain::NAME) } else { name.0 };
        ::opentelemetry::trace::Span::update_name(&mut self.span, name);
    }
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

impl MyappQueueDrain {
    pub const NAME: &'static str = "myapp.queue.drain";
}

impl Default for MyappQueueDrain {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}
#[deprecated(note = "Drain queues by dispatching their tasks instead.")]
pub fn r#start_myapp_queue_drain<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, name: MyappQueueDrainName) -> MyappQueueDrain<T::Span> {
    let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed(MyappQueueDrain::NAME) } else { name.0 };
    let attributes: [::opentelemetry::KeyValue; 0] = [];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Consumer)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    MyappQueueDrain { span }
}
#[derive(Clone, Debug)]
pub enum MyappTaskDispatchStartAttr {
    /// Recommended: When the task came from a named queue.
    QueueName(super::attribute::MyappQueueName),
    /// Opt-in: the convention records it only when a user asks for it.
    TaskAttempt(super::attribute::MyappTaskAttempt),
    TaskId(super::attribute::MyappTaskId),
}

impl From<MyappTaskDispatchStartAttr> for ::opentelemetry::KeyValue {
    fn from(value: MyappTaskDispatchStartAttr) -> Self {
        match value {
            MyappTaskDispatchStartAttr::QueueName(value) => ::opentelemetry::KeyValue::from(value),
            MyappTaskDispatchStartAttr::TaskAttempt(value) => ::opentelemetry::KeyValue::from(value),
            MyappTaskDispatchStartAttr::TaskId(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
#[derive(Clone, Debug)]
pub enum MyappTaskDispatchAttr {
    QueueName(super::attribute::MyappQueueName),
    TaskAttempt(super::attribute::MyappTaskAttempt),
}

impl From<MyappTaskDispatchAttr> for MyappTaskDispatchStartAttr {
    fn from(value: MyappTaskDispatchAttr) -> Self {
        match value {
            MyappTaskDispatchAttr::QueueName(value) => Self::QueueName(value),
            MyappTaskDispatchAttr::TaskAttempt(value) => Self::TaskAttempt(value),
        }
    }
}
/// The scheduler handing a task to a worker.
///
/// Ends when the worker accepts the task.
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

pub fn r#start_myapp_task_dispatch<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#worker_host_name: super::attribute::WorkerHostName, r#worker_host_port: super::attribute::WorkerHostPort, options: impl IntoIterator<Item = MyappTaskDispatchStartAttr>) -> MyappTaskDispatch<T::Span> {
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
    MyappTaskDispatch { span }
}
#[derive(Clone, Debug)]
pub enum MyappTaskRunStartAttr {
    TaskId(super::attribute::MyappTaskId),
}

impl From<MyappTaskRunStartAttr> for ::opentelemetry::KeyValue {
    fn from(value: MyappTaskRunStartAttr) -> Self {
        match value {
            MyappTaskRunStartAttr::TaskId(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
#[derive(Clone, Debug)]
pub enum MyappTaskRunAttr {
    TaskId(super::attribute::MyappTaskId),
}

impl From<MyappTaskRunAttr> for MyappTaskRunStartAttr {
    fn from(value: MyappTaskRunAttr) -> Self {
        match value {
            MyappTaskRunAttr::TaskId(value) => Self::TaskId(value),
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct MyappTaskRunName(::std::borrow::Cow<'static, str>);

impl From<&'static str> for MyappTaskRunName {
    fn from(value: &'static str) -> Self { Self(::std::borrow::Cow::Borrowed(value)) }
}

impl From<String> for MyappTaskRunName {
    fn from(value: String) -> Self { Self(::std::borrow::Cow::Owned(value)) }
}

impl From<::std::borrow::Cow<'static, str>> for MyappTaskRunName {
    fn from(value: ::std::borrow::Cow<'static, str>) -> Self { Self(value) }
}
/// One run of a task.
pub struct MyappTaskRun<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> MyappTaskRun<S> {
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
    pub fn update_name(&mut self, name: MyappTaskRunName) {
        let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed(MyappTaskRun::NAME) } else { name.0 };
        ::opentelemetry::trace::Span::update_name(&mut self.span, name);
    }
    pub fn add_event(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>, attributes: impl IntoIterator<Item = MyappTaskRunAttr>) {
        ::opentelemetry::trace::Span::add_event(&mut self.span, name, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(MyappTaskRunStartAttr::from(attribute))).collect());
    }
    pub fn add_event_with_timestamp(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>, timestamp: ::std::time::SystemTime, attributes: impl IntoIterator<Item = MyappTaskRunAttr>) {
        ::opentelemetry::trace::Span::add_event_with_timestamp(&mut self.span, name, timestamp, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(MyappTaskRunStartAttr::from(attribute))).collect());
    }
    pub fn add_link(&mut self, span_context: ::opentelemetry::trace::SpanContext, attributes: impl IntoIterator<Item = MyappTaskRunAttr>) {
        ::opentelemetry::trace::Span::add_link(&mut self.span, span_context, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(MyappTaskRunStartAttr::from(attribute))).collect());
    }
    pub fn set_attributes(&mut self, attributes: impl IntoIterator<Item = MyappTaskRunAttr>) {
        for attribute in attributes { ::opentelemetry::trace::Span::set_attribute(&mut self.span, ::opentelemetry::KeyValue::from(MyappTaskRunStartAttr::from(attribute))); }
    }
}

impl MyappTaskRun {
    pub const NAME: &'static str = "myapp.task.run";
}

impl Default for MyappTaskRun {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_myapp_task_run<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, name: MyappTaskRunName, r#myapp_task_state: super::attribute::MyappTaskState, options: impl IntoIterator<Item = MyappTaskRunStartAttr>) -> MyappTaskRun<T::Span> {
    let name = if name.0.is_empty() { ::std::borrow::Cow::Borrowed(MyappTaskRun::NAME) } else { name.0 };
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#myapp_task_state)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes.into_iter().chain(options.into_iter().map(::opentelemetry::KeyValue::from)))
        .start_with_context(tracer.inner(), context);
    MyappTaskRun { span }
}
#[derive(Clone, Debug)]
pub enum WorkerTaskDispatchStartAttr {
    /// Recommended: When the task came from a named queue.
    QueueName(super::attribute::MyappQueueName),
    TaskId(super::attribute::MyappTaskId),
}

impl From<WorkerTaskDispatchStartAttr> for ::opentelemetry::KeyValue {
    fn from(value: WorkerTaskDispatchStartAttr) -> Self {
        match value {
            WorkerTaskDispatchStartAttr::QueueName(value) => ::opentelemetry::KeyValue::from(value),
            WorkerTaskDispatchStartAttr::TaskId(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
#[derive(Clone, Debug)]
pub enum WorkerTaskDispatchAttr {
    QueueName(super::attribute::MyappQueueName),
}

impl From<WorkerTaskDispatchAttr> for WorkerTaskDispatchStartAttr {
    fn from(value: WorkerTaskDispatchAttr) -> Self {
        match value {
            WorkerTaskDispatchAttr::QueueName(value) => Self::QueueName(value),
        }
    }
}
/// The scheduler handing a task to a worker.
///
/// Ends when the worker accepts the task.
pub struct WorkerTaskDispatch<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> {
    span: S,
}

impl<S: ::opentelemetry::trace::Span> WorkerTaskDispatch<S> {
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
    pub fn add_event(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>, attributes: impl IntoIterator<Item = WorkerTaskDispatchAttr>) {
        ::opentelemetry::trace::Span::add_event(&mut self.span, name, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(WorkerTaskDispatchStartAttr::from(attribute))).collect());
    }
    pub fn add_event_with_timestamp(&mut self, name: impl Into<::std::borrow::Cow<'static, str>>, timestamp: ::std::time::SystemTime, attributes: impl IntoIterator<Item = WorkerTaskDispatchAttr>) {
        ::opentelemetry::trace::Span::add_event_with_timestamp(&mut self.span, name, timestamp, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(WorkerTaskDispatchStartAttr::from(attribute))).collect());
    }
    pub fn add_link(&mut self, span_context: ::opentelemetry::trace::SpanContext, attributes: impl IntoIterator<Item = WorkerTaskDispatchAttr>) {
        ::opentelemetry::trace::Span::add_link(&mut self.span, span_context, attributes.into_iter().map(|attribute| ::opentelemetry::KeyValue::from(WorkerTaskDispatchStartAttr::from(attribute))).collect());
    }
    pub fn set_attributes(&mut self, attributes: impl IntoIterator<Item = WorkerTaskDispatchAttr>) {
        for attribute in attributes { ::opentelemetry::trace::Span::set_attribute(&mut self.span, ::opentelemetry::KeyValue::from(WorkerTaskDispatchStartAttr::from(attribute))); }
    }
}

impl WorkerTaskDispatch {
    pub const NAME: &'static str = "myapp.task.dispatch";
}

impl Default for WorkerTaskDispatch {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start(Self::NAME) }
    }
}

pub fn r#start_worker_task_dispatch<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#myapp_task_attempt: super::attribute::MyappTaskAttempt, r#worker_host_name: super::attribute::WorkerHostName, r#worker_host_port: super::attribute::WorkerHostPort, options: impl IntoIterator<Item = WorkerTaskDispatchStartAttr>) -> WorkerTaskDispatch<T::Span> {
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
    WorkerTaskDispatch { span }
}
