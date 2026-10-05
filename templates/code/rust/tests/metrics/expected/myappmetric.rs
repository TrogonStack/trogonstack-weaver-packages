#![allow(deprecated)]
/// Number of tasks waiting in the queue.
#[deprecated(note = "Replaced by `myapp.task.active`.")]
#[derive(Clone)]
pub struct QueueDepthGauge { instrument: ::opentelemetry::metrics::Gauge<i64> }
impl QueueDepthGauge {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().i64_gauge("myapp.queue.depth")
            .with_description("Number of tasks waiting in the queue.")
            .with_unit("{task}")
            .build() }
    }
    pub fn record(&self, value: i64) {
        let attributes = vec![];
        self.instrument.record(value, &attributes);
    }
}
impl Default for QueueDepthGauge {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct QueueDepthGaugeObserver<'a> { observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<i64> }
impl QueueDepthGaugeObserver<'_> {
    pub fn observe(&self, value: i64) {
        let attributes = vec![];
        self.observer.observe(value, &attributes);
    }
}
#[deprecated(note = "Replaced by `myapp.task.active`.")]
#[derive(Clone)]
pub struct QueueDepthObservableGauge { _instrument: ::opentelemetry::metrics::ObservableGauge<i64> }
impl QueueDepthObservableGauge {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(QueueDepthGaugeObserver<'_>) + Send + Sync + 'static) -> Self {
        Self { _instrument: meter.inner().i64_observable_gauge("myapp.queue.depth")
            .with_description("Number of tasks waiting in the queue.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(QueueDepthGaugeObserver { observer }))
            .build() }
    }
}
impl Default for QueueDepthObservableGauge {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
/// Number of tasks currently running.
#[derive(Clone)]
pub struct TaskActiveUpDownCounter { instrument: ::opentelemetry::metrics::UpDownCounter<i64> }
impl TaskActiveUpDownCounter {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().i64_up_down_counter("myapp.task.active")
            .with_description("Number of tasks currently running.")
            .with_unit("{task}")
            .build() }
    }
    pub fn add(&self, value: i64, r#myapp_task_state: super::myappattr::TaskStateAttr) {
        let attributes = vec![r#myapp_task_state.key_value()];
        self.instrument.add(value, &attributes);
    }
}
impl Default for TaskActiveUpDownCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct TaskActiveUpDownCounterObserver<'a> { observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<i64> }
impl TaskActiveUpDownCounterObserver<'_> {
    pub fn observe(&self, value: i64, r#myapp_task_state: super::myappattr::TaskStateAttr) {
        let attributes = vec![r#myapp_task_state.key_value()];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct TaskActiveObservableUpDownCounter { _instrument: ::opentelemetry::metrics::ObservableUpDownCounter<i64> }
impl TaskActiveObservableUpDownCounter {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(TaskActiveUpDownCounterObserver<'_>) + Send + Sync + 'static) -> Self {
        Self { _instrument: meter.inner().i64_observable_up_down_counter("myapp.task.active")
            .with_description("Number of tasks currently running.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(TaskActiveUpDownCounterObserver { observer }))
            .build() }
    }
}
impl Default for TaskActiveObservableUpDownCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
#[derive(Clone, Debug)]
pub enum TaskDurationHistogramAttr {
/// Opt-in: the convention records it only when a user asks for it.
    Method(super::authattr::MethodAttr),
/// Conditionally required: If the task reached a final state.
    TaskState(super::myappattr::TaskStateAttr),
}
impl TaskDurationHistogramAttr {
    fn key_value(self) -> ::opentelemetry::KeyValue {
        match self {
            Self::Method(value) => value.key_value(),
            Self::TaskState(value) => value.key_value(),
        }
    }
}
/// Time a task took from start to finish.
///
/// Measured by the worker that ran the task.
#[derive(Clone)]
pub struct TaskDurationHistogram { instrument: ::opentelemetry::metrics::Histogram<f64> }
impl TaskDurationHistogram {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().f64_histogram("myapp.task.duration")
            .with_description("Time a task took from start to finish.")
            .with_unit("s")
            .with_boundaries(vec![0_f64, 0.005_f64, 0.01_f64, 0.1_f64, 1_f64, 10_f64])
            .build() }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::myappattr::TaskIdAttr, options: impl IntoIterator<Item = TaskDurationHistogramAttr>) {
        let mut attributes = vec![r#myapp_task_id.key_value()];
        attributes.extend(options.into_iter().map(TaskDurationHistogramAttr::key_value));
        self.instrument.record(value, &attributes);
    }
}
impl Default for TaskDurationHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
/// Fractional active task adjustment.
#[derive(Clone)]
pub struct TaskFloatActiveUpDownCounter { instrument: ::opentelemetry::metrics::UpDownCounter<f64> }
impl TaskFloatActiveUpDownCounter {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().f64_up_down_counter("myapp.task.float.active")
            .with_description("Fractional active task adjustment.")
            .with_unit("{task}")
            .build() }
    }
    pub fn add(&self, value: f64) {
        let attributes = vec![];
        self.instrument.add(value, &attributes);
    }
}
impl Default for TaskFloatActiveUpDownCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct TaskFloatActiveUpDownCounterObserver<'a> { observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<f64> }
impl TaskFloatActiveUpDownCounterObserver<'_> {
    pub fn observe(&self, value: f64) {
        let attributes = vec![];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct TaskFloatActiveObservableUpDownCounter { _instrument: ::opentelemetry::metrics::ObservableUpDownCounter<f64> }
impl TaskFloatActiveObservableUpDownCounter {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(TaskFloatActiveUpDownCounterObserver<'_>) + Send + Sync + 'static) -> Self {
        Self { _instrument: meter.inner().f64_observable_up_down_counter("myapp.task.float.active")
            .with_description("Fractional active task adjustment.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(TaskFloatActiveUpDownCounterObserver { observer }))
            .build() }
    }
}
impl Default for TaskFloatActiveObservableUpDownCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
/// Fractional queue depth.
#[derive(Clone)]
pub struct TaskFloatDepthGauge { instrument: ::opentelemetry::metrics::Gauge<f64> }
impl TaskFloatDepthGauge {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().f64_gauge("myapp.task.float.depth")
            .with_description("Fractional queue depth.")
            .with_unit("{task}")
            .build() }
    }
    pub fn record(&self, value: f64) {
        let attributes = vec![];
        self.instrument.record(value, &attributes);
    }
}
impl Default for TaskFloatDepthGauge {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct TaskFloatDepthGaugeObserver<'a> { observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<f64> }
impl TaskFloatDepthGaugeObserver<'_> {
    pub fn observe(&self, value: f64) {
        let attributes = vec![];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct TaskFloatDepthObservableGauge { _instrument: ::opentelemetry::metrics::ObservableGauge<f64> }
impl TaskFloatDepthObservableGauge {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(TaskFloatDepthGaugeObserver<'_>) + Send + Sync + 'static) -> Self {
        Self { _instrument: meter.inner().f64_observable_gauge("myapp.task.float.depth")
            .with_description("Fractional queue depth.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(TaskFloatDepthGaugeObserver { observer }))
            .build() }
    }
}
impl Default for TaskFloatDepthObservableGauge {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
/// Fractional tasks started.
#[derive(Clone)]
pub struct TaskFloatStartedCounter { instrument: ::opentelemetry::metrics::Counter<f64> }
impl TaskFloatStartedCounter {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().f64_counter("myapp.task.float.started")
            .with_description("Fractional tasks started.")
            .with_unit("{task}")
            .build() }
    }
    pub fn add(&self, value: f64) {
        let attributes = vec![];
        self.instrument.add(value, &attributes);
    }
}
impl Default for TaskFloatStartedCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct TaskFloatStartedCounterObserver<'a> { observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<f64> }
impl TaskFloatStartedCounterObserver<'_> {
    pub fn observe(&self, value: f64) {
        let attributes = vec![];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct TaskFloatStartedObservableCounter { _instrument: ::opentelemetry::metrics::ObservableCounter<f64> }
impl TaskFloatStartedObservableCounter {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(TaskFloatStartedCounterObserver<'_>) + Send + Sync + 'static) -> Self {
        Self { _instrument: meter.inner().f64_observable_counter("myapp.task.float.started")
            .with_description("Fractional tasks started.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(TaskFloatStartedCounterObserver { observer }))
            .build() }
    }
}
impl Default for TaskFloatStartedObservableCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
/// Task payload size.
#[derive(Clone)]
pub struct TaskPayloadHistogram { instrument: ::opentelemetry::metrics::Histogram<u64> }
impl TaskPayloadHistogram {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().u64_histogram("myapp.task.payload")
            .with_description("Task payload size.")
            .with_unit("By")
            .with_boundaries(vec![128_f64, 256_f64, 1024_f64])
            .build() }
    }
    pub fn record(&self, value: u64) {
        let attributes = vec![];
        self.instrument.record(value, &attributes);
    }
}
impl Default for TaskPayloadHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
#[derive(Clone, Debug)]
pub enum TaskRetryDurationHistogramAttr {
/// Opt-in: the convention records it only when a user asks for it.
    Method(super::authattr::MethodAttr),
}
impl TaskRetryDurationHistogramAttr {
    fn key_value(self) -> ::opentelemetry::KeyValue {
        match self {
            Self::Method(value) => value.key_value(),
        }
    }
}
/// Time a retried task took from start to finish.
///
/// Recorded only for tasks that ran more than once.
#[derive(Clone)]
pub struct TaskRetryDurationHistogram { instrument: ::opentelemetry::metrics::Histogram<f64> }
impl TaskRetryDurationHistogram {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().f64_histogram("myapp.task.duration")
            .with_description("Time a task took from start to finish.")
            .with_unit("s")
            .with_boundaries(vec![0_f64, 0.005_f64, 0.01_f64, 0.1_f64, 1_f64, 10_f64])
            .build() }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::myappattr::TaskIdAttr, r#myapp_task_state: super::myappattr::TaskStateAttr, options: impl IntoIterator<Item = TaskRetryDurationHistogramAttr>) {
        let mut attributes = vec![r#myapp_task_id.key_value(), r#myapp_task_state.key_value()];
        attributes.extend(options.into_iter().map(TaskRetryDurationHistogramAttr::key_value));
        self.instrument.record(value, &attributes);
    }
}
impl Default for TaskRetryDurationHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
#[derive(Clone, Debug)]
pub enum TaskStartedCounterAttr {
/// Recommended: When the scheduler assigned an id before the task started.
    TaskId(super::myappattr::TaskIdAttr),
}
impl TaskStartedCounterAttr {
    fn key_value(self) -> ::opentelemetry::KeyValue {
        match self {
            Self::TaskId(value) => value.key_value(),
        }
    }
}
/// Number of tasks started.
/// Opt-in: record only when the user requests it.
#[derive(Clone)]
pub struct TaskStartedCounter { instrument: ::opentelemetry::metrics::Counter<u64> }
impl TaskStartedCounter {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().u64_counter("myapp.task.started")
            .with_description("Number of tasks started.")
            .with_unit("{task}")
            .build() }
    }
    pub fn add(&self, value: u64, options: impl IntoIterator<Item = TaskStartedCounterAttr>) {
        let mut attributes = vec![];
        attributes.extend(options.into_iter().map(TaskStartedCounterAttr::key_value));
        self.instrument.add(value, &attributes);
    }
}
impl Default for TaskStartedCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct TaskStartedCounterObserver<'a> { observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64> }
impl TaskStartedCounterObserver<'_> {
    pub fn observe(&self, value: u64, options: impl IntoIterator<Item = TaskStartedCounterAttr>) {
        let mut attributes = vec![];
        attributes.extend(options.into_iter().map(TaskStartedCounterAttr::key_value));
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct TaskStartedObservableCounter { _instrument: ::opentelemetry::metrics::ObservableCounter<u64> }
impl TaskStartedObservableCounter {
    pub fn new(meter: &super::meter::Meter, callback: impl Fn(TaskStartedCounterObserver<'_>) + Send + Sync + 'static) -> Self {
        Self { _instrument: meter.inner().u64_observable_counter("myapp.task.started")
            .with_description("Number of tasks started.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(TaskStartedCounterObserver { observer }))
            .build() }
    }
}
impl Default for TaskStartedObservableCounter {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
