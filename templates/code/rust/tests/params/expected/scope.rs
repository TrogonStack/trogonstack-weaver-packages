#[derive(Clone, Debug, Default)]
pub struct Scope {
    pub attributes: Vec<::opentelemetry::KeyValue>,
}
impl From<Scope> for ::opentelemetry::InstrumentationScope {
    fn from(value: Scope) -> Self {
        Self::builder("synthetic-billing")
            .with_version("2.3.4")
            .with_schema_url(super::SCHEMA_URL)
            .with_attributes(value.attributes)
            .build()
    }
}
