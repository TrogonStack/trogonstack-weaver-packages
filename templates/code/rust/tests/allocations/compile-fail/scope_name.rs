use generated_semconv::{options::InstrumentationOptions, scope::Scope};

fn main() {
    let _ = InstrumentationOptions {
        scope: Scope {
            name: "caller-identity".into(),
            ..Scope::default()
        },
    };
}
