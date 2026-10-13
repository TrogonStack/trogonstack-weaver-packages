use generated_semconv::{attribute, metric};

fn main() {
    metric::AuthAttempts::default().add(1, attribute::AuthMethod::Password);
}
