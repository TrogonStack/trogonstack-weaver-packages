use generated_semconv::{authattr, myappmetric};

fn main() {
    let _ = myappmetric::TaskDurationHistogramAttr::Success(authattr::SuccessAttr::new(true));
}
