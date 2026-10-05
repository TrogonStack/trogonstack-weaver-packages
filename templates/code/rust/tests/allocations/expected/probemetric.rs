#![allow(deprecated)]
#[derive(Clone, Debug)]
pub enum OptionalCounterAttr {
    Optional(super::probeattr::OptionalAttr),
}

impl From<OptionalCounterAttr> for ::opentelemetry::KeyValue {
    fn from(value: OptionalCounterAttr) -> Self {
        match value {
            OptionalCounterAttr::Optional(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
/// Measurements with optional numeric attributes.
#[derive(Clone)]
pub struct OptionalCounter {
    instrument: ::opentelemetry::metrics::Counter<u64>,
}

impl OptionalCounter {
    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().u64_counter("probe.optional")
            .with_description("Measurements with optional numeric attributes.")
            .with_unit("{measurement}")
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: u64, r#probe_required: super::probeattr::RequiredAttr, options: impl IntoIterator<Item = OptionalCounterAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 2]>::new();
        attributes.extend([::opentelemetry::KeyValue::from(r#probe_required)]);
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.instrument.add(value, &attributes);
    }
}

impl Default for OptionalCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct OptionalCounterObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64>,
}

impl OptionalCounterObserver<'_> {
    pub fn observe(&self, value: u64, r#probe_required: super::probeattr::RequiredAttr, options: impl IntoIterator<Item = OptionalCounterAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 2]>::new();
        attributes.extend([::opentelemetry::KeyValue::from(r#probe_required)]);
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct OptionalObservableCounter {
    _instrument: ::opentelemetry::metrics::ObservableCounter<u64>,
}

impl OptionalObservableCounter {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(OptionalCounterObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().u64_observable_counter("probe.optional")
            .with_description("Measurements with optional numeric attributes.")
            .with_unit("{measurement}")
            .with_callback(move |observer| callback(OptionalCounterObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for OptionalObservableCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
/// Measurements with a required numeric attribute.
#[derive(Clone)]
pub struct RequiredCounter {
    instrument: ::opentelemetry::metrics::Counter<u64>,
}

impl RequiredCounter {
    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().u64_counter("probe.required")
            .with_description("Measurements with a required numeric attribute.")
            .with_unit("{measurement}")
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: u64, r#probe_required: super::probeattr::RequiredAttr) {
        let attributes = [::opentelemetry::KeyValue::from(r#probe_required)];
        self.instrument.add(value, &attributes);
    }
}

impl Default for RequiredCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct RequiredCounterObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64>,
}

impl RequiredCounterObserver<'_> {
    pub fn observe(&self, value: u64, r#probe_required: super::probeattr::RequiredAttr) {
        let attributes = [::opentelemetry::KeyValue::from(r#probe_required)];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct RequiredObservableCounter {
    _instrument: ::opentelemetry::metrics::ObservableCounter<u64>,
}

impl RequiredObservableCounter {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(RequiredCounterObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().u64_observable_counter("probe.required")
            .with_description("Measurements with a required numeric attribute.")
            .with_unit("{measurement}")
            .with_callback(move |observer| callback(RequiredCounterObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for RequiredObservableCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
