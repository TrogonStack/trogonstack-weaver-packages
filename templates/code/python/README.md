# Python Template Package

Generates typed attribute keys, enum classes, and metric instruments from a
semantic convention registry using the vanilla OpenTelemetry Python metrics API.

Stability: Development. Generated code is tested against `opentelemetry-api`
and `opentelemetry-sdk` 1.45.2 on Python 3.12, and passes `mypy --strict`,
`ruff check`, and `ruff format --check`.

Importing generated code needs only `opentelemetry-api`. The SDK is not
imported, so a service keeps its choice of SDK, exporters, and views.

Spans, events, and entities are out of scope. The template does not render them
and does not fail on a registry that contains them. Attributes that only those
signals use still get key constants and enum classes.

## Generate and use a package

From a checkout of this repository:

```sh
weaver registry generate \
  --v2 \
  -r {your registry} \
  -t templates \
  code/python \
  {your project}/semconv
```

For a released package, replace `-t templates` with
`-t 'https://github.com/TrogonStack/trogonstack-weaver-packages.git@v{version}[templates]'`.
The output directory becomes a Python package with three flat modules:
`__init__.py`, `attributes.py`, and `metrics.py`. Import it by the name of the
directory, here `semconv`. Add the one runtime dependency:

```toml
[project]
requires-python = ">=3.12"
dependencies = ["opentelemetry-api>=1.30.0"]
```

Version 1.30.0 is the first release whose `create_histogram` accepts
`explicit_bucket_boundaries_advisory`. Weaver writes only the files it
produces, so empty the generated directory before regenerating after removing
conventions. Run `ruff format` after generation if your project formats
generated code. The output is already formatted for Ruff's default style, but a
very long attribute or metric name can make Ruff choose a different wrapping.
Generated code does not keep every line within 88 characters, so do not enforce
`E501` on it.

## Generated API

The package root carries the scope identity and the schema URL of the registry.

```python
from semconv import INSTRUMENTATION_NAME, INSTRUMENTATION_VERSION, SCHEMA_URL
```

`SCHEMA_URL` is the registry's `schema_url`. `get_meter` in `semconv.metrics`
passes it as `schema_url` together with the instrumentation name and version, so
every instrument reports the schema it was generated from. A registry whose
`schema_url` does not end in a schema version fails generation.

### Attributes

Each attribute becomes a `Final` key constant named from the attribute key, with
the registry brief as its docstring. An enum attribute also becomes an
`enum.StrEnum`, `enum.IntEnum`, `float` plus `enum.Enum`, or `enum.Enum` class,
chosen by the value type of its members, with one member per registry member.

```python
from semconv import attributes

attributes.MYAPP_TASK_ID_KEY  # "myapp.task.id"
attributes.MyappTaskState.QUEUED.value  # "queued"
```

Constants are named `<NAME>_KEY`. An enum class has the title-cased name of the
attribute, without the `_KEY` suffix. Deprecated attributes and members carry a
`Deprecated:` line in their docstring.

### Metrics

Each metric becomes one class. The class constants carry the registry values, so
code that records by name, unit, or description can read them from one place.

```python
from semconv import metrics

metrics.MyappTaskDuration.NAME  # "myapp.task.duration"
metrics.MyappTaskDuration.UNIT  # "s"
metrics.MyappTaskDuration.DESCRIPTION
metrics.MyappTaskDuration.KIND  # metrics.InstrumentKind.HISTOGRAM
metrics.MyappTaskDuration.BUCKET_BOUNDARIES  # tuple of floats, or None
```

The constructor creates the instrument on a meter. It uses `get_meter()` when
you pass none, or the meter you pass:

```python
duration = metrics.MyappTaskDuration()
duration.record(0.42, task_id="task-1", task_state=attributes.MyappTaskState.QUEUED)
```

Attributes are keyword-only parameters, named from the attribute key without the
leading metric namespace. Required attributes are required parameters. Every
other requirement level is an optional parameter that defaults to `None` and is
left out of the recorded attributes when `None`. Enum members are recorded by
value, and array attributes are copied into a list at record time.

| Instrument kind | Class method | Observable class method and constructor           |
| --------------- | ------------ | ------------------------------------------------- |
| `counter`       | `add`        | `Observable.observation`, `Observable(callbacks)` |
| `updowncounter` | `add`        | `Observable.observation`, `Observable(callbacks)` |
| `gauge`         | `set`        | `Observable.observation`, `Observable(callbacks)` |
| `histogram`     | `record`     | none, because the API has no observable histogram |

