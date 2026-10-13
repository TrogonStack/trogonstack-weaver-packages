use generated_semconv::{attribute, metric};

fn main() {
    let _ = metric::MyappTaskDurationAttr::Success(attribute::AuthSuccess::from(true));
}
