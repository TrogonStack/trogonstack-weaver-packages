#![allow(deprecated)]
#[derive(Clone, Debug)]
pub enum TaskFailedCounterAttr {
    HttpRequestMethodList(Vec<String>),
}
impl From<TaskFailedCounterAttr> for ::opentelemetry::KeyValue {
    fn from(value: TaskFailedCounterAttr) -> Self {
        match value {
            TaskFailedCounterAttr::HttpRequestMethodList(value) => ::opentelemetry::KeyValue::new("http.request.method_list", ::opentelemetry::Value::Array(::opentelemetry::Array::String(value.into_iter().map(Into::into).collect()))),
        }
    }
}
/// Number of tasks that failed.
#[derive(Clone)]
pub struct TaskFailedCounter { instrument: ::opentelemetry::metrics::Counter<u64> }
impl TaskFailedCounter {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().u64_counter("myapp.task.failed")
            .with_description("Number of tasks that failed.")
            .with_unit("{task}")
            .build() }
    }
    pub fn add(&self, value: u64, r#error_type: String, r#myapp_task_id: super::myappattr::TaskIdAttr, options: impl IntoIterator<Item = TaskFailedCounterAttr>) {
        let mut attributes = vec![::opentelemetry::KeyValue::new("error.type", r#error_type), ::opentelemetry::KeyValue::from(r#myapp_task_id)];
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.instrument.add(value, &attributes);
    }
}
impl Default for TaskFailedCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct TaskFailedCounterObserver<'a> { observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64> }
impl TaskFailedCounterObserver<'_> {
    pub fn observe(&self, value: u64, r#error_type: String, r#myapp_task_id: super::myappattr::TaskIdAttr, options: impl IntoIterator<Item = TaskFailedCounterAttr>) {
        let mut attributes = vec![::opentelemetry::KeyValue::new("error.type", r#error_type), ::opentelemetry::KeyValue::from(r#myapp_task_id)];
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct TaskFailedObservableCounter { _instrument: ::opentelemetry::metrics::ObservableCounter<u64> }
impl TaskFailedObservableCounter {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(TaskFailedCounterObserver<'_>) + Send + Sync + 'static) -> Self {
        Self { _instrument: meter.inner().u64_observable_counter("myapp.task.failed")
            .with_description("Number of tasks that failed.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(TaskFailedCounterObserver { observer }))
            .build() }
    }
}
impl Default for TaskFailedObservableCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
