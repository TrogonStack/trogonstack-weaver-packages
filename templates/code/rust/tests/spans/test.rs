use generated_semconv::{attribute, trace, tracer, SCHEMA_URL};
use opentelemetry::trace::TraceContextExt;
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};

#[test]
fn derived_names_refinements_parent_context_and_late_attributes() {
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let tracer = tracer::Tracer::new(
        &provider,
        generated_semconv::options::InstrumentationOptions {
            scope: generated_semconv::scope::Scope {
                attributes: vec![opentelemetry::KeyValue::new("scope.kind", "test")],
            },
        },
    );
    let _: &tracer::Tracer<opentelemetry_sdk::trace::SdkTracer> = &tracer;
    let parent = trace::start_myapp_task_dispatch(
        &opentelemetry::Context::new(),
        &tracer,
        attribute::WorkerHostName::from("worker-1"),
        attribute::WorkerHostPort::from(8080),
        [trace::MyappTaskDispatchStartAttr::TaskId(
            attribute::MyappTaskId::from("task-1"),
        )],
    );
    assert!(parent.is_recording());
    assert!(parent.span_context().is_valid());
    let context = parent.into_context(&opentelemetry::Context::new());
    let mut child = trace::start_myapp_task_run(
        &context,
        &tracer,
        trace::MyappTaskRunName::from("run task-1"),
        attribute::MyappTaskState::Done,
        [],
    );
    child.set_attributes([trace::MyappTaskRunAttr::TaskId(attribute::MyappTaskId::from(
        "task-1",
    ))]);
    child.set_status(opentelemetry::trace::Status::Ok);
    child.end();
    context.span().end();
    let mut refined = trace::start_worker_task_dispatch(
        &opentelemetry::Context::new(),
        &tracer,
        attribute::MyappTaskAttempt::from(3),
        attribute::WorkerHostName::from("worker-2"),
        attribute::WorkerHostPort::from(9090),
        [],
    );
    refined.end();
    let mut fallback = trace::start_myapp_task_run(
        &opentelemetry::Context::new(),
        &tracer,
        trace::MyappTaskRunName::default(),
        attribute::MyappTaskState::Done,
        [],
    );
    fallback.end();
    trace::start_myapp_name_apostrophe(
        &opentelemetry::Context::new(),
        &tracer,
        attribute::WorkerHostPort::from(1),
    )
    .end();
    trace::start_myapp_name_backslash(
        &opentelemetry::Context::new(),
        &tracer,
        attribute::WorkerHostPort::from(1),
    )
    .end();
    trace::start_myapp_name_unicode(
        &opentelemetry::Context::new(),
        &tracer,
        attribute::WorkerHostPort::from(1),
    )
    .end();
    trace::start_myapp_name_control(
        &opentelemetry::Context::new(),
        &tracer,
        attribute::WorkerHostPort::from(1),
    )
    .end();
    for value in [f64::MAX, -f64::from_bits(1)] {
        trace::start_myapp_name_float(
            &opentelemetry::Context::new(),
            &tracer,
            attribute::MyappNumberValue::from(value),
        )
        .end();
    }
    trace::start_myapp_name_arrays(
        &opentelemetry::Context::new(),
        &tracer,
        attribute::MyappArrayFlags::from(vec![true, false]),
        attribute::MyappArrayIndices::from(vec![-1, 2]),
        attribute::MyappArraySamples::from(vec![1.5, 2.5]),
        attribute::MyappArrayTags::from(vec![
            opentelemetry::StringValue::from("a"),
            opentelemetry::StringValue::from("b"),
        ]),
    )
    .end();
    for name in [
        trace::MyappNameStaticName::default(),
        trace::MyappNameStaticName::from("explicit static"),
        trace::MyappNameStaticName::from(String::from("owned dynamic")),
    ] {
        trace::start_myapp_name_static(&opentelemetry::Context::new(), &tracer, name).end();
    }
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 14);
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
    assert_eq!(parent.instrumentation_scope.name(), "generated-semconv");
    assert_eq!(parent.instrumentation_scope.version(), None);
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
    for value in [f64::MAX, -f64::from_bits(1)] {
        assert!(value.to_string().len() <= 328);
        assert!(spans.iter().any(|span| span.name == value.to_string()));
    }
    assert!(spans
        .iter()
        .any(|span| span.name == "[\"a\",\"b\"]:[-1,2]:[1.5,2.5]:[true,false]"));
    for name in ["myapp.name.static", "explicit static", "owned dynamic"] {
        assert!(spans.iter().any(|span| span.name == name));
    }
    let mut noop = trace::MyappTaskRun::default();
    assert!(!noop.is_recording());
    noop.end();
}

