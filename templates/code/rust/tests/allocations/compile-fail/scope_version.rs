use generated_semconv::{options::InstrumentationOptions, scope::Scope};

fn main() {
    let _ = InstrumentationOptions {
        scope: Scope {
            version: Some("1.2.3".into()),
            ..Scope::default()
        },
    };
}
