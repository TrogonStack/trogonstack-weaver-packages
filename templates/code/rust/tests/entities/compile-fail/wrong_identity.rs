use generated_semconv::{attribute, resource};

fn main() {
    let _ = resource::MyappHost::new(attribute::MyappQueueName::from("queue-1"), []);
}
