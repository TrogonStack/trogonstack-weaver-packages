use generated_semconv::myappmetric;

fn main() {
    myappmetric::TaskStartedCounter::default().add(-1_i64, []);
}