const _: &str = trace::MyappTaskRun::NAME;

#[test]
fn spans_expose_their_type_as_a_constant() {
    assert_eq!(trace::MyappTaskRun::NAME, "myapp.task.run");
    assert_eq!(trace::WorkerTaskDispatch::NAME, trace::MyappTaskDispatch::NAME);
}

#[test]
fn typed_events_links_names_and_span_context_reach_the_exporter() {
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let tracer = tracer::Tracer::new(&provider, Default::default());
    let linked = trace::start_myapp_task_run(
        &opentelemetry::Context::new(),
        &tracer,
        trace::MyappTaskRunName::from("linked"),
        attribute::MyappTaskState::Done,
        [],
    );
    let linked_context = linked.span_context().clone();
    let mut span = trace::start_myapp_task_dispatch(
        &opentelemetry::Context::new(),
        &tracer,
        attribute::WorkerHostName::from("worker-1"),
        attribute::WorkerHostPort::from(8080),
        [],
    );
    assert!(span.is_recording());
    assert!(span.span_context().is_valid());
    let timestamp = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(5);
    span.add_event(
        "queued",
        [trace::MyappTaskDispatchAttr::TaskAttempt(
            attribute::MyappTaskAttempt::from(2),
        )],
    );
    span.add_event_with_timestamp(
        "retried",
        timestamp,
        [trace::MyappTaskDispatchAttr::QueueName(
            attribute::MyappQueueName::from("reports"),
        )],
    );
    span.add_link(
        linked_context.clone(),
        [trace::MyappTaskDispatchAttr::TaskAttempt(
            attribute::MyappTaskAttempt::from(1),
        )],
    );
    span.end();

    let mut renamed = trace::start_myapp_name_static(
        &opentelemetry::Context::new(),
        &tracer,
        trace::MyappNameStaticName::from("before"),
    );
    renamed.update_name(trace::MyappNameStaticName::from("after"));
    renamed.add_event("plain");
    renamed.add_event_with_timestamp("plain at", timestamp);
    renamed.add_link(linked_context.clone());
    renamed.end();
    let mut defaulted = trace::start_myapp_name_static(
        &opentelemetry::Context::new(),
        &tracer,
        trace::MyappNameStaticName::from("custom"),
    );
    defaulted.update_name(trace::MyappNameStaticName::default());
    defaulted.end();

    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    let dispatch = spans
        .iter()
        .find(|span| span.name == "dispatch worker-1:8080")
        .unwrap();
    let events = dispatch.events.iter().collect::<Vec<_>>();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].name, "queued");
    assert_eq!(
        events[0].attributes,
        [opentelemetry::KeyValue::new("myapp.task.attempt", 2)]
    );
    assert_eq!(events[1].name, "retried");
    assert_eq!(events[1].timestamp, timestamp);
    assert_eq!(
        events[1].attributes,
        [opentelemetry::KeyValue::new("myapp.queue.name", "reports")]
    );
    let links = dispatch.links.iter().collect::<Vec<_>>();
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].span_context, linked_context);
    assert_eq!(
        links[0].attributes,
        [opentelemetry::KeyValue::new("myapp.task.attempt", 1)]
    );
    let renamed = spans.iter().find(|span| span.name == "after").unwrap();
    assert_eq!(
        renamed.events.iter().map(|event| event.name.as_ref()).collect::<Vec<_>>(),
        ["plain", "plain at"]
    );
    assert!(renamed.events.iter().all(|event| event.attributes.is_empty()));
    assert_eq!(renamed.links.iter().count(), 1);
    assert!(spans
        .iter()
        .any(|span| span.name == trace::MyappNameStatic::NAME));
}
