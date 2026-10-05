#[derive(Clone, Debug, Default)]
pub struct Scope {
    pub attributes: Vec<::opentelemetry::KeyValue>,
}

impl From<Scope> for ::opentelemetry::InstrumentationScope {
    fn from(value: Scope) -> Self {
        Self::builder("literal \\b \\f \\u1234 quote\" double\\\\slash")
            .with_version("controls \u{0008} \u{000c} \u{0001} literal\\b quote\"")
            .with_schema_url(super::SCHEMA_URL)
            .with_attributes(value.attributes)
            .build()
    }
}
