# Go Template Package

Generates typed Go attributes and metric instruments from a semantic convention
registry, on top of the OpenTelemetry Go API.

Stability: Development

## Usage

```bash
weaver registry generate \
  --v2 \
  -r {your registry} \
  -p 'https://github.com/TrogonStack/trogonstack-weaver-packages.git@v{version}[policies/check/go_codegen]' \
  -t 'https://github.com/TrogonStack/trogonstack-weaver-packages.git@v{version}[templates]' \
  --param import_path=example.com/myservice/internal/semconv \
  code/go \
  internal/semconv
```

The package reads the version 2 resolved schema, so `--v2` is required. Pair it
with the [`check/go_codegen`](../../../policies/check/go_codegen/README.md)
policy package so that a registry the template cannot render fails with a
diagnostic instead of producing Go that does not compile.

Weaver only writes the files a template produces, so empty the output directory
before each run, or files for conventions you removed stay behind.

## What it generates

One package per namespace and signal kind, below a root package, `<root>`,
that carries the registry's schema URL:

| Path                            | Contents                                                               |
| ------------------------------- | ---------------------------------------------------------------------- |
| `doc.go`                        | The root package, listing every generated package.                     |
| `schema.go`                     | `SchemaURL`.                                                           |
| `<root>meter/doc.go`            | Package comment for the meter package, when the registry has metrics.  |
| `<root>meter/meter.go`          | The `Meter` every instrument is created from.                          |
| `<namespace>attr/doc.go`        | Package comment for the attribute package.                             |
| `<namespace>attr/attributes.go` | A typed value per attribute, plus enum members as package variables.   |
| `<namespace>metric/doc.go`      | Package comment for the metric package.                                |
| `<namespace>metric/metrics.go`  | A typed instrument per metric and refinement, plus asynchronous forms. |

The namespace is the first segment of a name, or the second when the first is
listed in `vendor_prefixes`. Identifiers drop that prefix, so `myapp.task.id`
becomes `myappattr.TaskIDAttr`.

### Attributes

Each attribute becomes a struct that can only be built through its constructor
or, for enums, through its members:

```go
myappattr.NewTaskIDAttr("task_0001").KeyValue()
myappattr.TaskStateQueued.KeyValue()
myappattr.TaskIDAttr{}.Key()
```

Supported types are `string`, `int`, `double`, `boolean`, their array forms, and
enums of those. An enum takes the Go type of its members: `string`, `int64`,
`float64` when any member is fractional, or `bool`. Template and `any` types are
not rendered.

### Root package

The root package is named after the last element of `import_path`, as Go
expects of a package in that directory, so `example.com/acme/acmeconv` declares
`package acmeconv`. Set `root_package` to name it otherwise.

A version belongs in `import_path`, never in a package name. Elements shaped
like a version, such as `v2` or `v1.2.0`, are skipped, so
`example.com/acme/acmeconv/v1.2.0` still declares `package acmeconv`, and two
versions of a registry can live side by side under one module. A file that needs
both renames one on import.

### Schema URL

`SchemaURL` is the `schema_url` from the registry manifest, which must end in
the schema version, such as `https://example.com/schemas/1.0.0`. The root
package declares nothing else, so code that only needs the URL imports no
OpenTelemetry API.

Instruments are created from a `<root>meter.Meter` rather than a
`metric.Meter`, and the only way to build one is `<root>meter.New`, which sets
`SchemaURL` on the meter's instrumentation scope. Telemetry recorded through the
generated instruments is therefore always tied to the registry's schema:

```go
meter := semconvmeter.New(provider, "example.com/myservice")
```

The meter lives in its own package, named after the root package so that it
never clashes with the OpenTelemetry `metric` package imported beside it, and
only code that records metrics imports the metric API through it.

A schema URL passed in the options is replaced. The zero `Meter` creates
instruments that record nothing.

### Metrics

Each metric becomes a struct wrapping the matching synchronous instrument, built
with `New<Name><Instrument>(meter)`. Required attributes are typed parameters of
`Add` or `Record`, and every other attribute is a trailing option:

