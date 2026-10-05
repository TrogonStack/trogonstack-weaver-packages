#![allow(deprecated)]
/// Synthetic count.
#[derive(Clone)]
pub struct Http2STATUSCountCounter { instrument: ::opentelemetry::metrics::Counter<u64> }
impl Http2STATUSCountCounter {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().u64_counter("caps.http2_STATUS.count")
            .with_description("Synthetic count.")
            .with_unit("{item}")
            .build() }
    }
    pub fn add(&self, value: u64, r#caps_http2_status_api_id: super::capsattr::Http2STATUSApiIDAttr, r#caps_mode2_state: super::capsattr::Mode2STATEAttr) {
        let attributes = vec![::opentelemetry::KeyValue::from(r#caps_http2_status_api_id), ::opentelemetry::KeyValue::from(r#caps_mode2_state)];
        self.instrument.add(value, &attributes);
    }
}
impl Default for Http2STATUSCountCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct Http2STATUSCountCounterObserver<'a> { observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64> }
impl Http2STATUSCountCounterObserver<'_> {
    pub fn observe(&self, value: u64, r#caps_http2_status_api_id: super::capsattr::Http2STATUSApiIDAttr, r#caps_mode2_state: super::capsattr::Mode2STATEAttr) {
        let attributes = vec![::opentelemetry::KeyValue::from(r#caps_http2_status_api_id), ::opentelemetry::KeyValue::from(r#caps_mode2_state)];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct Http2STATUSCountObservableCounter { _instrument: ::opentelemetry::metrics::ObservableCounter<u64> }
impl Http2STATUSCountObservableCounter {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(Http2STATUSCountCounterObserver<'_>) + Send + Sync + 'static) -> Self {
        Self { _instrument: meter.inner().u64_observable_counter("caps.http2_STATUS.count")
            .with_description("Synthetic count.")
            .with_unit("{item}")
            .with_callback(move |observer| callback(Http2STATUSCountCounterObserver { observer }))
            .build() }
    }
}
impl Default for Http2STATUSCountObservableCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
