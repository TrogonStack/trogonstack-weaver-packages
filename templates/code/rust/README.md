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

The root `mod.rs` carries `SCHEMA_URL` and declares the generated modules.
Each attribute namespace lives in a sibling `<namespace>attr.rs` file, alongside
the signal modules. Names use the first dotted
segment as their namespace; `vendor_prefixes` removes a leading vendor segment
before choosing that namespace.

| Module              | API                                                                                          |
| ------------------- | -------------------------------------------------------------------------------------------- |
| `<namespace>attr`   | Attribute newtypes and enum variants, `KEY` and conversion into `KeyValue`.                  |
| `meter`             | Schema-aware `Meter`, with `new`, `new_with_scope`, `inner`, and a no-op `Default`.          |
| `tracer`            | Schema-aware `Tracer`, with `new`, `new_with_scope`, `inner`, and a no-op `Default`.         |
| `logger`            | Schema-aware generic `Logger`, with `new`, `new_with_scope`, `inner`, and a no-op `Default`. |
| `<namespace>metric` | Synchronous instruments and observable counters, up-down counters, and gauges.               |
| `<namespace>span`   | Span starters, typed names, start attributes, and attributes allowed after starting.         |
| `<namespace>event`  | Typed log event emitters and options for attributes, severity, and timestamp.                |
| `<namespace>entity` | Typed entity values, attribute iterators, and resources carrying the registry schema URL.    |

Handle modules are emitted when the registry includes their corresponding
signals. They set the registry schema URL on their instrumentation scope.
Use `new_with_scope` to preserve a supplied scope's name, version, and
attributes; its schema URL is replaced by the registry's schema URL.
Attribute keys, metric names, and event names retain their full registry names.

### Typed attributes

An attribute such as `myapp.task.id` becomes
`myappattr::TaskIdAttr::from("task-1")`. Convert the attribute into an OpenTelemetry `KeyValue` with
`KeyValue::from(attribute)` or `attribute.into()`. String attributes implement
`From<String>`, `From<&'static str>`, and `From<opentelemetry::StringValue>`.
Static strings retain their static storage. Own a dynamic borrowed string
explicitly, such as `Attr::from(text.to_owned())`. Scalar and array attributes
implement `From` for their backing types; string arrays accept `Vec<String>`
and native `Vec<opentelemetry::StringValue>`. Native storage moves into the SDK
without rebuilding the vector. Use `Attr::from(value)` or `value.into()` when
the attribute type is known. Enums expose named variants,
such as `myappattr::TaskStateAttr::Running`, rather than arbitrary constructors.

Supported backing types are `StringValue`, `i64`, `f64`, `bool`, and their `Vec`
forms. Enums can have string, integer, fractional, or boolean values.
String attributes implement `AsRef<str>`; arrays expose their borrowed slices
through `AsRef<[T]>`. Attributes implement `Display`
without building a temporary `KeyValue`; arrays follow OpenTelemetry's array
formatting. Boolean, integer, floating-point and enum attributes are `Copy`.
Deprecation metadata becomes Rust deprecation annotations.

### Metrics

Each metric becomes a typed instrument with a `new(&meter)` constructor and
an `add` or `record` method. Required attributes are typed arguments. Other
attributes are variants of the instrument's `Attr` enum, passed as an iterator.
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
attributes, and optional start attributes. A registry name template whose
placeholders refer to required attributes is expanded from those values.
Otherwise the starter takes a typed `Name`, constructed with `From` for a static
string, owned `String`, or `Cow<'static, str>`. An empty name borrows the span's
registry type; static names stay borrowed and owned names move into the span.

All starter attributes are present before sampling. Sampling-relevant
attributes appear only in the start attribute enum. The separate late attribute
enum is accepted by `set_attributes`. The wrapper exposes `end`,
`end_with_timestamp`, `record_error`, `set_status`, and the underlying span.
`into_context` transfers the span into a parent context for nested spans and
normal OpenTelemetry context attachment. The generated tracer and span wrappers
retain the provider's concrete types. Their default type is the no-op API type;
`into_context` uses OpenTelemetry's synchronization and type erasure when
context integration is needed.

Derived names borrow attribute values and write directly into a preallocated
`String`, reserving known literal and string lengths plus space for other
values. Nonempty derived names allocate their output buffer. Required attributes use a
stack array and optional iterators, which the span builder collects into its
owned attribute buffer.

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

That mapping refers to `upstream::errorattr` in the consuming crate. Both
registries must use compatible namespace and vendor-prefix settings.
Plain imported strings use `opentelemetry::StringValue`, and string arrays use
`Vec<opentelemetry::StringValue>`, preserving their SDK storage. Convert a static
string with `.into()` or `StringValue::from` at the call site.

## Parameters

| Parameter                      | Default                     | Purpose                                                                   |
| ------------------------------ | --------------------------- | ------------------------------------------------------------------------- |
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
filtering away required signal attributes fail generation. Attribute names use
ASCII letter-led dotted segments containing letters, digits, and underscores.
Namespace segments must be lowercase and may contain digits and underscores.

Conditional requirements remain options because Rust cannot prove registry
conditions. Event bodies, span events, and links have no registry form; use
the underlying OpenTelemetry handles when needed. Template and `any` attribute
types are not generated.

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
