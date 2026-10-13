use generated_semconv::{attribute, trace};
fn main() {
    let mut span = trace::MyappTaskDispatch::default();
    span.set_attributes([trace::MyappTaskDispatchStartAttr::TaskId(
        attribute::MyappTaskId::from("task-1"),
    )]);
}
