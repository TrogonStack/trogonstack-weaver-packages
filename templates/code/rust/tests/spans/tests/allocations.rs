use generated_semconv::{myappattr, myappspan, tracer, workerattr};
use opentelemetry::trace::noop::NoopTracerProvider;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct CountingAllocator;
thread_local! {
    static ALLOCATIONS: Cell<Option<usize>> = const { Cell::new(None) };
}
fn record_allocation() {
    let _ = ALLOCATIONS.try_with(|count| {
        if let Some(value) = count.get() {
            count.set(Some(value + 1));
        }
    });
}
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record_allocation();
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record_allocation();
        unsafe { System.realloc(ptr, layout, size) }
    }
}
#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;
fn allocations<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    ALLOCATIONS.with(|count| count.set(Some(0)));
    let value = operation();
    let count = ALLOCATIONS.with(|count| count.replace(None).unwrap());
    (value, count)
}
#[test]
fn static_scope_constructor_has_no_generated_heap_allocations() {
    let (_, count) = allocations(|| {
        tracer::Tracer::new(
            &NoopTracerProvider::new(),
            generated_semconv::options::InstrumentationOptions {
                scope: "worker".into(),
            },
        )
    });
    assert_eq!(count, 0);
}
#[test]
fn derived_name_only_allocates_output_and_attribute_vector() {
    let tracer = tracer::Tracer::default();
    let context = opentelemetry::Context::new();
    let host = workerattr::HostNameAttr::from("worker-1");
    let port = workerattr::HostPortAttr::from(8080);
    let (_, count) =
        allocations(|| myappspan::start_task_dispatch(&context, &tracer, host, port, []));
    assert_eq!(count, 2);
}

#[test]
fn extreme_float_names_do_not_reallocate_the_output() {
    let tracer = tracer::Tracer::default();
    let context = opentelemetry::Context::new();
    for value in [f64::MAX, -f64::from_bits(1)] {
        let attribute = myappattr::NumberValueAttr::from(value);
        let (_, count) = allocations(|| myappspan::start_name_float(&context, &tracer, attribute));
        assert_eq!(count, 2);
    }
}
#[test]
fn long_owned_string_names_do_not_clone_or_reallocate() {
    let tracer = tracer::Tracer::default();
    let context = opentelemetry::Context::new();
    let host = workerattr::HostNameAttr::from("worker".repeat(1000));
    let port = workerattr::HostPortAttr::from(8080);
    let (_, count) =
        allocations(|| myappspan::start_task_dispatch(&context, &tracer, host, port, []));
    assert_eq!(count, 2);
}
#[test]
fn array_names_only_allocate_output_and_builder_attributes() {
    let tracer = tracer::Tracer::default();
    let context = opentelemetry::Context::new();
    let flags = myappattr::ArrayFlagsAttr::from(vec![true, false]);
    let indices = myappattr::ArrayIndicesAttr::from(vec![-1, 2]);
    let samples = myappattr::ArraySamplesAttr::from(vec![f64::MAX, -f64::from_bits(1)]);
    let tags = myappattr::ArrayTagsAttr::from(vec![
        opentelemetry::StringValue::from("a"),
        opentelemetry::StringValue::from("b"),
    ]);
    let (_, count) = allocations(|| {
        myappspan::start_name_arrays(&context, &tracer, flags, indices, samples, tags)
    });
    assert_eq!(count, 2);
}

#[test]
fn untemplated_names_move_or_borrow_without_heap_allocations() {
    let tracer = tracer::Tracer::default();
    let context = opentelemetry::Context::new();
    for name in [
        myappspan::NameStaticName::default(),
        myappspan::NameStaticName::from("explicit static"),
        myappspan::NameStaticName::from(String::from("owned dynamic")),
    ] {
        let (_, count) = allocations(|| myappspan::start_name_static(&context, &tracer, name));
        assert_eq!(count, 0);
    }
}

#[test]
fn configured_scope_moves_existing_metadata_without_cloning() {
    let scope = generated_semconv::scope::Scope {
        name: "worker".into(),
        version: Some("1.0.0".into()),
        attributes: vec![opentelemetry::KeyValue::new("scope.kind", "worker")],
    };
    let (_, count) = allocations(|| {
        tracer::Tracer::new(
            &NoopTracerProvider::new(),
            generated_semconv::options::InstrumentationOptions { scope },
        )
    });
    assert_eq!(count, 0);
}
