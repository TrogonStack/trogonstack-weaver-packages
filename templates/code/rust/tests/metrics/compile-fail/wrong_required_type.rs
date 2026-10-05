use generated_semconv::{myappattr, myappmetric};

fn main() {
    myappmetric::TaskDurationHistogram::default().record(0.1, myappattr::TaskStateAttr::Done, []);
}
