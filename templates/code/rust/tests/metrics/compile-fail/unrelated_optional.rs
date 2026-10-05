use generated_semconv::{authattr, myappmetric};

fn main() {
    let _ = myappmetric::TaskDurationHistogramAttr::Success(authattr::SuccessAttr::from(true));
}
