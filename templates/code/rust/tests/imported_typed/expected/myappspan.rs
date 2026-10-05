#![allow(deprecated)]





#[derive(Clone, Debug)]
pub enum TaskDispatchStartAttr {


    ServerPort(i64),

}
impl TaskDispatchStartAttr {
    fn key_value(self) -> ::opentelemetry::KeyValue {
        match self {

            Self::ServerPort(value) => ::opentelemetry::KeyValue::new("server.port", value),

        }
    }
}


#[derive(Clone, Debug)]
pub enum TaskDispatchAttr {

    ServerPort(i64),

}
impl From<TaskDispatchAttr> for TaskDispatchStartAttr {
    fn from(value: TaskDispatchAttr) -> Self {
        match value {

            TaskDispatchAttr::ServerPort(value) => Self::ServerPort(value),

        }
    }
}


/// Dispatch of a task.
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

pub fn r#start_task_dispatch(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer, r#error_type: upstream::errorattr::TypeAttr, r#myapp_task_id: super::myappattr::TaskIdAttr, options: impl IntoIterator<Item = TaskDispatchStartAttr>) -> TaskDispatchSpan {
    use ::opentelemetry::trace::Tracer;

    let mut name = String::new();



    name.push_str("task ");




    name.push_str(&r#myapp_task_id.clone().key_value().value.to_string());




    name.push_str(" failed with ");




    name.push_str(&r#error_type.key_value().value.to_string());



    let mut attributes = vec![r#error_type.key_value(), r#myapp_task_id.key_value()];

    attributes.extend(options.into_iter().map(TaskDispatchStartAttr::key_value));

    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    TaskDispatchSpan { span }
}
