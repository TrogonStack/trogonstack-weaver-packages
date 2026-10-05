use super::scope::Scope;

pub struct Tracer<T: opentelemetry::trace::Tracer = opentelemetry::trace::noop::NoopTracer>(T);
impl<T: opentelemetry::trace::Tracer> Tracer<T> {
    pub fn new<P: opentelemetry::trace::TracerProvider<Tracer = T>>(provider: &P, scope: impl Into<Scope>) -> Self {
        let scope: Scope = scope.into();
        let tracer = provider.tracer_with_scope(scope.into());
        Self(tracer)
    }
    pub fn inner(&self) -> &T { &self.0 }
}
impl Default for Tracer {
    fn default() -> Self { Self::new(&opentelemetry::trace::noop::NoopTracerProvider::new(), Scope::default()) }
}
