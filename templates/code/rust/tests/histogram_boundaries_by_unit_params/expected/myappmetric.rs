#![allow(deprecated)]








/// Time a task took from start to finish.
#[derive(Clone)]
pub struct TaskDurationHistogram { instrument: ::opentelemetry::metrics::Histogram<f64> }
impl TaskDurationHistogram {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().f64_histogram("myapp.task.duration")
            .with_description("Time a task took from start to finish.")
            .with_unit("s")

            .build() }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::myappattr::TaskIdAttr) {
        let attributes = vec![r#myapp_task_id.key_value()];

        self.instrument.record(value, &attributes);
    }
}
impl Default for TaskDurationHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}








/// Time a task waited in the queue, in minutes.
#[derive(Clone)]
pub struct TaskQueueWaitHistogram { instrument: ::opentelemetry::metrics::Histogram<f64> }
impl TaskQueueWaitHistogram {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().f64_histogram("myapp.task.queue_wait")
            .with_description("Time a task waited in the queue, in minutes.")
            .with_unit("min")

            .with_boundaries(vec![1_f64, 5_f64, 10_f64, 30_f64, 60_f64])

            .build() }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::myappattr::TaskIdAttr) {
        let attributes = vec![r#myapp_task_id.key_value()];

        self.instrument.record(value, &attributes);
    }
}
impl Default for TaskQueueWaitHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
