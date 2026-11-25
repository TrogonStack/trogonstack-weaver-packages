# Rust Template Package

Generates typed attributes, metric instruments, span starters, event emitters,
and entity values from a semantic convention registry using OpenTelemetry Rust.

Stability: Development. Generated code targets `opentelemetry` and
`opentelemetry_sdk` 0.33.0.

## Generate and use a module

From a checkout of this repository:

```sh
weaver registry generate \
  --v2 \
  -r {your registry} \
  -t templates \
  code/rust \
  {your crate}/src/semconv
```

For a released package, replace `-t templates` with
`-t 'https://github.com/TrogonStack/trogonstack-weaver-packages.git@v{version}[templates]'`.
Declare `mod semconv;` in the consuming crate and add its dependencies:

```toml
[dependencies]
opentelemetry = "0.33.0"
opentelemetry_sdk = "0.33.0"
smallvec = { version = "=1.16.2", features = ["const_generics"] }
```

Run `cargo fmt` after generation. Weaver writes only the files it produces, so
empty the generated directory before regenerating after removing conventions.
Keep the consuming crate's files outside that directory.

## Generated API

The root `mod.rs` carries `SCHEMA_URL` and declares the generated modules. The
layout mirrors the `opentelemetry-semantic-conventions` crate: one flat module
per signal, holding every namespace. Each generated type and function name
starts with its namespace, the first dotted segment of the registry name;
`vendor_prefixes` removes a leading vendor segment before choosing that
namespace. For example, `myapp.task.id` becomes `attribute::MyappTaskId`,
and the `myapp.task.run` span starter for the `by_state` name template becomes
`trace::start_myapp_task_run_by_state`.

| Module      | API                                                                                            |
| ----------- | ---------------------------------------------------------------------------------------------- |
| `attribute` | Attribute newtypes and enum variants, `KEY` and conversions into `KeyValue` or log `AnyValue`. |
| `options`   | Typed instrumentation options containing scope configuration.                                  |
| `scope`     | Scope attributes configuration; package identity is fixed.                                     |
| `meter`     | Schema-aware `Meter`, with `new` and a no-op `Default`.                                        |
| `tracer`    | Schema-aware `Tracer`, with `new` and a no-op `Default`.                                       |
| `logger`    | Schema-aware generic `Logger`, with `new` and a no-op `Default`.                               |
| `metric`    | Synchronous instruments and observable counters, up-down counters, and gauges.                 |
| `trace`     | Span starters, typed names, start attributes, and attributes allowed after starting.           |
| `event`     | Typed log event emitters and options for attributes, severity, and timestamp.                  |
| `resource`  | Typed entity values, attribute iterators, and resources carrying the registry schema URL.      |

Handle modules are emitted when the registry includes their corresponding
signals. Their `new(provider, options)` constructor takes the exact
`options::InstrumentationOptions` type, with metadata nested under `scope`:

```rust
let options = semconv::options::InstrumentationOptions {
    scope: semconv::scope::Scope {
        attributes: vec![opentelemetry::KeyValue::new("scope.kind", "worker")],
    },
};
let tracer = semconv::tracer::Tracer::new(&provider, options);
```

Use `InstrumentationOptions::default()` when no scope attributes are needed.

The schema-scoped meter, tracer, and logger are consumed only through the
generated instruments, span starters, and emitters. They do not expose the
underlying OpenTelemetry handle, so callers cannot record untyped attributes
through them.

The generated code never reads the OpenTelemetry globals. Build each provider
once in `main` and pass it, or the handles built from it, to whatever records
telemetry, so a test can hand in its own provider and run in parallel with
others. The constructors accept unsized providers, so a component can depend on
`&dyn MeterProvider` rather than on the SDK type:

```rust
fn task_metrics(provider: &dyn opentelemetry::metrics::MeterProvider) -> semconv::meter::Meter {
    semconv::meter::Meter::new(provider, semconv::options::InstrumentationOptions::default())
}
```

The instrumentation identity is fixed during generation. Set
`instrumentation_name` and `instrumentation_version` in the Weaver params to
choose its name and version. The name defaults to `generated-semconv`; an empty
version omits SDK version metadata. The registry supplies the schema URL.
Runtime options cannot override these values.

Configured scope attributes move into OpenTelemetry without rebuilding their
storage. The shared scope and options modules are emitted when a handle is
needed.
Attribute keys, metric names, and event names retain their full registry names.

### Typed attributes

