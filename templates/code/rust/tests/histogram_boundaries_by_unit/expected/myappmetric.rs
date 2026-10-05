#![allow(deprecated)]
/// Time a task took from start to finish.
#[derive(Clone)]
pub struct TaskDurationHistogram {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl TaskDurationHistogram {
    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram("myapp.task.duration")
            .with_description("Time a task took from start to finish.")
            .with_unit("s")
            .with_boundaries(vec![0.005_f64, 0.01_f64, 0.025_f64, 0.05_f64, 0.075_f64, 0.1_f64, 0.25_f64, 0.5_f64, 0.75_f64, 1_f64, 2.5_f64, 5_f64, 7.5_f64, 10_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::myappattr::TaskIdAttr) {
        let attributes = [::opentelemetry::KeyValue::from(r#myapp_task_id)];
        self.instrument.record(value, &attributes);
    }
}

impl Default for TaskDurationHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
/// Time between a task being queued and started.
#[derive(Clone)]
pub struct TaskLatencyHistogram {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl TaskLatencyHistogram {
    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram("myapp.task.latency")
            .with_description("Time between a task being queued and started.")
            .with_unit("s")
            .with_boundaries(vec![1_f64, 10_f64, 100_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::myappattr::TaskIdAttr) {
        let attributes = [::opentelemetry::KeyValue::from(r#myapp_task_id)];
        self.instrument.record(value, &attributes);
    }
}

impl Default for TaskLatencyHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
/// Time a retried task took from start to finish.
#[derive(Clone)]
pub struct TaskRetryDurationHistogram {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl TaskRetryDurationHistogram {
    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram("myapp.task.duration")
            .with_description("Time a task took from start to finish.")
            .with_unit("s")
            .with_boundaries(vec![0.005_f64, 0.01_f64, 0.025_f64, 0.05_f64, 0.075_f64, 0.1_f64, 0.25_f64, 0.5_f64, 0.75_f64, 1_f64, 2.5_f64, 5_f64, 7.5_f64, 10_f64])
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::myappattr::TaskIdAttr) {
        let attributes = [::opentelemetry::KeyValue::from(r#myapp_task_id)];
        self.instrument.record(value, &attributes);
    }
}

impl Default for TaskRetryDurationHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
/// Size of a task's payload.
#[derive(Clone)]
pub struct TaskSizeHistogram {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl TaskSizeHistogram {
    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram("myapp.task.size")
            .with_description("Size of a task\u{0027}s payload.")
            .with_unit("By")
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#myapp_task_id: super::myappattr::TaskIdAttr) {
        let attributes = [::opentelemetry::KeyValue::from(r#myapp_task_id)];
        self.instrument.record(value, &attributes);
    }
}

impl Default for TaskSizeHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
