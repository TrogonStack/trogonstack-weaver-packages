#![allow(deprecated)]
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
/// Time a task took on the worker that ran it.
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
