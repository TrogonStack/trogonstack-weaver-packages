#![allow(deprecated)]
/// Time a task took from start to finish.
#[derive(Clone)]
pub struct MyappTaskDuration {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl MyappTaskDuration {
    pub const NAME: &'static str = "myapp.task.duration";
    pub const UNIT: &'static str = "s";
    pub const DESCRIPTION: &'static str = "Time a task took from start to finish.";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram(Self::NAME)
            .with_description(Self::DESCRIPTION)
            .with_unit(Self::UNIT)
            .with_boundaries(vec![0.005_f64, 0.01_f64, 0.025_f64, 0.05_f64, 0.075_f64, 0.1_f64, 0.25_f64, 0.5_f64, 0.75_f64, 1_f64, 2.5_f64, 5_f64, 7.5_f64, 10_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::attribute::MyappTaskId) {
        let attributes = [::opentelemetry::KeyValue::from(r#myapp_task_id)];
        self.instrument.record(value, &attributes);
    }
}

impl Default for MyappTaskDuration {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
/// Time between a task being queued and started.
#[derive(Clone)]
pub struct MyappTaskLatency {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl MyappTaskLatency {
    pub const NAME: &'static str = "myapp.task.latency";
    pub const UNIT: &'static str = "s";
    pub const DESCRIPTION: &'static str = "Time between a task being queued and started.";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram(Self::NAME)
            .with_description(Self::DESCRIPTION)
            .with_unit(Self::UNIT)
            .with_boundaries(vec![1_f64, 10_f64, 100_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::attribute::MyappTaskId) {
        let attributes = [::opentelemetry::KeyValue::from(r#myapp_task_id)];
        self.instrument.record(value, &attributes);
    }
}

impl Default for MyappTaskLatency {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
/// Time a retried task took from start to finish.
#[derive(Clone)]
pub struct MyappTaskRetryDuration {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl MyappTaskRetryDuration {
    pub const NAME: &'static str = "myapp.task.duration";
    pub const UNIT: &'static str = "s";
    pub const DESCRIPTION: &'static str = "Time a task took from start to finish.";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram(Self::NAME)
            .with_description(Self::DESCRIPTION)
            .with_unit(Self::UNIT)
            .with_boundaries(vec![0.005_f64, 0.01_f64, 0.025_f64, 0.05_f64, 0.075_f64, 0.1_f64, 0.25_f64, 0.5_f64, 0.75_f64, 1_f64, 2.5_f64, 5_f64, 7.5_f64, 10_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::attribute::MyappTaskId) {
        let attributes = [::opentelemetry::KeyValue::from(r#myapp_task_id)];
        self.instrument.record(value, &attributes);
    }
}

impl Default for MyappTaskRetryDuration {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
/// Size of a task's payload.
#[derive(Clone)]
pub struct MyappTaskSize {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl MyappTaskSize {
    pub const NAME: &'static str = "myapp.task.size";
    pub const UNIT: &'static str = "By";
    pub const DESCRIPTION: &'static str = "Size of a task\u{0027}s payload.";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram(Self::NAME)
            .with_description(Self::DESCRIPTION)
            .with_unit(Self::UNIT)
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::attribute::MyappTaskId) {
        let attributes = [::opentelemetry::KeyValue::from(r#myapp_task_id)];
        self.instrument.record(value, &attributes);
    }
}

impl Default for MyappTaskSize {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