An attribute such as `myapp.task.id` becomes
`attribute::MyappTaskId::from("task-1")`. Convert the attribute into an OpenTelemetry `KeyValue` with
`KeyValue::from(attribute)` or `attribute.into()`. String attributes implement
`From<String>`, `From<&'static str>`, and `From<opentelemetry::StringValue>`.
Static strings retain their static storage. Own a dynamic borrowed string
explicitly, such as `MyappTaskId::from(text.to_owned())`. Scalar and array attributes
implement `From` for their backing types; string arrays accept `Vec<String>`
and native `Vec<opentelemetry::StringValue>`. Native storage moves into the SDK
without rebuilding the vector. Use `MyappTaskId::from(value)` or `value.into()` when
the attribute type is known. Enums expose named variants,
such as `attribute::MyappTaskState::Running`, rather than arbitrary constructors.

Supported backing types are `StringValue`, `i64`, `f64`, `bool`, and their `Vec`
forms. Enums can have string, integer, fractional, or boolean values.
String attributes implement `AsRef<str>`; arrays expose their borrowed slices
through `AsRef<[T]>`. Attributes implement `Display`
without building a temporary `KeyValue`; arrays follow OpenTelemetry's array
formatting. Boolean, integer, floating-point and enum attributes are `Copy`.
Deprecation metadata becomes Rust deprecation annotations.

Attributes also convert directly into `opentelemetry::logs::AnyValue` through
`From`. Generated event emitters pass the attribute type's `KEY` and the typed attribute to
the SDK log record.

### Metrics

Each metric becomes a typed instrument with a `new(&meter)` constructor and
an `add` or `record` method. Required attributes are typed arguments. Other
attributes are variants of the metric's `Attr` enum, such as `metric::MyappTaskDurationAttr`, passed as an iterator.
Observable constructors register a callback whose typed observer has the same
required arguments and optional attribute contract.

Recording required attributes uses a stack array. Optional attributes use an
inline buffer sized for the required attributes and the lesser of the declared
optional attributes or eight. Larger inputs spill to the heap; duplicate
attributes retain their order. This avoids generated buffer allocations for
ordinary attribute sets while preserving iterator input. Attribute values and
the SDK may still require ownership or aggregation allocations.

Set `annotations.code_generation.metric_value_type` to `int` or `double`.
Rust's integer counters and histograms use `u64`; integer up-down counters and
gauges use `i64`. Double instruments use `f64`. Constructors follow the Rust
API and return instruments directly. Observable callbacks return `()`.

Histograms accept `annotations.aggregation` with `method: explicithistogram`
and `parameters.boundaries`. Boundaries must be a nonempty, strictly increasing
list of numbers. Without explicit aggregation, `histogram_boundaries_by_unit`
provides unit-specific boundaries; units absent from that map use SDK defaults.

### Spans

A span starter takes a parent `Context`, a generated tracer, required typed
attributes, and optional start attributes. A span with `name.templates` is
named from the first template whose attributes the starter received, each set,
not empty, and, for an enum, not `_OTHER`. A placeholder may name a required or
an optional attribute. A template without placeholders ends the list and is the
fallback; without one, the fallback is the span's registry type. A placeholder
for an attribute the span does not declare fails generation, and Weaver rejects
a template it cannot parse. A template whose attribute `stable_only` or
`exclude_deprecated` leaves out is dropped. The starter doc comment quotes the
templates, and the name is set once, when the span starts.

Each span has one starter, `start_<name>`. A span with one template takes no
name argument. A span with more than one template takes a
`name: <Name>Name` argument before its required attributes. That enum has one
variant for each template, and the variant chooses the template that names the
span first. A variant holds, as named fields, the placeholder attributes of its
template that the span does not require. A variant for a template without such
attributes is a unit variant. With the templates above, the span
`myapp.task.run` gets this enum:

```rust
pub enum MyappTaskRunName {
    /// `run {myapp.task.id}`
    ByTaskId { myapp_task_id: attribute::MyappTaskId },
    /// `run {myapp.task.state}`
    ByState,
}
```

The state is a required attribute, so `ByState` holds no field and the starter
takes the state as a parameter. A named-field variant without its field does
not compile, and the starter of one span does not accept the name enum of
another span. A call site that has an optional value chooses the variant with
`match` or `if let` and makes one call:

```rust
let name = match task_id {
    Some(task_id) => trace::MyappTaskRunName::ByTaskId { myapp_task_id: task_id },
    None => trace::MyappTaskRunName::ByState,
};
let span = trace::start_myapp_task_run(&context, &tracer, name, state, []);
```

The enum exists whenever the span declares more than one template, also when
`stable_only` or `exclude_deprecated` leaves only one of them.

