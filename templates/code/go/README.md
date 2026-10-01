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
  --param root_package=semconv \
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

One package per namespace and signal kind, below a root package that carries
the registry's schema URL:

| Path                            | Contents                                                             |
| ------------------------------- | -------------------------------------------------------------------- |
| `doc.go`                        | Package `{root_package}`, listing every generated package.           |
| `schema.go`                     | `SchemaURL`, plus the `Meter` type when the registry has metrics.    |
| `<namespace>attr/doc.go`        | Package comment for the attribute package.                           |
| `<namespace>attr/attributes.go` | A typed value per attribute, plus enum members as package variables. |
| `<namespace>metric/doc.go`      | Package comment for the metric package.                              |
| `<namespace>metric/metrics.go`  | A typed instrument per metric.                                       |

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

### Schema URL

`SchemaURL` is the `schema_url` from the registry manifest, which must end in
the schema version, such as `https://example.com/schemas/1.0.0`. Instruments are
created from a `{root_package}.Meter` rather than a `metric.Meter`, and the only
way to build one is `NewMeter`, which sets `SchemaURL` on the meter's
instrumentation scope. Telemetry recorded through the generated instruments is
therefore always tied to the registry's schema:

```go
meter := semconv.NewMeter(provider, "example.com/myservice")
```

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

### Deprecation

Deprecated attributes, enum members, and metrics are generated with a
`Deprecated:` paragraph in their doc comment, which Go tooling recognizes. The
members of a deprecated enum carry its paragraph unless they declare their own,
since the registry does not pass an attribute's deprecation on to its members
([weaver#878](https://github.com/open-telemetry/weaver/issues/878)). Set
`exclude_deprecated` to leave them out instead.

## Parameters

| Param                | Default               | Description                                                                                 |
| -------------------- | --------------------- | ------------------------------------------------------------------------------------------- |
| `root_package`       | `semconv`             | Package name declared by the root `doc.go`.                                                 |
| `root_description`   | `""`                  | Extra paragraph in the root `doc.go` comment. Omitted when empty.                           |
| `import_path`        | `example.com/semconv` | Import path of the output directory. Metric packages import attribute packages through it.  |
| `header_source`      | `""`                  | What the code was generated from, shown in the `Code generated` header. Omitted when empty. |
| `regenerate_command` | `""`                  | Command that regenerates the code, shown under the header. Omitted when empty.              |
| `vendor_prefixes`    | `[]`                  | Leading segments that are a vendor prefix rather than a namespace, such as `[acme]`.        |
| `exclude_deprecated` | `false`               | Leave deprecated attributes and metrics out.                                                |
| `stable_only`        | `false`               | Generate only stable attributes and metrics.                                                |

Pass them with `--param key=value` or a `--params` file. Acronyms that stay
upper case in identifiers, such as `ID` and `URL`, are listed in
[`weaver.yaml`](weaver.yaml) and can be replaced from your own `.weaver.toml`.

## Generation errors

Generation stops with an error, rather than writing Go that does not compile
or is tied to no schema, when:

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
- `exclude_deprecated` or `stable_only` keeps a metric but leaves out one of
  its required attributes.

## Tests

| Case         | Covers                                                                                                   |
| ------------ | -------------------------------------------------------------------------------------------------------- |
| `attributes` | Every supported type, enums of every member type, notes, deprecated attributes and members.              |
| `metrics`    | Every instrument, every requirement level, cross-namespace references, bucket boundaries.                |
| `imported`   | Required and optional attributes imported from a dependency registry, including an enum.                 |
| `params`     | Custom root package, root description, import path, header, `vendor_prefixes`, and `exclude_deprecated`. |
| `error_*`    | Each case fails generation with one of the errors above.                                                 |

Run them with `mise run weaver:test:templates`, and compile their expected
output with `mise run weaver:test:go`.
