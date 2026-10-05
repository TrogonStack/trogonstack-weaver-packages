use generated_semconv::{myappattr, myappspan, tracer, workerattr, workerspan, SCHEMA_URL};
use opentelemetry::trace::{Span, TraceContextExt};
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};

#[test]
fn derived_names_refinements_parent_context_and_late_attributes() {
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let tracer = tracer::Tracer::new_with_scope(
        &provider,
        opentelemetry::InstrumentationScope::builder("worker")
            .with_version("1.2.3")
            .with_schema_url("https://example.com/ignored/1.0.0")
            .with_attributes([opentelemetry::KeyValue::new("scope.kind", "test")])
            .build(),
    );
    let parent = myappspan::start_task_dispatch(
        &opentelemetry::Context::new(),
        &tracer,
        workerattr::HostNameAttr::new("worker-1"),
        workerattr::HostPortAttr::new(8080),
        [myappspan::TaskDispatchStartAttr::TaskId(
            myappattr::TaskIdAttr::new("task-1"),
        )],
    );
    let context = parent.into_context(&opentelemetry::Context::new());
    let mut child = myappspan::start_task_run(
        &context,
        &tracer,
        myappspan::TaskRunName::new("run task-1"),
        myappattr::TaskStateAttr::Done,
        [],
    );
    child.set_attributes([myappspan::TaskRunAttr::TaskId(myappattr::TaskIdAttr::new(
        "task-1",
    ))]);
    child.set_status(opentelemetry::trace::Status::Ok);
    child.end();
    context.span().end();
    let mut refined = workerspan::start_task_dispatch(
        &opentelemetry::Context::new(),
        &tracer,
        myappattr::TaskAttemptAttr::new(3),
        workerattr::HostNameAttr::new("worker-2"),
        workerattr::HostPortAttr::new(9090),
        [],
    );
    refined.end();
    let mut fallback = myappspan::start_task_run(
        &opentelemetry::Context::new(),
        &tracer,
        myappspan::TaskRunName::default(),
        myappattr::TaskStateAttr::Done,
        [],
    );
    fallback.end();
    myappspan::start_name_apostrophe(
        &opentelemetry::Context::new(),
        &tracer,
        workerattr::HostPortAttr::new(1),
    )
    .end();
    myappspan::start_name_backslash(
        &opentelemetry::Context::new(),
        &tracer,
        workerattr::HostPortAttr::new(1),
    )
    .end();
    myappspan::start_name_unicode(
        &opentelemetry::Context::new(),
        &tracer,
        workerattr::HostPortAttr::new(1),
    )
    .end();
    myappspan::start_name_control(
        &opentelemetry::Context::new(),
        &tracer,
        workerattr::HostPortAttr::new(1),
    )
    .end();
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 8);
    let child = spans.iter().find(|span| span.name == "run task-1").unwrap();
    let parent = spans
        .iter()
        .find(|span| span.name == "dispatch worker-1:8080")
        .unwrap();
    assert_eq!(child.parent_span_id, parent.span_context.span_id());
    assert_eq!(parent.span_kind, opentelemetry::trace::SpanKind::Client);
    assert_eq!(child.status, opentelemetry::trace::Status::Ok);
    assert!(child
        .attributes
        .iter()
        .any(|kv| kv.key.as_str() == "myapp.task.id" && kv.value.as_str() == "task-1"));
    assert_eq!(parent.instrumentation_scope.schema_url(), Some(SCHEMA_URL));
    assert_eq!(parent.instrumentation_scope.name(), "worker");
    assert_eq!(parent.instrumentation_scope.version(), Some("1.2.3"));
    assert!(parent
        .instrumentation_scope
        .attributes()
        .any(|kv| kv.key.as_str() == "scope.kind"));
    let refined = spans
        .iter()
        .find(|span| span.name == "dispatch worker-2:9090")
        .unwrap();
    assert!(refined
        .attributes
        .iter()
        .any(|kv| kv.key.as_str() == "myapp.task.attempt"
            && kv.value == opentelemetry::Value::I64(3)));
    assert!(spans.iter().any(|span| span.name == "myapp.task.run"));
    for name in ["'1", "\\1", "é1", "\u{0008}1"] {
        assert!(spans.iter().any(|span| span.name == name));
    }
    let mut noop = myappspan::TaskRunSpan::default();
    assert!(!noop.span().is_recording());
    noop.end();
}
