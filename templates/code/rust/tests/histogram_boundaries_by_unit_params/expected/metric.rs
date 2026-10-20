#![allow(deprecated)]
/// Time a task took from start to finish.
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
/// Time a task waited in the queue, in minutes.
#[derive(Clone)]
pub struct MyappTaskQueueWait {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl MyappTaskQueueWait {
    pub const NAME: &'static str = "myapp.task.queue_wait";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram(Self::NAME)
            .with_description("Time a task waited in the queue, in minutes.")
            .with_unit("min")
            .with_boundaries(vec![1_f64, 5_f64, 10_f64, 30_f64, 60_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::attribute::MyappTaskId) {
        let attributes = [::opentelemetry::KeyValue::from(r#myapp_task_id)];
        self.instrument.record(value, &attributes);
    }
}

impl Default for MyappTaskQueueWait {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
