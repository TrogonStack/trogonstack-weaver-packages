#![allow(deprecated)]
#[derive(Clone, Debug)]
pub enum ProbeOptionalAttr {
    Optional(super::attribute::ProbeOptional),
}

impl From<ProbeOptionalAttr> for ::opentelemetry::KeyValue {
    fn from(value: ProbeOptionalAttr) -> Self {
        match value {
            ProbeOptionalAttr::Optional(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
/// Measurements with optional numeric attributes.
#[derive(Clone)]
pub struct ProbeOptional {
    instrument: ::opentelemetry::metrics::Counter<u64>,
}

impl ProbeOptional {
    pub const NAME: &'static str = "probe.optional";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().u64_counter(Self::NAME)
            .with_description("Measurements with optional numeric attributes.")
            .with_unit("{measurement}")
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: u64, r#probe_required: super::attribute::ProbeRequired, options: impl IntoIterator<Item = ProbeOptionalAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 2]>::new();
        attributes.extend([::opentelemetry::KeyValue::from(r#probe_required)]);
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.instrument.add(value, &attributes);
    }
}

impl Default for ProbeOptional {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct ProbeOptionalObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64>,
}

impl ProbeOptionalObserver<'_> {
    pub fn observe(&self, value: u64, r#probe_required: super::attribute::ProbeRequired, options: impl IntoIterator<Item = ProbeOptionalAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 2]>::new();
        attributes.extend([::opentelemetry::KeyValue::from(r#probe_required)]);
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct ProbeOptionalObservable {
    _instrument: ::opentelemetry::metrics::ObservableCounter<u64>,
}

impl ProbeOptionalObservable {
    pub const NAME: &'static str = ProbeOptional::NAME;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(ProbeOptionalObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().u64_observable_counter(Self::NAME)
            .with_description("Measurements with optional numeric attributes.")
            .with_unit("{measurement}")
            .with_callback(move |observer| callback(ProbeOptionalObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for ProbeOptionalObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
/// Measurements with a required numeric attribute.
#[derive(Clone)]
pub struct ProbeRequired {
    instrument: ::opentelemetry::metrics::Counter<u64>,
}

impl ProbeRequired {
    pub const NAME: &'static str = "probe.required";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().u64_counter(Self::NAME)
            .with_description("Measurements with a required numeric attribute.")
            .with_unit("{measurement}")
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: u64, r#probe_required: super::attribute::ProbeRequired) {
        let attributes = [::opentelemetry::KeyValue::from(r#probe_required)];
        self.instrument.add(value, &attributes);
    }
}

impl Default for ProbeRequired {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct ProbeRequiredObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64>,
}

impl ProbeRequiredObserver<'_> {
    pub fn observe(&self, value: u64, r#probe_required: super::attribute::ProbeRequired) {
        let attributes = [::opentelemetry::KeyValue::from(r#probe_required)];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct ProbeRequiredObservable {
    _instrument: ::opentelemetry::metrics::ObservableCounter<u64>,
}

impl ProbeRequiredObservable {
    pub const NAME: &'static str = ProbeRequired::NAME;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(ProbeRequiredObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().u64_observable_counter(Self::NAME)
            .with_description("Measurements with a required numeric attribute.")
            .with_unit("{measurement}")
            .with_callback(move |observer| callback(ProbeRequiredObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for ProbeRequiredObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