```go
duration, err := myappmetric.NewTaskDurationHistogram(meter)
duration.Record(ctx, 1.5, myappattr.NewTaskIDAttr("task_0001"),
	myappmetric.WithTaskDurationHistogramTaskState(myappattr.TaskStateRunning))
```

Counters, up-down counters, and gauges also get an asynchronous form, built
with `New<Name>Observable<Instrument>(meter, callback)`. The callback receives a
typed observer whose `Observe` takes the same parameters and options as the
synchronous method, so an observation cannot skip a required attribute or
attach one the convention does not declare:

```go
_, err := myappmetric.NewTaskActiveObservableUpDownCounter(meter,
	func(ctx context.Context, o myappmetric.TaskActiveUpDownCounterObserver) error {
		o.Observe(int64(queue.Len()), myappattr.TaskStateQueued)
		return nil
	})
```

The callback has its own type, `<Name><Instrument>Callback`, so it can be
declared ahead of the constructor. It is registered when the instrument is
created and runs on every collection. An error it returns is returned by that
collection, and the observations made before it are still recorded. Histograms
have no asynchronous form in OpenTelemetry, so they get none.

Go cannot check a requirement condition, so conditionally required,
recommended, and opt-in attributes are all options. The option's doc comment
carries the level and its condition, such as
`Conditionally required: If the task reached a final state.`, and a metric whose
own `requirement_level` is `opt_in` says so on its type and constructor.

The zero value of an instrument records nothing, so a service can hold one
before its meter exists. A metric package imports the attribute packages its
metrics reference, which can belong to other namespaces, and attribute packages
never import metric packages.

An attribute imported from a dependency registry, such as `error.type` from the
OpenTelemetry registry, has no generated package. It is taken as its plain Go
value instead, with the key filled in, so an imported enum is taken as the Go
value of its members:

```go
failed.Add(ctx, 1, "timeout", myappattr.NewTaskIDAttr("task_0001"))
```

Every metric must set its value type with the
`code_generation.metric_value_type` annotation that the OpenTelemetry semantic
conventions use, either `int` for an `Int64` instrument or `double` for a
`Float64` one:

```yaml
metrics:
  - name: myapp.task.duration
    instrument: histogram
    annotations:
      code_generation:
        metric_value_type: double
```

A histogram can also set the bucket boundaries its constructor passes to
`metric.WithExplicitBucketBoundaries`, in strictly increasing order. Without
them the SDK's default boundaries apply, which suit milliseconds rather than a
unit such as `s` or `By`.

