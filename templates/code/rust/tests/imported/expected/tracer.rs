pub struct Tracer(opentelemetry::global::BoxedTracer);
impl Tracer {
    pub fn new<P: opentelemetry::trace::TracerProvider>(provider: &P, scope: impl Into<std::borrow::Cow<'static, str>>) -> Self
    where
        P::Tracer: Send + Sync + 'static,
        <P::Tracer as opentelemetry::trace::Tracer>::Span: Send + Sync + 'static,
    {
        Self::new_with_scope(provider, opentelemetry::InstrumentationScope::builder(scope).build())
    }
    pub fn new_with_scope<P: opentelemetry::trace::TracerProvider>(provider: &P, scope: opentelemetry::InstrumentationScope) -> Self
    where
        P::Tracer: Send + Sync + 'static,
        <P::Tracer as opentelemetry::trace::Tracer>::Span: Send + Sync + 'static,
    {
        Self(opentelemetry::global::BoxedTracer::new(Box::new(provider.tracer_with_scope(registry_scope(scope)))))
    }
    pub fn inner(&self) -> &opentelemetry::global::BoxedTracer { &self.0 }
}
impl Default for Tracer {
    fn default() -> Self { Self::new(&opentelemetry::trace::noop::NoopTracerProvider::new(), "") }
}

fn registry_scope(scope: opentelemetry::InstrumentationScope) -> opentelemetry::InstrumentationScope {
    let mut builder = opentelemetry::InstrumentationScope::builder(scope.name().to_owned())
        .with_schema_url(super::SCHEMA_URL)
        .with_attributes(scope.attributes().cloned());
    if let Some(version) = scope.version() { builder = builder.with_version(version.to_owned()); }
    builder.build()
}
