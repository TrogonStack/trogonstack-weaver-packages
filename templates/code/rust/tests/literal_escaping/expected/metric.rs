#![allow(deprecated)]
/// literal \b \f \u1234
#[derive(Clone)]
pub struct DemoLiteralCount {
    instrument: ::opentelemetry::metrics::Counter<u64>,
}

impl DemoLiteralCount {
    pub const NAME: &'static str = "demo.literal.count";
    pub const UNIT: &'static str = "literal\\b\\u1234";
    pub const DESCRIPTION: &'static str = "literal \\b \\f \\u1234";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().u64_counter(Self::NAME)
            .with_description(Self::DESCRIPTION)
            .with_unit(Self::UNIT)
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: u64, r#demo_literal: super::attribute::DemoLiteral) {
        let attributes = [::opentelemetry::KeyValue::from(r#demo_literal)];
        self.instrument.add(value, &attributes);
    }
}

impl Default for DemoLiteralCount {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct DemoLiteralCountObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64>,
}

impl DemoLiteralCountObserver<'_> {
    pub fn observe(&self, value: u64, r#demo_literal: super::attribute::DemoLiteral) {
        let attributes = [::opentelemetry::KeyValue::from(r#demo_literal)];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct DemoLiteralCountObservable {
    _instrument: ::opentelemetry::metrics::ObservableCounter<u64>,
}

impl DemoLiteralCountObservable {
    pub const NAME: &'static str = DemoLiteralCount::NAME;
    pub const UNIT: &'static str = DemoLiteralCount::UNIT;
    pub const DESCRIPTION: &'static str = DemoLiteralCount::DESCRIPTION;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(DemoLiteralCountObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().u64_observable_counter(Self::NAME)
            .with_description(Self::DESCRIPTION)
            .with_unit(Self::UNIT)
            .with_callback(move |observer| callback(DemoLiteralCountObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for DemoLiteralCountObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
