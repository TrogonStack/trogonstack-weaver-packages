use generated_semconv::{options::InstrumentationOptions, scope::Scope, tracer, SCHEMA_URL};
use opentelemetry::trace::{Span as _, Tracer as _};
use opentelemetry::{Context, InstrumentationScope, KeyValue};
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};
use std::fmt::Debug;

fn require_clone_and_debug<T: Clone + Debug>(_: &T) {}

#[test]
fn default_tracer_supports_clone_and_debug() {
    let tracer = tracer::Tracer::default();
    require_clone_and_debug(&tracer);
    require_clone_and_debug(&tracer.clone());
}

#[test]
fn cloned_sdk_tracer_preserves_scope_and_exports_spans() {
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let original = tracer::Tracer::new(
        &provider,
        InstrumentationOptions {
            scope: Scope {
                attributes: vec![KeyValue::new("scope.kind", "clone-test")],
            },
        },
    );
    require_clone_and_debug(&original);
    let cloned = original.clone();
    drop(original);
    let mut span = cloned.inner().start("cloned-handle");
    span.end();
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, "cloned-handle");
    assert_eq!(
        spans[0].instrumentation_scope.schema_url(),
        Some(SCHEMA_URL)
    );
    assert!(spans[0]
        .instrumentation_scope
        .attributes()
        .any(|attribute| attribute == &KeyValue::new("scope.kind", "clone-test")));
    provider.shutdown().unwrap();
}

struct BareTracer;
impl opentelemetry::trace::Tracer for BareTracer {
    type Span = opentelemetry::trace::noop::NoopSpan;
    fn build_with_context(&self, _: opentelemetry::trace::SpanBuilder, _: &Context) -> Self::Span {
        opentelemetry::trace::noop::NoopSpan::DEFAULT
    }
}
struct BareProvider;
impl opentelemetry::trace::TracerProvider for BareProvider {
    type Tracer = BareTracer;
    fn tracer_with_scope(&self, _: InstrumentationScope) -> Self::Tracer {
        BareTracer
    }
}

#[test]
fn custom_tracer_without_clone_or_debug_still_constructs_and_starts_spans() {
    let tracer = tracer::Tracer::new(&BareProvider, InstrumentationOptions::default());
    let mut span = tracer.inner().start("bare-handle");
    assert!(!span.is_recording());
    span.end();
}
