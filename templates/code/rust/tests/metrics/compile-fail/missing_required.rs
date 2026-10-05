use generated_semconv::{authattr, authmetric};

fn main() {
    authmetric::AttemptsCounter::default().add(1, authattr::MethodAttr::Password);
}