This is experimental. The registry schema has no field for it yet, so it is
read from `annotations.aggregation`, which mirrors the metric aggregation field
proposed in [weaver#844](https://github.com/open-telemetry/weaver/issues/844)
and will follow that proposal as it changes. Only `method: explicithistogram`
with `parameters.boundaries` is accepted, since that is all the Go API can
express:

```yaml
metrics:
  - name: myapp.task.duration
    instrument: histogram
    unit: s
    annotations:
      code_generation:
        metric_value_type: double
      aggregation:
        method: explicithistogram
        parameters:
          boundaries: [0.005, 0.01, 0.05, 0.1, 0.5, 1, 5, 10]
```

### Metric refinements

Each metric refinement becomes its own typed instrument, named from its id,
with the attributes and requirement levels the refinement declares. A
refinement that makes an attribute required therefore makes it a parameter:

```go
retried, err := myappmetric.NewTaskRetryDurationHistogram(meter)
retried.Record(ctx, 1.5, myappattr.NewTaskIDAttr("task_0001"), myappattr.TaskStateDone)
```

It records under the name of the metric it refines, created with that metric's
description, unit, value type, and bucket boundaries, since the SDK reports two
instruments of one name with different descriptions as a conflict. A refinement
is generated only when the metric it refines is.

### Deprecation

Deprecated attributes, enum members, and metrics are generated with a
`Deprecated:` paragraph in their doc comment, which Go tooling recognizes. The
members of a deprecated enum carry its paragraph unless they declare their own,
since the registry does not pass an attribute's deprecation on to its members
([weaver#878](https://github.com/open-telemetry/weaver/issues/878)). Set
`exclude_deprecated` to leave them out instead.

### Not generated

Spans, events, and entities are not generated: the package covers attributes
and metrics only. Attribute `examples` and entity associations are not rendered
either, since neither has a Go API counterpart.

### Known registry caveats

- Annotations set on an attribute reference replace the attribute's own rather
  than merging with them
  ([weaver#1705](https://github.com/open-telemetry/weaver/issues/1705)), so a
  reference that sets any annotation drops an inherited
  `code_generation.exclude`. Repeat it on the reference.
- Enum members do not inherit their attribute's deprecation
  ([weaver#878](https://github.com/open-telemetry/weaver/issues/878)). The
  template carries it over, see [Deprecation](#deprecation).

## Parameters

| Param                | Default               | Description                                                                                            |
| -------------------- | --------------------- | ------------------------------------------------------------------------------------------------------ |
| `root_package`       | `""`                  | Package name declared by the root `doc.go`. When empty, the last non-version element of `import_path`. |
| `root_description`   | `""`                  | Extra paragraph in the root `doc.go` comment. Omitted when empty.                                      |
| `import_path`        | `example.com/semconv` | Import path of the output directory. Metric packages import attribute packages through it.             |
| `header_source`      | `""`                  | What the code was generated from, shown in the `Code generated` header. Omitted when empty.            |
| `regenerate_command` | `""`                  | Command that regenerates the code, shown under the header. Omitted when empty.                         |
| `vendor_prefixes`    | `[]`                  | Leading segments that are a vendor prefix rather than a namespace, such as `[acme]`.                   |
| `exclude_deprecated` | `false`               | Leave deprecated attributes and metrics out.                                                           |
| `stable_only`        | `false`               | Generate only stable attributes and metrics.                                                           |

Pass them with `--param key=value` or a `--params` file. Acronyms that stay
upper case in identifiers, such as `ID` and `URL`, are listed in
[`weaver.yaml`](weaver.yaml) and can be replaced from your own `.weaver.toml`.

## Generation errors

Generation stops with an error, rather than writing Go that does not compile
or is tied to no schema, when:

- The root package name, whether set or taken from `import_path`, is not a
  valid Go package name, such as `my-conv` or `go`.
- The registry has no manifest declaring a `schema_url`, or the URL does not
  end in the schema version.
- Two attribute keys in one package render to the same Go identifier, such as
  `myapp.task.id` and `myapp.task_id`.
- Two attributes of one metric render to the same parameter or option name.
- A key has nothing left after its namespace.
- An enum mixes member value types, or `exclude_deprecated` or `stable_only`
  leaves it with no members.
- A metric references an imported attribute whose type is not supported.
- `annotations.aggregation` is set on an instrument other than a histogram,
  uses a method or parameter other than `explicithistogram` and `boundaries`,
  or its boundaries are not a non-empty list of numbers in strictly increasing
  order.
- Two metrics or refinements in one package render to the same Go
  identifier, such as `myapp.task.started` and `myapp.task_started`, or the
  observable form of one is named like the other, such as
  `myapp.task.started` and `myapp.task.started.observable`.
- A metric refinement sets a different `code_generation.metric_value_type` or
  `annotations.aggregation` than the metric it refines.
- `exclude_deprecated` or `stable_only` keeps a metric but leaves out one of
  its required attributes.

## Tests

| Case                            | Covers                                                                                                   |
| ------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `attributes`                    | Every supported type, enums of every member type, notes, deprecated attributes and members.              |
| `metrics`                       | Every instrument, every requirement level, cross-namespace references, bucket boundaries.                |
| `imported`                      | Required and optional attributes imported from a dependency registry, including an enum.                 |
| `params`                        | Custom root package, root description, import path, header, `vendor_prefixes`, and `exclude_deprecated`. |
| `root_package_from_import_path` | The root package named after `import_path`.                                                              |
| `root_package_version_segment`  | A version element at the end of `import_path` skipped in the root package name.                          |
| `error_*`                       | Each case fails generation with one of the errors above.                                                 |

Run them with `mise run weaver:test:templates`, and compile their expected
output with `mise run weaver:test:go`.
