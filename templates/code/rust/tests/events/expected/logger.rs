use opentelemetry::logs::AnyValue;
use opentelemetry::{Array, Value};

#[derive(Clone, Debug)]
pub struct Logger<L: opentelemetry::logs::Logger = <opentelemetry::logs::NoopLoggerProvider as opentelemetry::logs::LoggerProvider>::Logger>(L);
impl<L: opentelemetry::logs::Logger> Logger<L> {
    pub fn new<P: opentelemetry::logs::LoggerProvider<Logger = L>>(provider: &P, scope: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        Self::new_with_scope(provider, opentelemetry::InstrumentationScope::builder(scope).build())
    }
    pub fn new_with_scope<P: opentelemetry::logs::LoggerProvider<Logger = L>>(provider: &P, scope: opentelemetry::InstrumentationScope) -> Self {
        Self(provider.logger_with_scope(registry_scope(scope)))
    }
    pub fn inner(&self) -> &L { &self.0 }
}
impl Default for Logger {
    fn default() -> Self { Self::new(&opentelemetry::logs::NoopLoggerProvider::new(), "") }
}
pub(crate) fn log_value(value: opentelemetry::Value) -> opentelemetry::logs::AnyValue {
    match value {
        Value::Bool(value) => value.into(),
        Value::I64(value) => value.into(),
        Value::F64(value) => value.into(),
        Value::String(value) => value.into(),
        Value::Array(array) => match array {
            Array::Bool(values) => values.into_iter().collect::<AnyValue>(),
            Array::I64(values) => values.into_iter().collect::<AnyValue>(),
            Array::F64(values) => values.into_iter().collect::<AnyValue>(),
            Array::String(values) => values.into_iter().collect::<AnyValue>(),
            _ => AnyValue::String(array.to_string().into()),
        },
        _ => AnyValue::String(value.to_string().into()),
    }
}

fn registry_scope(scope: opentelemetry::InstrumentationScope) -> opentelemetry::InstrumentationScope {
    let mut builder = opentelemetry::InstrumentationScope::builder(scope.name().to_owned())
        .with_schema_url(super::SCHEMA_URL)
        .with_attributes(scope.attributes().cloned());
    if let Some(version) = scope.version() { builder = builder.with_version(version.to_owned()); }
    builder.build()
}
