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
    assert_eq!(spans[0].instrumentation_scope.name(), "generated_semconv");
    assert_eq!(spans[0].instrumentation_scope.version(), Some("0.0.0"));
    assert_eq!(
        spans[0].instrumentation_scope.schema_url(),
        Some(generated_semconv::SCHEMA_URL)
    );
    assert_eq!(spans[0].name, "[\"GET\",\"POST\"]");
}
