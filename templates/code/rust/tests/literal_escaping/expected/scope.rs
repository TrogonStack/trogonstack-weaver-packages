use ::std::borrow::Cow;

#[derive(Clone, Debug, Default)]
pub struct Scope {
    pub name: Cow<'static, str>,
    pub version: Option<Cow<'static, str>>,
    pub attributes: Vec<::opentelemetry::KeyValue>,
}
impl From<&'static str> for Scope {
    fn from(name: &'static str) -> Self { Self { name: Cow::Borrowed(name), ..Self::default() } }
}
impl From<String> for Scope {
    fn from(name: String) -> Self { Self { name: Cow::Owned(name), ..Self::default() } }
}
impl From<Cow<'static, str>> for Scope {
    fn from(name: Cow<'static, str>) -> Self { Self { name, ..Self::default() } }
}
impl From<Scope> for ::opentelemetry::InstrumentationScope {
    fn from(value: Scope) -> Self {
        let Scope { name, version, attributes } = value;
        let mut builder = Self::builder(name)
            .with_schema_url(super::SCHEMA_URL)
            .with_attributes(attributes);
        if let Some(version) = version { builder = builder.with_version(version); }
        builder.build()
    }
}
