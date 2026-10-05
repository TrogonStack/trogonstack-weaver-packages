#[derive(Clone, Debug)]
pub struct Meter(opentelemetry::metrics::Meter);
impl Meter {
    pub fn new(provider: &impl opentelemetry::metrics::MeterProvider, scope: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        Self::new_with_scope(provider, opentelemetry::InstrumentationScope::builder(scope).build())
    }
    pub fn new_with_scope(provider: &impl opentelemetry::metrics::MeterProvider, scope: opentelemetry::InstrumentationScope) -> Self {
        Self(provider.meter_with_scope(registry_scope(scope)))
    }
    pub fn inner(&self) -> &opentelemetry::metrics::Meter { &self.0 }
}
impl Default for Meter {
    fn default() -> Self { Self::new(&opentelemetry::metrics::noop::NoopMeterProvider::new(), "") }
}

fn registry_scope(scope: opentelemetry::InstrumentationScope) -> opentelemetry::InstrumentationScope {
    let mut builder = opentelemetry::InstrumentationScope::builder(scope.name().to_owned())
        .with_schema_url(super::SCHEMA_URL)
        .with_attributes(scope.attributes().cloned());
    if let Some(version) = scope.version() { builder = builder.with_version(version.to_owned()); }
    builder.build()
}