A span with optional attributes has one start attribute enum,
`<Name>StartAttr`, such as `trace::MyappTaskRunStartAttr`. The starter takes
`options` only when the span has optional attributes. A start attribute can
give the same attribute as a field of the chosen variant. The value in the
variant wins, and the starter drops the matching start attribute, so the span
records the key once.

The starter names the span from the template of the variant first. If a value
of that template is empty, `_OTHER`, or an empty array, the name falls back
through the other templates of the span, in their declared order, and then to
the fallback. A start attribute can fill a later template. For example,
`ByState` with the state `_OTHER` and the start attribute
`MyappTaskRunStartAttr::TaskId("task_0001")` names the span `run task_0001`.

Each template needs an ID, which becomes the variant name in PascalCase. Weaver
gives a template no ID of its own
([open-telemetry/weaver#1916](https://github.com/open-telemetry/weaver/issues/1916)),
so the package derives one. A template with placeholders gets `by_` and the
short names of its distinct placeholder attributes, in order of first use, in
lower snake case and joined with `_and_`. A template without placeholders gets
`fallback`. The package derives the IDs before `stable_only` or
`exclude_deprecated` drops a template, so the variant names do not change with
those parameters. For example:

| Template                                         | Derived ID                   | Variant                 |
| ------------------------------------------------ | ---------------------------- | ----------------------- |
| `run {myapp.task.id}`                            | `by_task_id`                 | `ByTaskId`              |
| `dispatch {worker.host.name}:{worker.host.port}` | `by_host_name_and_host_port` | `ByHostNameAndHostPort` |
| `retry`                                          | `fallback`                   | `Fallback`              |

Two templates that use the same attributes derive the same ID, and generation
fails. Set `annotations.code_generation.name_templates` on that span. It is a
map from an ID to the exact pattern of one template, and it replaces the
derived IDs for every template of the span:

```yaml
name:
  templates:
    - "run {myapp.task.id}"
    - "run {myapp.task.state}"
annotations:
  code_generation:
    name_templates:
      by_task_id: "run {myapp.task.id}"
      by_state: "run {myapp.task.state}"
```

The map names a template by its pattern, not by its position, so a change to
the order of the templates keeps every variant name. Use the map also to keep
a variant name when you rename an attribute.

A span without `name.templates` takes a typed `Name`, since `name.note` is
prose. Construct it with `From` for a static string, owned `String`, or
`Cow<'static, str>`. An empty name borrows the span's registry type; static
names stay borrowed and owned names move into the span.

All starter attributes are present before sampling. Sampling-relevant
attributes appear only in the start attribute enum. The separate late attribute
enum is accepted by `set_attributes`. The wrapper exposes `end`,
`end_with_timestamp`, `record_error`, `set_status`, `span_context`, and
`is_recording`. Events and links take the late attribute enum, so
`add_event`, `add_event_with_timestamp`, and `add_link` accept only declared
attributes; a span with no late attributes takes none. `update_name` is emitted
for spans with a typed `Name` and is absent when the name is derived from
templates. The wrapper does not expose the underlying span, so every attribute
it records is typed. `into_context` transfers the span into a parent context for nested spans and
normal OpenTelemetry context attachment. The generated tracer and span wrappers
retain the provider's concrete types. Their default type is the no-op API type;
`into_context` uses OpenTelemetry's synchronization and type erasure when
context integration is needed. The typed span is the only generated surface;
once the span is in a `Context`, the OpenTelemetry API's own `SpanRef` governs
it, and context propagation is that API's boundary.

A span named from templates collects its start attributes into a `Vec`, which
the span builder takes without copying, and reads the template values from it.
A template that matches writes into a `String` preallocated from known literal
and string lengths plus space for other values, so it allocates once. A literal
fallback is borrowed. Other spans use a stack array for required attributes and
chain the optional iterator.

### Events

Emitters produce log records with the event name and no body. Required
attributes are typed parameters. Options accept declared attributes, severity,
and timestamp. Severity defaults to Info. The emitter checks whether the logger
accepts the event, sets the observed timestamp, and copies a valid trace context
from the supplied `Context`.

The last severity and timestamp options win. Declared attribute options remain
typed until the enabled check succeeds, so disabled events avoid attribute
conversion and context attachment. Optional attributes use inline storage for
the lesser of the declared optional attributes or eight, with heap overflow
for larger inputs. Enabled emitters add required attributes and then optional
attributes directly to the record, preserving duplicates and their order.

### Entities

Entity constructors take required identity attributes before required
description attributes, followed by optional attributes. Required attributes
remain typed fields, and optional attributes remain variants of the entity's
attribute enum. Optional storage uses a bounded inline buffer and supports
heap overflow when the caller supplies more values.

`attributes()` returns an iterator over SDK-owned `KeyValue` values. Borrowed
export clones only the values needed to give the SDK ownership, without
building an intermediate attribute vector. Convert an entity with
`Resource::from(entity)` or `entity.into()` to consume its fields and optional
values without cloning them. The resource retains the registry schema URL and
has no implicit SDK detector attributes. `Default` is available only when the
entity has no required attributes.

### Refinements and dependencies

Metric, span, and entity refinements receive separate APIs and are emitted only
when their base convention is emitted. Metric refinements retain their base
instrument's name, description, unit, value type, and boundaries. Event
refinements follow Weaver's resolved event name, which is the refinement id.

Imported attributes use their plain Rust backing type by default. Set
`imported_crate_paths` to use types from a separately generated dependency:

```yaml
params:
  imported_crate_paths:
    error: upstream
```

That mapping refers to `upstream::attribute` in the consuming crate, so
`error.type` becomes `upstream::attribute::ErrorType`. Both
registries must use compatible namespace and vendor-prefix settings.
Plain imported strings use `opentelemetry::StringValue`, and string arrays use
`Vec<opentelemetry::StringValue>`, preserving their SDK storage. Convert a static
string with `.into()` or `StringValue::from` at the call site.

### Signal metadata

Signal types carry their registry name as an associated constant, usable in
`const` contexts. The generated code reads these constants, so each name
appears once. Metric units and descriptions are not exposed; they reach the SDK
when the instrument is built and appear on the exported metric.

| Type                                | Constants                                 |
| ----------------------------------- | ----------------------------------------- |
| `metric::MyappTaskDuration`         | `NAME`                                    |
| `metric::MyappQueueDepthObservable` | `NAME`, defined as the synchronous type's |
| `trace::MyappTaskRun`               | `NAME`, the span type                     |
| `event::MyappTaskFinished`          | `NAME`, the event name                    |
| `resource::MyappHost`               | `NAME`, the entity type                   |

Refinements report the type they refine. Each event has a unit struct that
holds its `NAME`; emitting still goes through its `emit_` function.

```rust
const DURATION: &str = metric::MyappTaskDuration::NAME;
```

## Parameters

| Parameter                      | Default                     | Purpose                                                                   |
| ------------------------------ | --------------------------- | ------------------------------------------------------------------------- |
| `instrumentation_name`         | `generated-semconv`         | Name baked into every generated instrumentation scope.                    |
| `instrumentation_version`      | `""`                        | Version baked into generated scopes; empty omits SDK version metadata.    |
| `vendor_prefixes`              | `[]`                        | Leading vendor segments removed when deriving namespaces and identifiers. |
| `exclude_deprecated`           | `false`                     | Exclude deprecated conventions and enum members.                          |
| `stable_only`                  | `false`                     | Include only stable conventions and enum members.                         |
| `imported_crate_paths`         | `{}`                        | Namespace to Rust path for a separately generated dependency.             |
| `histogram_boundaries_by_unit` | Duration boundaries for `s` | Default histogram boundaries by unit; `{}` disables them.                 |
| `header_source`                | `""`                        | Source description in the generated header.                               |
| `module_description`           | `""`                        | Additional root module documentation.                                     |
| `regenerate_command`           | `""`                        | Regeneration instruction in the generated header.                         |

## Generation errors and limits

The manifest must provide a schema URL ending in a schema version. Naming
collisions, unusable enums, invalid aggregation, conflicting refinements, and
filtering away required signal attributes fail generation. A span fails when it
has one name template and sets `annotations.code_generation.name_templates`,
or has more than one and does not set it. It also fails when a template has no
ID, an ID names a pattern that matches no template, two IDs name the same
pattern, or an ID does not match `^[a-z][a-z0-9]*(_[a-z0-9]+)*$`. Attribute names use
ASCII letter-led dotted segments containing letters, digits, and underscores.
Namespace segments must be lowercase and may contain digits and underscores.

Conditional requirements remain options because Rust cannot prove registry
conditions. An event body has no registry form, so events are emitted without
one. Span events and links have no registry form either, so `add_event`,
`add_event_with_timestamp`, and `add_link` take only the span's declared late
attributes. Template and `any` attribute types are not generated.

## Validate a change

```sh
mise run weaver:test:templates
mise run weaver:test:rust
```

The Rust check formats generated fixture code in temporary crates, runs consumer
and documentation tests, verifies compile-fail examples, and runs Clippy with
warnings denied. Snapshot checks compare the original Weaver output.
The allocation fixture runs in release mode with a thread-local counting
allocator and custom providers that do not allocate while recording. It checks
generated buffer costs separately from SDK storage costs and verifies duplicate
attributes beyond inline capacity.
