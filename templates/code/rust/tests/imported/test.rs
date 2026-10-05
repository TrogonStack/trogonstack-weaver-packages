use generated_semconv::options::InstrumentationOptions;
use generated_semconv::{edgeevent, edgespan, logger, myappattr, myappmetric, tracer};
use opentelemetry::{Array, KeyValue, StringValue, Value};

#[test]
fn imported_native_strings_and_arrays_keep_their_sdk_storage() {
    let methods = vec![StringValue::from("GET"), StringValue::from("POST")];
    let pointer = methods.as_ptr();
    let attribute = KeyValue::from(myappmetric::TaskFailedCounterAttr::HttpRequestMethodList(
        methods,
    ));
    assert_eq!(attribute.key.as_str(), "http.request.method_list");
    let Value::Array(Array::String(methods)) = attribute.value else {
        panic!("imported string arrays must keep native SDK storage");
    };
    assert_eq!(methods.as_ptr(), pointer);
    assert_eq!(
        methods.iter().map(StringValue::as_str).collect::<Vec<_>>(),
        ["GET", "POST"]
    );
    myappmetric::TaskFailedCounter::default().add(
        1,
        StringValue::from("timeout"),
        myappattr::TaskIdAttr::from("task-1"),
        [myappmetric::TaskFailedCounterAttr::HttpRequestMethodList(
            methods,
        )],
    );
    let context = opentelemetry::Context::new();
    edgeevent::emit_upstream_failed(
        &context,
        &logger::Logger::default(),
        StringValue::from("timeout"),
        [edgeevent::UpstreamFailedOption::ServerAddress(
            StringValue::from("example.com"),
        )],
    );
    let mut span = edgespan::start_upstream_request(
        &context,
        &tracer::Tracer::default(),
        StringValue::from("example.com"),
        443,
        [],
    );
    span.end();
}

#[test]
fn imported_native_string_arrays_format_derived_span_names() {
    let exporter = opentelemetry_sdk::trace::InMemorySpanExporter::default();
    let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let tracer = tracer::Tracer::new(&provider, InstrumentationOptions::default());
    let mut span = edgespan::start_upstream_methods(
        &opentelemetry::Context::new(),
        &tracer,
        vec![StringValue::from("GET"), StringValue::from("POST")],
    );
    span.end();
    provider.force_flush().unwrap();
    let spans = exporter.get_finished_spans().unwrap();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].instrumentation_scope.name(), "generated-semconv");
    assert_eq!(spans[0].instrumentation_scope.version(), None);
    assert_eq!(
        spans[0].instrumentation_scope.schema_url(),
        Some(generated_semconv::SCHEMA_URL)
    );
    assert_eq!(spans[0].name, "[\"GET\",\"POST\"]");
}

#[test]
fn imported_native_string_array_events_preserve_order_and_duplicates() {
    let exporter = opentelemetry_sdk::logs::InMemoryLogExporter::default();
    let provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    edgeevent::emit_upstream_failed(
        &opentelemetry::Context::new(),
        &logger::Logger::new(&provider, InstrumentationOptions::default()),
        StringValue::from("timeout"),
        [edgeevent::UpstreamFailedOption::HttpRequestMethodList(
            vec![
                StringValue::from("GET"),
                StringValue::from("GET"),
                StringValue::from("POST"),
            ],
        )],
    );
    provider.force_flush().unwrap();
    let records = exporter.get_emitted_logs().unwrap();
    let value = &records[0]
        .record
        .attributes_iter()
        .find(|(key, _)| key.as_str() == "http.request.method_list")
        .unwrap()
        .1;
    assert_eq!(
        value,
        &opentelemetry::logs::AnyValue::ListAny(Box::new(vec![
            "GET".into(),
            "GET".into(),
            "POST".into()
        ]))
    );
}
