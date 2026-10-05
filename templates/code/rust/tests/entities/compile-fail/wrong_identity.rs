use generated_semconv::{myappattr, myappentity};

fn main() {
    let _ = myappentity::HostEntity::new(myappattr::QueueNameAttr::new("queue-1"), []);
}
