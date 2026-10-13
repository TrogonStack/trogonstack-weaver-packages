use generated_semconv::metric;

fn main() {
    metric::MyappTaskStarted::default().add(-1_i64, []);
}
