#[derive(Clone, Debug)]
pub struct Logger<L: opentelemetry::logs::Logger = <opentelemetry::logs::NoopLoggerProvider as opentelemetry::logs::LoggerProvider>::Logger>(L);
impl<L: opentelemetry::logs::Logger> Logger<L> {
    pub fn new<P: opentelemetry::logs::LoggerProvider<Logger = L>>(provider: &P, scope: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        Self(provider.logger_with_scope(opentelemetry::InstrumentationScope::builder(scope).with_schema_url(super::SCHEMA_URL).build()))
    }
    pub fn new_with_scope<P: opentelemetry::logs::LoggerProvider<Logger = L>>(provider: &P, scope: opentelemetry::InstrumentationScope) -> Self {
        Self(provider.logger_with_scope(registry_scope(scope)))
    }
    pub fn inner(&self) -> &L { &self.0 }
}
impl Default for Logger {
    fn default() -> Self { Self::new(&opentelemetry::logs::NoopLoggerProvider::new(), "") }
}

fn registry_scope(scope: opentelemetry::InstrumentationScope) -> opentelemetry::InstrumentationScope {
    if scope.schema_url() == Some(super::SCHEMA_URL) { return scope; }
    let mut builder = opentelemetry::InstrumentationScope::builder(scope.name().to_owned())
        .with_schema_url(super::SCHEMA_URL)
        .with_attributes(scope.attributes().cloned());
    if let Some(version) = scope.version() { builder = builder.with_version(version.to_owned()); }
    builder.build()
}
