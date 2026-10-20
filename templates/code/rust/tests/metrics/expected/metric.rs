#![allow(deprecated)]
/// Number of authentication attempts.
#[derive(Clone)]
pub struct AuthAttempts {
    instrument: ::opentelemetry::metrics::Counter<u64>,
}

impl AuthAttempts {
    pub const NAME: &'static str = "auth.attempts";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().u64_counter(Self::NAME)
            .with_description("Number of authentication attempts.")
            .with_unit("{attempt}")
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: u64, r#auth_method: super::attribute::AuthMethod, r#auth_success: super::attribute::AuthSuccess) {
        let attributes = [::opentelemetry::KeyValue::from(r#auth_method), ::opentelemetry::KeyValue::from(r#auth_success)];
        self.instrument.add(value, &attributes);
    }
}

impl Default for AuthAttempts {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct AuthAttemptsObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64>,
}

impl AuthAttemptsObserver<'_> {
    pub fn observe(&self, value: u64, r#auth_method: super::attribute::AuthMethod, r#auth_success: super::attribute::AuthSuccess) {
        let attributes = [::opentelemetry::KeyValue::from(r#auth_method), ::opentelemetry::KeyValue::from(r#auth_success)];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct AuthAttemptsObservable {
    _instrument: ::opentelemetry::metrics::ObservableCounter<u64>,
}

impl AuthAttemptsObservable {
    pub const NAME: &'static str = AuthAttempts::NAME;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(AuthAttemptsObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().u64_observable_counter(Self::NAME)
            .with_description("Number of authentication attempts.")
            .with_unit("{attempt}")
            .with_callback(move |observer| callback(AuthAttemptsObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for AuthAttemptsObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
/// Number of tasks waiting in the queue.
#[deprecated(note = "Replaced by `myapp.task.active`.")]
#[derive(Clone)]
pub struct MyappQueueDepth {
    instrument: ::opentelemetry::metrics::Gauge<i64>,
}

impl MyappQueueDepth {
    pub const NAME: &'static str = "myapp.queue.depth";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().i64_gauge(Self::NAME)
            .with_description("Number of tasks waiting in the queue.")
            .with_unit("{task}")
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: i64) {
        let attributes = [];
        self.instrument.record(value, &attributes);
    }
}

impl Default for MyappQueueDepth {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct MyappQueueDepthObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<i64>,
}

impl MyappQueueDepthObserver<'_> {
    pub fn observe(&self, value: i64) {
        let attributes = [];
        self.observer.observe(value, &attributes);
    }
}
#[deprecated(note = "Replaced by `myapp.task.active`.")]
#[derive(Clone)]
pub struct MyappQueueDepthObservable {
    _instrument: ::opentelemetry::metrics::ObservableGauge<i64>,
}

impl MyappQueueDepthObservable {
    pub const NAME: &'static str = MyappQueueDepth::NAME;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(MyappQueueDepthObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().i64_observable_gauge(Self::NAME)
            .with_description("Number of tasks waiting in the queue.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(MyappQueueDepthObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for MyappQueueDepthObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
/// Number of tasks currently running.
#[derive(Clone)]
pub struct MyappTaskActive {
    instrument: ::opentelemetry::metrics::UpDownCounter<i64>,
}

impl MyappTaskActive {
    pub const NAME: &'static str = "myapp.task.active";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().i64_up_down_counter(Self::NAME)
            .with_description("Number of tasks currently running.")
            .with_unit("{task}")
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: i64, r#myapp_task_state: super::attribute::MyappTaskState) {
        let attributes = [::opentelemetry::KeyValue::from(r#myapp_task_state)];
        self.instrument.add(value, &attributes);
    }
}

impl Default for MyappTaskActive {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct MyappTaskActiveObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<i64>,
}

impl MyappTaskActiveObserver<'_> {
    pub fn observe(&self, value: i64, r#myapp_task_state: super::attribute::MyappTaskState) {
        let attributes = [::opentelemetry::KeyValue::from(r#myapp_task_state)];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct MyappTaskActiveObservable {
    _instrument: ::opentelemetry::metrics::ObservableUpDownCounter<i64>,
}

impl MyappTaskActiveObservable {
    pub const NAME: &'static str = MyappTaskActive::NAME;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(MyappTaskActiveObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().i64_observable_up_down_counter(Self::NAME)
            .with_description("Number of tasks currently running.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(MyappTaskActiveObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for MyappTaskActiveObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
#[derive(Clone, Debug)]
pub enum MyappTaskDurationAttr {
    /// Opt-in: the convention records it only when a user asks for it.
    Method(super::attribute::AuthMethod),
    /// Conditionally required: If the task reached a final state.
    TaskState(super::attribute::MyappTaskState),
}

impl From<MyappTaskDurationAttr> for ::opentelemetry::KeyValue {
    fn from(value: MyappTaskDurationAttr) -> Self {
        match value {
            MyappTaskDurationAttr::Method(value) => ::opentelemetry::KeyValue::from(value),
            MyappTaskDurationAttr::TaskState(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
/// Time a task took from start to finish.
///
/// Measured by the worker that ran the task.
#[derive(Clone)]
pub struct MyappTaskDuration {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl MyappTaskDuration {
    pub const NAME: &'static str = "myapp.task.duration";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram(Self::NAME)
            .with_description("Time a task took from start to finish.")
            .with_unit("s")
            .with_boundaries(vec![0_f64, 0.005_f64, 0.01_f64, 0.1_f64, 1_f64, 10_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::attribute::MyappTaskId, options: impl IntoIterator<Item = MyappTaskDurationAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 3]>::new();
        attributes.extend([::opentelemetry::KeyValue::from(r#myapp_task_id)]);
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.instrument.record(value, &attributes);
    }
}

impl Default for MyappTaskDuration {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
/// Fractional active task adjustment.
#[derive(Clone)]
pub struct MyappTaskFloatActive {
    instrument: ::opentelemetry::metrics::UpDownCounter<f64>,
}

impl MyappTaskFloatActive {
    pub const NAME: &'static str = "myapp.task.float.active";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_up_down_counter(Self::NAME)
            .with_description("Fractional active task adjustment.")
            .with_unit("{task}")
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: f64) {
        let attributes = [];
        self.instrument.add(value, &attributes);
    }
}

impl Default for MyappTaskFloatActive {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct MyappTaskFloatActiveObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<f64>,
}

impl MyappTaskFloatActiveObserver<'_> {
    pub fn observe(&self, value: f64) {
        let attributes = [];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct MyappTaskFloatActiveObservable {
    _instrument: ::opentelemetry::metrics::ObservableUpDownCounter<f64>,
}

impl MyappTaskFloatActiveObservable {
    pub const NAME: &'static str = MyappTaskFloatActive::NAME;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(MyappTaskFloatActiveObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().f64_observable_up_down_counter(Self::NAME)
            .with_description("Fractional active task adjustment.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(MyappTaskFloatActiveObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for MyappTaskFloatActiveObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
/// Fractional queue depth.
#[derive(Clone)]
pub struct MyappTaskFloatDepth {
    instrument: ::opentelemetry::metrics::Gauge<f64>,
}

impl MyappTaskFloatDepth {
    pub const NAME: &'static str = "myapp.task.float.depth";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_gauge(Self::NAME)
            .with_description("Fractional queue depth.")
            .with_unit("{task}")
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64) {
        let attributes = [];
        self.instrument.record(value, &attributes);
    }
}

impl Default for MyappTaskFloatDepth {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct MyappTaskFloatDepthObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<f64>,
}

impl MyappTaskFloatDepthObserver<'_> {
    pub fn observe(&self, value: f64) {
        let attributes = [];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct MyappTaskFloatDepthObservable {
    _instrument: ::opentelemetry::metrics::ObservableGauge<f64>,
}

impl MyappTaskFloatDepthObservable {
    pub const NAME: &'static str = MyappTaskFloatDepth::NAME;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(MyappTaskFloatDepthObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().f64_observable_gauge(Self::NAME)
            .with_description("Fractional queue depth.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(MyappTaskFloatDepthObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for MyappTaskFloatDepthObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
/// Fractional tasks started.
#[derive(Clone)]
pub struct MyappTaskFloatStarted {
    instrument: ::opentelemetry::metrics::Counter<f64>,
}

impl MyappTaskFloatStarted {
    pub const NAME: &'static str = "myapp.task.float.started";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_counter(Self::NAME)
            .with_description("Fractional tasks started.")
            .with_unit("{task}")
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: f64) {
        let attributes = [];
        self.instrument.add(value, &attributes);
    }
}

impl Default for MyappTaskFloatStarted {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct MyappTaskFloatStartedObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<f64>,
}

impl MyappTaskFloatStartedObserver<'_> {
    pub fn observe(&self, value: f64) {
        let attributes = [];
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct MyappTaskFloatStartedObservable {
    _instrument: ::opentelemetry::metrics::ObservableCounter<f64>,
}

impl MyappTaskFloatStartedObservable {
    pub const NAME: &'static str = MyappTaskFloatStarted::NAME;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(MyappTaskFloatStartedObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().f64_observable_counter(Self::NAME)
            .with_description("Fractional tasks started.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(MyappTaskFloatStartedObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for MyappTaskFloatStartedObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
/// Task payload size.
#[derive(Clone)]
pub struct MyappTaskPayload {
    instrument: ::opentelemetry::metrics::Histogram<u64>,
}

impl MyappTaskPayload {
    pub const NAME: &'static str = "myapp.task.payload";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().u64_histogram(Self::NAME)
            .with_description("Task payload size.")
            .with_unit("By")
            .with_boundaries(vec![128_f64, 256_f64, 1024_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: u64) {
        let attributes = [];
        self.instrument.record(value, &attributes);
    }
}

impl Default for MyappTaskPayload {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
#[derive(Clone, Debug)]
pub enum MyappTaskRetryDurationAttr {
    /// Opt-in: the convention records it only when a user asks for it.
    Method(super::attribute::AuthMethod),
}

impl From<MyappTaskRetryDurationAttr> for ::opentelemetry::KeyValue {
    fn from(value: MyappTaskRetryDurationAttr) -> Self {
        match value {
            MyappTaskRetryDurationAttr::Method(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
/// Time a retried task took from start to finish.
///
/// Recorded only for tasks that ran more than once.
#[derive(Clone)]
pub struct MyappTaskRetryDuration {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl MyappTaskRetryDuration {
    pub const NAME: &'static str = "myapp.task.duration";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram(Self::NAME)
            .with_description("Time a task took from start to finish.")
            .with_unit("s")
            .with_boundaries(vec![0_f64, 0.005_f64, 0.01_f64, 0.1_f64, 1_f64, 10_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::attribute::MyappTaskId, r#myapp_task_state: super::attribute::MyappTaskState, options: impl IntoIterator<Item = MyappTaskRetryDurationAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 3]>::new();
        attributes.extend([::opentelemetry::KeyValue::from(r#myapp_task_id), ::opentelemetry::KeyValue::from(r#myapp_task_state)]);
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.instrument.record(value, &attributes);
    }
}

impl Default for MyappTaskRetryDuration {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
#[derive(Clone, Debug)]
pub enum MyappTaskStartedAttr {
    /// Recommended: When the scheduler assigned an id before the task started.
    TaskId(super::attribute::MyappTaskId),
}

impl From<MyappTaskStartedAttr> for ::opentelemetry::KeyValue {
    fn from(value: MyappTaskStartedAttr) -> Self {
        match value {
            MyappTaskStartedAttr::TaskId(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
/// Number of tasks started.
/// Opt-in: record only when the user requests it.
#[derive(Clone)]
pub struct MyappTaskStarted {
    instrument: ::opentelemetry::metrics::Counter<u64>,
}

impl MyappTaskStarted {
    pub const NAME: &'static str = "myapp.task.started";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().u64_counter(Self::NAME)
            .with_description("Number of tasks started.")
            .with_unit("{task}")
            .build();
        Self { instrument }
    }
    pub fn add(&self, value: u64, options: impl IntoIterator<Item = MyappTaskStartedAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 1]>::new();
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.instrument.add(value, &attributes);
    }
}

impl Default for MyappTaskStarted {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
pub struct MyappTaskStartedObserver<'a> {
    observer: &'a dyn ::opentelemetry::metrics::AsyncInstrument<u64>,
}

impl MyappTaskStartedObserver<'_> {
    pub fn observe(&self, value: u64, options: impl IntoIterator<Item = MyappTaskStartedAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 1]>::new();
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.observer.observe(value, &attributes);
    }
}

#[derive(Clone)]
pub struct MyappTaskStartedObservable {
    _instrument: ::opentelemetry::metrics::ObservableCounter<u64>,
}

impl MyappTaskStartedObservable {
    pub const NAME: &'static str = MyappTaskStarted::NAME;

    pub fn new(meter: &super::meter::Meter, callback: impl Fn(MyappTaskStartedObserver<'_>) + Send + Sync + 'static) -> Self {
        let instrument = meter.inner().u64_observable_counter(Self::NAME)
            .with_description("Number of tasks started.")
            .with_unit("{task}")
            .with_callback(move |observer| callback(MyappTaskStartedObserver { observer }))
            .build();
        Self { _instrument: instrument }
    }
}

impl Default for MyappTaskStartedObservable {
    fn default() -> Self { Self::new(&super::meter::Meter::default(), |_| {}) }
}
#[derive(Clone, Debug)]
pub enum WorkerTaskDurationAttr {
    /// Opt-in: the convention records it only when a user asks for it.
    Method(super::attribute::AuthMethod),
    /// Conditionally required: If the task reached a final state.
    TaskState(super::attribute::MyappTaskState),
}

impl From<WorkerTaskDurationAttr> for ::opentelemetry::KeyValue {
    fn from(value: WorkerTaskDurationAttr) -> Self {
        match value {
            WorkerTaskDurationAttr::Method(value) => ::opentelemetry::KeyValue::from(value),
            WorkerTaskDurationAttr::TaskState(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
/// Time a task took on the worker that ran it.
///
/// Measured by the worker that ran the task.
#[derive(Clone)]
pub struct WorkerTaskDuration {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl WorkerTaskDuration {
    pub const NAME: &'static str = "myapp.task.duration";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram(Self::NAME)
            .with_description("Time a task took from start to finish.")
            .with_unit("s")
            .with_boundaries(vec![0_f64, 0.005_f64, 0.01_f64, 0.1_f64, 1_f64, 10_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::attribute::MyappTaskId, options: impl IntoIterator<Item = WorkerTaskDurationAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 3]>::new();
        attributes.extend([::opentelemetry::KeyValue::from(r#myapp_task_id)]);
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.instrument.record(value, &attributes);
    }
}

impl Default for WorkerTaskDuration {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
