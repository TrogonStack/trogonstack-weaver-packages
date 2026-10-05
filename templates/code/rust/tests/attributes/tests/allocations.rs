use generated_semconv::myappattr;
use opentelemetry::{KeyValue, Value};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::hint::black_box;

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
fn string_enum_conversion_has_no_heap_allocation() {
    let (value, count) =
        allocations(|| KeyValue::from(black_box(myappattr::TaskStateAttr::Running)));
    assert_eq!(black_box(count), 0);
    assert_eq!(value.value, Value::String("running".into()));
}

#[test]
fn static_string_construction_and_conversion_have_no_heap_allocation() {
    let (value, count) = allocations(|| {
        let attribute = myappattr::TaskIdAttr::from(black_box("static-task"));
        black_box(KeyValue::from(attribute))
    });
    assert_eq!(black_box(count), 0);
    assert_eq!(value.value, Value::String("static-task".into()));
}

#[test]
fn exact_capacity_owned_string_moves_without_another_allocation() {
    let mut text = String::from("owned-task");
    text.shrink_to_fit();
    let (value, count) =
        allocations(|| KeyValue::from(myappattr::TaskIdAttr::from(black_box(text))));
    assert_eq!(black_box(count), 0);
    assert_eq!(value.value, Value::String("owned-task".into()));
}

#[test]
fn native_string_array_moves_without_another_allocation() {
    let values = vec![opentelemetry::StringValue::from("static-tag")];
    let (_, count) =
        allocations(|| KeyValue::from(myappattr::TaskTagsAttr::from(black_box(values))));
    assert_eq!(black_box(count), 0);
}
