#![allow(deprecated)]
/// Synthetic count.
#[derive(Clone)]
pub struct CapsHttp2STATUSCount {
    instrument: ::opentelemetry::metrics::Counter<u64>,
}

impl CapsHttp2STATUSCount {
    pub const NAME: &'static str = "caps.http2_STATUS.count";
    pub const UNIT: &'static str = "{item}";
    pub const DESCRIPTION: &'static str = "Synthetic count.";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().u64_counter(Self::NAME)
            .with_description(Self::DESCRIPTION)
            .with_unit(Self::UNIT)
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: u64, r#caps_http2_status_api_id: super::attribute::CapsHttp2STATUSApiID, r#caps_mode2_state: super::attribute::CapsMode2STATE) {
        let attributes = [::opentelemetry::KeyValue::from(r#caps_http2_status_api_id), ::opentelemetry::KeyValue::from(r#caps_mode2_state)];
        self.instrument.add(value, &attributes);
    }
}

impl Default for CapsHttp2STATUSCount {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct CapsHttp2STATUSCountObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64>,
}

impl CapsHttp2STATUSCountObserver<'_> {
    pub fn observe(&self, value: u64, r#caps_http2_status_api_id: super::attribute::CapsHttp2STATUSApiID, r#caps_mode2_state: super::attribute::CapsMode2STATE) {
        let attributes = [::opentelemetry::KeyValue::from(r#caps_http2_status_api_id), ::opentelemetry::KeyValue::from(r#caps_mode2_state)];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct CapsHttp2STATUSCountObservable {
    _instrument: ::opentelemetry::metrics::ObservableCounter<u64>,
}

impl CapsHttp2STATUSCountObservable {
    pub const NAME: &'static str = CapsHttp2STATUSCount::NAME;
    pub const UNIT: &'static str = CapsHttp2STATUSCount::UNIT;
    pub const DESCRIPTION: &'static str = CapsHttp2STATUSCount::DESCRIPTION;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(CapsHttp2STATUSCountObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().u64_observable_counter(Self::NAME)
            .with_description(Self::DESCRIPTION)
            .with_unit(Self::UNIT)
            .with_callback(move |observer| callback(CapsHttp2STATUSCountObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for CapsHttp2STATUSCountObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
