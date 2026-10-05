#![allow(deprecated)]
/// Number of authentication attempts.
#[derive(Clone)]
pub struct AttemptsCounter { instrument: ::opentelemetry::metrics::Counter<u64> }
impl AttemptsCounter {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().u64_counter("auth.attempts")
            .with_description("Number of authentication attempts.")
            .with_unit("{attempt}")
            .build() }
    }
    pub fn add(&self, value: u64, r#auth_method: super::authattr::MethodAttr, r#auth_success: super::authattr::SuccessAttr) {
        let attributes = vec![::opentelemetry::KeyValue::from(r#auth_method), ::opentelemetry::KeyValue::from(r#auth_success)];
        self.instrument.add(value, &attributes);
    }
}
impl Default for AttemptsCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct AttemptsCounterObserver<'a> { observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64> }
impl AttemptsCounterObserver<'_> {
    pub fn observe(&self, value: u64, r#auth_method: super::authattr::MethodAttr, r#auth_success: super::authattr::SuccessAttr) {
        let attributes = vec![::opentelemetry::KeyValue::from(r#auth_method), ::opentelemetry::KeyValue::from(r#auth_success)];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct AttemptsObservableCounter { _instrument: ::opentelemetry::metrics::ObservableCounter<u64> }
impl AttemptsObservableCounter {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(AttemptsCounterObserver<'_>) + Send + Sync + 'static) -> Self {
        Self { _instrument: meter.inner().u64_observable_counter("auth.attempts")
            .with_description("Number of authentication attempts.")
            .with_unit("{attempt}")
            .with_callback(move |observer| callback(AttemptsCounterObserver { observer }))
            .build() }
    }
}
impl Default for AttemptsObservableCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
