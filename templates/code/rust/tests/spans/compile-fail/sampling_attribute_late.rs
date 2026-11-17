use generated_semconv::{attribute, trace};
fn main() {
    let mut span = trace::MyappTaskDispatch::default();
    span.set_attributes([trace::MyappTaskDispatchByWorkerStartAttr::TaskId(
        attribute::MyappTaskId::from("task-1"),
    )]);
}
