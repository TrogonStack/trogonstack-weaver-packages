use generated_semconv::{options::InstrumentationOptions, scope::Scope};

fn main() {
    let _ = InstrumentationOptions {
        scope: Scope {
            schema_url: Some("https://example.com/caller/1.0.0".into()),
            ..Scope::default()
        },
    };
}
