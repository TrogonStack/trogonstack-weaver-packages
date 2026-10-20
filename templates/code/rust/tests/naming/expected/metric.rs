#![allow(deprecated)]
/// Synthetic count.
#[derive(Clone)]
pub struct CapsHttp2STATUSCount {
    instrument: ::opentelemetry::metrics::Counter<u64>,
}

impl CapsHttp2STATUSCount {
    pub const NAME: &'static str = "caps.http2_STATUS.count";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().u64_counter(Self::NAME)
            .with_description("Synthetic count.")
            .with_unit("{item}")
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

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(CapsHttp2STATUSCountObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().u64_observable_counter(Self::NAME)
            .with_description("Synthetic count.")
            .with_unit("{item}")
            .with_callback(move |observer| callback(CapsHttp2STATUSCountObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for CapsHttp2STATUSCountObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
