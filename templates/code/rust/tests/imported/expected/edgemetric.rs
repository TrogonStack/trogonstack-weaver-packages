#![allow(deprecated)]








#[derive(Clone, Debug)]
pub enum UpstreamDurationHistogramAttr {


    ServerPort(i64),

}
impl UpstreamDurationHistogramAttr {
    fn key_value(self) -> ::opentelemetry::KeyValue {
        match self {

            Self::ServerPort(value) => ::opentelemetry::KeyValue::new("server.port", value),

        }
    }
}

/// Time spent waiting on an upstream server.
#[derive(Clone)]
pub struct UpstreamDurationHistogram { instrument: ::opentelemetry::metrics::Histogram<f64> }
impl UpstreamDurationHistogram {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().f64_histogram("edge.upstream.duration")
            .with_description("Time spent waiting on an upstream server.")
            .with_unit("s")

            .with_boundaries(vec![0.005_f64, 0.01_f64, 0.025_f64, 0.05_f64, 0.075_f64, 0.1_f64, 0.25_f64, 0.5_f64, 0.75_f64, 1_f64, 2.5_f64, 5_f64, 7.5_f64, 10_f64])

            .build() }
    }
    pub fn record(&self, value: f64, r#server_address: String, options: impl IntoIterator<Item = UpstreamDurationHistogramAttr>) {
        let mut attributes = vec![::opentelemetry::KeyValue::new("server.address", r#server_address)];

        attributes.extend(options.into_iter().map(UpstreamDurationHistogramAttr::key_value));

        self.instrument.record(value, &attributes);
    }
}
impl Default for UpstreamDurationHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
