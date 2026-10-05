#![allow(deprecated)]





#[derive(Clone, Debug)]
pub enum UpstreamRequestStartAttr {

/// Conditionally required: If the request failed.
    ErrorType(String),


    HttpRequestMethodList(Vec<String>),

}
impl UpstreamRequestStartAttr {
    fn key_value(self) -> ::opentelemetry::KeyValue {
        match self {

            Self::ErrorType(value) => ::opentelemetry::KeyValue::new("error.type", value),

            Self::HttpRequestMethodList(value) => ::opentelemetry::KeyValue::new("http.request.method_list", ::opentelemetry::Value::Array(::opentelemetry::Array::String(value.into_iter().map(Into::into).collect()))),

        }
    }
}


#[derive(Clone, Debug)]
pub enum UpstreamRequestAttr {

    HttpRequestMethodList(Vec<String>),

}
impl From<UpstreamRequestAttr> for UpstreamRequestStartAttr {
    fn from(value: UpstreamRequestAttr) -> Self {
        match value {

            UpstreamRequestAttr::HttpRequestMethodList(value) => Self::HttpRequestMethodList(value),

        }
    }
}


/// A request to an upstream server.
pub struct UpstreamRequestSpan { span: ::opentelemetry::global::BoxedSpan }
impl UpstreamRequestSpan {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context {
        use ::opentelemetry::trace::TraceContextExt;
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut ::opentelemetry::global::BoxedSpan { &mut self.span }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }

    pub fn set_attributes(&mut self, attributes: impl IntoIterator<Item = UpstreamRequestAttr>) {
        for attribute in attributes { ::opentelemetry::trace::Span::set_attribute(&mut self.span, UpstreamRequestStartAttr::from(attribute).key_value()); }
    }

}
impl Default for UpstreamRequestSpan {
    fn default() -> Self {
        use ::opentelemetry::trace::Tracer;
        Self { span: super::tracer::Tracer::default().inner().start("edge.upstream.request") }
    }
}

pub fn r#start_upstream_request(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer, r#server_address: String, r#server_port: i64, options: impl IntoIterator<Item = UpstreamRequestStartAttr>) -> UpstreamRequestSpan {
    use ::opentelemetry::trace::Tracer;

    let mut name = String::new();


    name.push_str(&::opentelemetry::KeyValue::new("server.address", r#server_address.clone()).value.to_string());




    name.push(':');




    name.push_str(&::opentelemetry::KeyValue::new("server.port", r#server_port).value.to_string());



    let mut attributes = vec![::opentelemetry::KeyValue::new("server.address", r#server_address), ::opentelemetry::KeyValue::new("server.port", r#server_port)];

    attributes.extend(options.into_iter().map(UpstreamRequestStartAttr::key_value));

    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Client)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    UpstreamRequestSpan { span }
}
