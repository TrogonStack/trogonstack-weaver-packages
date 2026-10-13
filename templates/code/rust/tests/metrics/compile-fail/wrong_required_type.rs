use generated_semconv::{attribute, metric};

fn main() {
    metric::MyappTaskDuration::default().record(0.1, attribute::MyappTaskState::Done, []);
}