Counters, up-down counters, and gauges also get a `<Name>Observable` class.
Pass callbacks to its constructor. Inside a callback, build each value with the
static `observation` method, which takes the same attributes as the synchronous
method and returns an `Observation`.

```python
def read_depth(options: CallbackOptions) -> Iterable[Observation]:
    yield metrics.MyappQueueDepthObservable.observation(depth(), queue_name="main")


metrics.MyappQueueDepthObservable([read_depth])
```

A parameter that would collide with Python, such as a keyword or `amount`,
fails generation instead of being renamed.

### Refinements

A metric refinement gets its own class that keeps the name, unit, description,
value type, and boundaries of the metric it refines. Its attributes come from
the refinement, and its parameter names drop the namespace of the refinement id.
Refinements are emitted only when their base metric is emitted.

### Histogram boundaries

A histogram takes `explicit_bucket_boundaries_advisory` from, in order:

1. `annotations.aggregation.parameters.boundaries`, when the metric sets
   `annotations.aggregation.method: explicithistogram`.
2. `histogram_boundaries_by_unit` for the metric's unit.
3. Nothing, which leaves the SDK default boundaries in place.

The boundaries are an advisory. A view that sets its own boundaries overrides
them. `BUCKET_BOUNDARIES` is `None` when no boundaries apply.

## Parameters

| Parameter                      | Default                     | Purpose                                                                  |
| ------------------------------ | --------------------------- | ------------------------------------------------------------------------ |
| `instrumentation_name`         | `generated-semconv`         | Scope name passed to `get_meter`.                                        |
| `instrumentation_version`      | `""`                        | Scope version passed to `get_meter`; empty leaves the scope unversioned. |
| `vendor_prefixes`              | `[]`                        | Leading vendor segments removed when deriving names.                     |
| `exclude_deprecated`           | `false`                     | Exclude deprecated conventions and enum members.                         |
| `stable_only`                  | `false`                     | Include only stable conventions and enum members.                        |
| `histogram_boundaries_by_unit` | Duration boundaries for `s` | Default histogram boundaries by unit; `{}` disables them.                |
| `header_source`                | `""`                        | Source description in the generated header.                              |
| `module_description`           | `""`                        | Docstring of the generated package.                                      |
| `regenerate_command`           | `""`                        | Regeneration instruction in the generated header.                        |

With `vendor_prefixes: [acme]`, `acme.billing.invoice.id` renders as
`BILLING_INVOICE_ID_KEY`, exactly like `billing.invoice.id`.

## Generation errors and limits

Generation fails with a diagnostic, instead of emitting invalid Python, when:

- the manifest has no schema URL that ends in a schema version
- a metric does not set `code_generation.metric_value_type` to `int` or `double`
- two attributes, enum members, metrics, or parameters render to the same
  Python identifier, or one renders to a reserved Python symbol
- an attribute name is not ASCII letter-led dotted segments of letters, digits,
  and underscores, or an enum member id is not a valid identifier
- an enum has no members left after filtering, duplicate values, mixed value
  types, or an integer value outside 64 bits
- an attribute has a type the template cannot express, such as a template type
  or `any`
- `exclude_deprecated` or `stable_only` leaves out an attribute a kept metric
  requires
- `annotations.aggregation` is set on anything but a histogram, sets fields
  other than `method` and `parameters.boundaries`, names a method other than
  `explicithistogram`, or lists boundaries that are empty or not strictly
  increasing
- a metric refinement sets a value type that differs from its base metric
- `instrumentation_name` is empty or not a string, or
  `instrumentation_version` is not a string

A conditionally required attribute stays optional, because Python cannot prove
the registry condition.

## Validate a change

```sh
mise run weaver:test:templates
mise run weaver:test:python
```

The Python check installs locked, hash-verified requirements into a disposable
virtual environment outside the checkout. For each case it copies the generated
fixture next to its `test_consumer.py`, then runs `ruff format --check`,
`ruff check`, `mypy --strict`, and `pytest`. Each consumer records through an
SDK `MeterProvider` with an `InMemoryMetricReader` and asserts names, units,
attributes, the schema URL, bucket boundaries, and that the generated constants
equal the registry values. Snapshot checks compare the original Weaver output.
