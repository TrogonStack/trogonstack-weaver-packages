#![allow(deprecated)]
#[derive(Clone, Debug)]
pub enum MyappTaskFailedAttr {
    ServerPort(i64),
}

impl From<MyappTaskFailedAttr> for ::opentelemetry::KeyValue {
    fn from(value: MyappTaskFailedAttr) -> Self {
        match value {
            MyappTaskFailedAttr::ServerPort(value) => ::opentelemetry::KeyValue::new("server.port", value),
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
    pub fn add(&self, value: u64, r#error_type: upstream::attribute::ErrorType, r#myapp_task_id: super::attribute::MyappTaskId, options: impl IntoIterator<Item = MyappTaskFailedAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 3]>::new();
        attributes.extend([::opentelemetry::KeyValue::from(r#error_type), ::opentelemetry::KeyValue::from(r#myapp_task_id)]);
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
    pub fn observe(&self, value: u64, r#error_type: upstream::attribute::ErrorType, r#myapp_task_id: super::attribute::MyappTaskId, options: impl IntoIterator<Item = MyappTaskFailedAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 3]>::new();
        attributes.extend([::opentelemetry::KeyValue::from(r#error_type), ::opentelemetry::KeyValue::from(r#myapp_task_id)]);
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
