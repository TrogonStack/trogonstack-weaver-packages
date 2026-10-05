use generated_semconv::{myappattr, myappspan};
fn main() {
    let mut span = myappspan::TaskDispatchSpan::default();
    span.set_attributes([myappspan::TaskDispatchStartAttr::TaskId(
        myappattr::TaskIdAttr::from("task-1"),
    )]);
}
