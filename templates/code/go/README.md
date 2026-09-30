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

One package per namespace and signal kind, below a root package that only
carries documentation:

| Path                            | Contents                                                             |
| ------------------------------- | -------------------------------------------------------------------- |
| `doc.go`                        | Package `{root_package}`, listing every generated package.           |
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
enums of those. Template and `any` types are not rendered.

### Metrics

Each metric becomes a struct wrapping the matching synchronous instrument, built
with `New<Name><Instrument>(meter)`. Required attributes are typed parameters of
`Add` or `Record`, and every other attribute is a trailing option:

```go
duration, err := myappmetric.NewTaskDurationHistogram(meter)
duration.Record(ctx, 1.5, myappattr.NewTaskIDAttr("task_0001"),
	myappmetric.WithTaskDurationHistogramTaskState(myappattr.TaskStateRunning))
```

The zero value of an instrument records nothing, so a service can hold one
before its meter exists. A metric package imports the attribute packages its
metrics reference, which can belong to other namespaces, and attribute packages
never import metric packages.

The registry schema has no field for an instrument's value type, so every
metric must carry it as an annotation:

```yaml
metrics:
  - name: myapp.task.duration
    instrument: histogram
    annotations:
      go:
        value_type: float64 # or int64
```

### Deprecation

Deprecated attributes, enum members, and metrics are generated with a
`Deprecated:` paragraph in their doc comment, which Go tooling recognizes. Set
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

Generation stops with an error, rather than writing Go that does not compile,
when:

- Two attribute keys in one package render to the same Go identifier, such as
  `myapp.task.id` and `myapp.task_id`.
- Two attributes of one metric render to the same parameter or option name.
- A key has nothing left after its namespace.
- A metric references an attribute that `exclude_deprecated` or `stable_only`
  left out.

## Tests

| Case         | Covers                                                                                                   |
| ------------ | -------------------------------------------------------------------------------------------------------- |
| `attributes` | Every supported type, string and int enums, notes, deprecated attributes and members.                    |
| `metrics`    | Every instrument, required and optional attributes, cross-namespace references.                          |
| `params`     | Custom root package, root description, import path, header, `vendor_prefixes`, and `exclude_deprecated`. |

Run them with `mise run weaver:test:templates`, and compile their expected
output with `mise run weaver:test:go`.
