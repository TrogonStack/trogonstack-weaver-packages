#![allow(deprecated)]
#[derive(Clone, Debug)]
pub enum EdgeUpstreamDurationAttr {
    ServerPort(i64),
}

impl From<EdgeUpstreamDurationAttr> for ::opentelemetry::KeyValue {
    fn from(value: EdgeUpstreamDurationAttr) -> Self {
        match value {
            EdgeUpstreamDurationAttr::ServerPort(value) => ::opentelemetry::KeyValue::new("server.port", value),
        }
    }
}
/// Time spent waiting on an upstream server.
#[derive(Clone)]
pub struct EdgeUpstreamDuration {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl EdgeUpstreamDuration {
    pub const NAME: &'static str = "edge.upstream.duration";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram(Self::NAME)
            .with_description("Time spent waiting on an upstream server.")
            .with_unit("s")
            .with_boundaries(vec![0.005_f64, 0.01_f64, 0.025_f64, 0.05_f64, 0.075_f64, 0.1_f64, 0.25_f64, 0.5_f64, 0.75_f64, 1_f64, 2.5_f64, 5_f64, 7.5_f64, 10_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#server_address: ::opentelemetry::StringValue, options: impl IntoIterator<Item = EdgeUpstreamDurationAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 2]>::new();
        attributes.extend([::opentelemetry::KeyValue::new("server.address", r#server_address)]);
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.instrument.record(value, &attributes);
    }
}

impl Default for EdgeUpstreamDuration {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
#[derive(Clone, Debug)]
pub enum MyappTaskFailedAttr {
    HttpRequestMethodList(Vec<::opentelemetry::StringValue>),
}

impl From<MyappTaskFailedAttr> for ::opentelemetry::KeyValue {
    fn from(value: MyappTaskFailedAttr) -> Self {
        match value {
            MyappTaskFailedAttr::HttpRequestMethodList(value) => ::opentelemetry::KeyValue::new("http.request.method_list", ::opentelemetry::Value::Array(::opentelemetry::Array::String(value))),
        }
    }
}
/// Number of tasks that failed.
#[derive(Clone)]
pub struct MyappTaskFailed {
    instrument: ::opentelemetry::metrics::Counter<u64>,
}

impl MyappTaskFailed {
    pub const NAME: &'static str = "myapp.task.failed";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().u64_counter(Self::NAME)
            .with_description("Number of tasks that failed.")
            .with_unit("{task}")
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: u64, r#error_type: ::opentelemetry::StringValue, r#myapp_task_id: super::attribute::MyappTaskId, options: impl IntoIterator<Item = MyappTaskFailedAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 3]>::new();
        attributes.extend([::opentelemetry::KeyValue::new("error.type", r#error_type), ::opentelemetry::KeyValue::from(r#myapp_task_id)]);
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.instrument.add(value, &attributes);
    }
}

impl Default for MyappTaskFailed {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct MyappTaskFailedObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64>,
}

impl MyappTaskFailedObserver<'_> {
    pub fn observe(&self, value: u64, r#error_type: ::opentelemetry::StringValue, r#myapp_task_id: super::attribute::MyappTaskId, options: impl IntoIterator<Item = MyappTaskFailedAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 3]>::new();
        attributes.extend([::opentelemetry::KeyValue::new("error.type", r#error_type), ::opentelemetry::KeyValue::from(r#myapp_task_id)]);
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct MyappTaskFailedObservable {
    _instrument: ::opentelemetry::metrics::ObservableCounter<u64>,
}

impl MyappTaskFailedObservable {
    pub const NAME: &'static str = MyappTaskFailed::NAME;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(MyappTaskFailedObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().u64_observable_counter(Self::NAME)
            .with_description("Number of tasks that failed.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(MyappTaskFailedObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for MyappTaskFailedObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
