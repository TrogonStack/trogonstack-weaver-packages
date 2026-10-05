#![allow(deprecated)]
/// literal \b \f \u1234
#[derive(Clone)]
pub struct LiteralCountCounter { instrument: ::opentelemetry::metrics::Counter<u64> }
impl LiteralCountCounter {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().u64_counter("demo.literal.count")
            .with_description("literal \\b \\f \\u1234")
            .with_unit("literal\\b\\u1234")
            .build() }
    }
    pub fn add(&self, value: u64, r#demo_literal: super::demoattr::LiteralAttr) {
        let attributes = vec![::opentelemetry::KeyValue::from(r#demo_literal)];
        self.instrument.add(value, &attributes);
    }
}
impl Default for LiteralCountCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct LiteralCountCounterObserver<'a> { observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64> }
impl LiteralCountCounterObserver<'_> {
    pub fn observe(&self, value: u64, r#demo_literal: super::demoattr::LiteralAttr) {
        let attributes = vec![::opentelemetry::KeyValue::from(r#demo_literal)];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct LiteralCountObservableCounter { _instrument: ::opentelemetry::metrics::ObservableCounter<u64> }
impl LiteralCountObservableCounter {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(LiteralCountCounterObserver<'_>) + Send + Sync + 'static) -> Self {
        Self { _instrument: meter.inner().u64_observable_counter("demo.literal.count")
            .with_description("literal \\b \\f \\u1234")
            .with_unit("literal\\b\\u1234")
            .with_callback(move |observer| callback(LiteralCountCounterObserver { observer }))
            .build() }
    }
}
impl Default for LiteralCountObservableCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
