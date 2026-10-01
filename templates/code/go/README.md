# Go Template Package

Generates typed Go attributes, metric instruments, span starters, event
emitters, and entity value types from a semantic convention registry, on top
of the OpenTelemetry Go API.

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
| `<root>tracer/doc.go`           | Package comment for the tracer package, when the registry has spans.   |
| `<root>tracer/tracer.go`        | The `Tracer` that starts spans tied to the schema.                     |
| `<root>logger/doc.go`           | Package comment for the logger package, when the registry has events.  |
| `<root>logger/logger.go`        | The `Logger` that emits events tied to the schema.                     |
| `<namespace>attr/doc.go`        | Package comment for the attribute package.                             |
| `<namespace>attr/attributes.go` | A typed value per attribute, plus enum members as package variables.   |
| `<namespace>metric/doc.go`      | Package comment for the metric package.                                |
| `<namespace>metric/metrics.go`  | A typed instrument per metric and refinement, plus asynchronous forms. |
| `<namespace>span/doc.go`        | Package comment for the span package.                                  |
| `<namespace>span/spans.go`      | A typed starter and span per span and refinement.                      |
| `<namespace>event/doc.go`       | Package comment for the event package.                                 |
| `<namespace>event/events.go`    | A typed `Emit` function per event and refinement.                      |
| `<namespace>entity/doc.go`      | Package comment for the entity package.                                |
| `<namespace>entity/entities.go` | A typed value type per entity and refinement.                          |

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

### Tracer

When the registry declares a span, `<root>tracer` holds a `Tracer` built the
same way as the meter, with `SchemaURL` set on its instrumentation scope, so
spans started from it are tied to the registry's schema. The generated
[span starters](#spans) take it, and `TraceTracer` exposes the underlying
`trace.Tracer` for spans the registry does not declare:

```go
tracer := semconvtracer.New(provider, "example.com/myservice")
```

The zero `Tracer` starts spans that record nothing.

### Logger

When the registry declares an event, `<root>logger` holds a `Logger` built the
same way, over the OpenTelemetry Logs API, so log records emitted from it are
tied to the registry's schema. The generated [event emitters](#events) take
it, and `LogLogger` exposes the underlying `log.Logger` for records the
registry does not declare:

```go
logger := semconvlogger.New(provider, "example.com/myservice")
```

The zero `Logger` emits log records that go nowhere.

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
OpenTelemetry registry, has no generated package by default. It is taken as its
plain Go value instead, with the key filled in, so an imported enum is taken as
the Go value of its members:

```go
failed.Add(ctx, 1, "timeout", myappattr.NewTaskIDAttr("task_0001"))
```

Map its namespace to the Go import path of a dependency registry that was
itself generated with this template, with `imported_import_paths`, and the
attribute is taken as the typed value from `<path>/<ns>attr` instead, exactly
like a local attribute:

```go
failed.Add(ctx, 1, errorattr.TypeTimeout, myappattr.NewTaskIDAttr("task_0001"))
```

The map's key is the attribute's namespace as this registry's own
`vendor_prefixes` computes it, so the dependency must have been generated with
`vendor_prefixes` that name the same attributes the same way. An attribute
whose namespace is not in the map keeps the plain-value behavior above.

A metric name always keeps a namespace, unlike a span or event name, so a
metric with nothing left after its namespace is a registry error rather than
a package of its own: see [Generation errors](#generation-errors).

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
`metric.WithExplicitBucketBoundaries`, in strictly increasing order.

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

A histogram that sets no `annotations.aggregation` falls back to
`histogram_boundaries_by_unit`, keyed by the metric's `unit`, validated the
same way as explicit boundaries. Its default covers `s`, the semconv unit for
durations, with the boundaries the OpenTelemetry semantic conventions advise
for a duration histogram in seconds, such as `http.server.request.duration`.
A unit missing from the map falls back to the SDK's own default boundaries,
which suit milliseconds rather than a unit such as `s` or `By`. Set the param
to `{}` to opt out, or replace it to cover other units.

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

### Spans

Each span becomes a `Start<Name>` function and a `<Name>Span` it returns. The
starter takes the `<root>tracer.Tracer`, then every required attribute as a
typed parameter, then the rest as options, and starts the span with the kind
the registry gives it:

```go
ctx, span := myappspan.StartTaskDispatch(ctx, tracer,
	workerattr.NewHostNameAttr("worker-1"),
	workerattr.NewHostPortAttr(8080),
	myappspan.WithTaskDispatchTaskID(myappattr.NewTaskIDAttr("task_0001")),
)
defer span.End()
span.SetAttributes(myappspan.WithTaskDispatchQueueName(myappattr.NewQueueNameAttr("reports")))
```

Every attribute given to the starter is set as the span starts, since a sampler
sees only those. An option for an attribute marked `sampling_relevant` returns
only `<Name>StartAttr`, which the starter accepts, while every other option
returns `<Name>Attr`, which `SetAttributes` also accepts after the span started.
`<Name>Span` also has `End`, `RecordError`, `SetStatus`, and `Span` for the
underlying `trace.Span`. The zero `<Name>Span` records nothing.

The span name comes from `name.note`, read only for a name template: a single
code span holding at least one `{placeholder}`, each the key of an attribute
the span requires. Such a span is named from those attributes, so
`` `dispatch {worker.host.name}:{worker.host.port}` `` names the span above
`dispatch worker-1:8080`. Any other span takes a typed `<Name>Name`, whose doc
comment quotes `name.note`, since Go cannot check prose:

```go
ctx, span := myappspan.StartTaskRun(ctx, tracer,
	myappspan.NewTaskRunName("run task_0001"),
	myappattr.TaskStateQueued,
)
```

An empty `<Name>Name`, including the zero one, names the span after its type.

Each span refinement becomes its own starter, named from its id, with the
kind, name, attributes, and requirement levels the refinement resolves to. A
refinement is generated only when the span it refines is.

A span whose name has a single segment, such as a vendor-prefix-only name,
has nothing left after its namespace. Rather than failing, it renders
unprefixed, in its own `<name>span` package:

```go
ctx, span := heartbeatspan.Start(ctx, tracer, heartbeatattr.NewSequenceAttr(1))
defer span.End()
```

### Events

Each event becomes an `Emit<Name>` function that emits it as a log record
through the logger, with the event's name and no body. Required attributes are
typed parameters, and the rest are `<Name>Option` values, alongside the
severity and the timestamp:

```go
myappevent.EmitTaskFinished(ctx, logger, myappattr.TaskStateDone,
	myappevent.WithTaskFinishedTaskAttempt(myappattr.NewTaskAttemptAttr(2)),
	myappevent.WithTaskFinishedSeverity(log.SeverityWarn),
)
```

The severity is `log.SeverityInfo` unless an option sets another. Without a
timestamp option the record carries only the time it was observed. The record
is emitted only when `Enabled` reports the logger takes an event of that name
and severity, and the zero `Logger` emits nothing.

Each event refinement becomes its own `Emit` function, named from its id. Weaver
resolves a refinement's event name to its id and keeps no link to the event it
refines, so the record carries the refinement's id as its event name and the
refinement is generated on its own stability and deprecation.

An event whose name has a single segment, such as the upstream `exception`
event, has nothing left after its namespace. Rather than failing, it renders
unprefixed, in its own `<name>event` package:

```go
exceptionevent.Emit(ctx, logger, exceptionattr.NewTypeAttr("ValueError"),
	exceptionevent.WithMessage(exceptionattr.NewMessageAttr("division by zero")),
)
```

### Entities

Each entity becomes a `<Name>Entity` value type built by `New<Name>Entity`.
A required attribute of the entity's identity or description is a typed
parameter of the constructor, and every other attribute is a trailing option:

```go
host := myappentity.NewHostEntity(
	myappattr.NewHostNameAttr("worker-1"),
	myappentity.WithHostEntityHostType(myappattr.HostTypeVirtual),
)
```

`Attributes` returns a copy of the attributes the entity was built from, and
`Resource` builds a `*resource.Resource` from them, tied to the registry's
schema. Resources from the same registry merge cleanly:

```go
res, err := resource.Merge(host.Resource(), queue.Resource())
```

The zero `<Name>Entity` carries no attributes. Identity attributes come
before description attributes in both the constructor and the doc comment, so
a reader can tell at a glance which attributes identify the entity and which
merely describe it.

Each entity refinement becomes its own value type, named from its id, with
the identity and description the refinement resolves to. Unlike an event
refinement, weaver keeps the link to the entity it refines, so the doc
comment names the base entity. A refinement is generated only when the entity
it refines is.

An entity whose type has a single segment, such as `host`, has nothing left
after its namespace. Rather than failing, it renders
unprefixed, in its own `<name>entity` package:

```go
host := hostentity.NewEntity(hostattr.NewIDAttr("h-1"))
```

### Deprecation

Deprecated attributes, enum members, metrics, spans, and events are generated with a
`Deprecated:` paragraph in their doc comment, which Go tooling recognizes. The
members of a deprecated enum carry its paragraph unless they declare their own,
since the registry does not pass an attribute's deprecation on to its members
([weaver#878](https://github.com/open-telemetry/weaver/issues/878)). Set
`exclude_deprecated` to leave them out instead.

### Not generated

An event body has no registry form, so events are emitted without one. Span
events and links have no registry form either, so `Span` exposes the
`trace.Span` they are added through.
Attribute `examples` and an attribute's `entity_associations` are not
rendered either, since neither has a Go API counterpart.

### Known registry caveats

- Annotations set on an attribute reference replace the attribute's own rather
  than merging with them
  ([weaver#1705](https://github.com/open-telemetry/weaver/issues/1705)), so a
  reference that sets any annotation drops an inherited
  `code_generation.exclude`. Repeat it on the reference.
- Enum members do not inherit their attribute's deprecation
  ([weaver#878](https://github.com/open-telemetry/weaver/issues/878)). The
  template carries it over, see [Deprecation](#deprecation).

## Wiring it into a program

Create one `sdklog.LoggerProvider`, `sdktrace.TracerProvider`, and
`sdkmetric.MeterProvider` per process, register them as the OpenTelemetry
globals, and build the generated handles from them once at startup. A
provider caches the logger, tracer, or meter it hands out by instrumentation
scope, so calling `<root>logger.New`, `<root>tracer.New`, or `<root>meter.New`
again with the same scope never creates a second SDK instance. Build each
handle once and pass it down instead of rebuilding it. `devlog.Processor`
below, under [Readable local output](#readable-local-output), is a processor
that forwards records to a plain handler so they also print to the terminal.

```go
const scope = "example.com/myservice/internal/worker"

func main() {
	ctx := context.Background()

	exporter, err := otlploghttp.New(ctx)
	if err != nil {
		log.Fatal(err)
	}
	lp := sdklog.NewLoggerProvider(
		sdklog.WithProcessor(sdklog.NewBatchProcessor(exporter)),
		sdklog.WithProcessor(devlog.Processor{Handler: slog.NewTextHandler(os.Stderr, nil)}),
	)
	defer lp.Shutdown(ctx)
	global.SetLoggerProvider(lp)
	slog.SetDefault(otelslog.NewLogger(scope, otelslog.WithLoggerProvider(lp)))

	logger := semconvlogger.New(lp, scope)
	tracer := semconvtracer.New(tracerProvider, scope)
	meter := semconvmeter.New(meterProvider, scope)

	run(ctx, logger, tracer, meter)
}
```

`tracerProvider` and `meterProvider` are built the same way, with
`sdktrace.NewTracerProvider` and `sdkmetric.NewMeterProvider`, and registered
with `otel.SetTracerProvider` and `otel.SetMeterProvider`. A library that
takes its telemetry from the ambient OpenTelemetry API, rather than from a
handle passed to it, reaches the provider through those globals instead of
building one of its own, so it still ties into the same pipeline.

The scope is the instrumenting package's import path, such as
`example.com/myservice/internal/worker` above, not the service name. The
service name belongs in the `Resource` passed to all three providers with
`WithResource`, since every provider in a process should describe the same
service.

An entity describing the process, such as the host it runs on, belongs in
that same `Resource`. Pass its attributes to `resource.New` under the
registry's schema URL:

```go
host := myappentity.NewHostEntity(
	myappattr.NewHostNameAttr(hostname),
	myappentity.WithHostEntityHostType(myappattr.HostTypeVirtual),
)

res, err := resource.New(ctx,
	resource.WithSchemaURL(semconv.SchemaURL),
	resource.WithAttributes(host.Attributes()...),
)
if err != nil {
	log.Fatal(err)
}

lp := sdklog.NewLoggerProvider(sdklog.WithResource(res), ...)
```

The SDK's own resources, such as `resource.Default()` and the detectors
`resource.WithTelemetrySDK` and `resource.WithHost`, carry the schema URL of
the semantic conventions the SDK was built against. Merging one with an
entity's `Resource` fails with `resource.ErrSchemaURLConflict` and drops the
schema URL, unless the registry is that same release. Add the SDK's
attributes through `resource.WithAttributes(resource.Default().Attributes()...)`
to keep the registry's schema URL instead.

The zero value of a handle, like the zero `Logger` the [Logger](#logger)
section describes, records or emits nothing, so a package can hold one before
its provider exists.

### slog and typed events share one pipeline

`slog.SetDefault(otelslog.NewLogger(scope, otelslog.WithLoggerProvider(lp)))`
makes slog calls and the generated `Emit...` functions feed the same
`LoggerProvider`, so a line written with `slog.InfoContext` lands beside the
events emitted through `<root>logger.Logger`. Always use the `...Context`
variants, `InfoContext`, `WarnContext`, and so on: `otelslog` reads the trace
and span IDs from `ctx`, and without it a line is not linked to the span it
happened in.

Libraries take a `*slog.Logger`, or call `slog.Default()`, and never build
their own handler, so they inherit whichever handler `main` installed.

Use the typed event when the registry defines the occurrence, and slog for
everything else: debugging, startup, operational notes. Never log a fact both
ways. When a slog line starts driving dashboards or alerts, add it to the
registry and regenerate to get a typed `Emit...` in its place.

### Readable local output

A `sdklog.LoggerProvider` can run more than one processor, and that, not a
slog multi-handler, is where local output should fan out. The generated
`Emit...` functions call `log.Logger.Emit` directly, bypassing slog, so a
`slog.NewMultiHandler` tee registered only on the slog side never sees them.
A processor registered on the provider sees every record either path
produces:

```go
type Processor struct{ Handler slog.Handler }

func (p Processor) OnEmit(ctx context.Context, r *sdklog.Record) error {
	msg := r.Body().Emit()
	if name := r.EventName(); name != "" {
		msg = name
	}
	rec := slog.NewRecord(r.Timestamp(), levelOf(r.Severity()), msg, 0)
	if rec.Time.IsZero() {
		rec.Time = r.ObservedTimestamp()
	}
	r.WalkAttributes(func(kv attribute.KeyValue) bool {
		rec.AddAttrs(slog.String(string(kv.Key), kv.Value.Emit()))
		return true
	})
	return p.Handler.Handle(ctx, rec)
}

func (p Processor) Enabled(ctx context.Context, param sdklog.EnabledParameters) bool {
	return p.Handler.Enabled(ctx, levelOf(param.Severity))
}

func (Processor) Shutdown(context.Context) error   { return nil }
func (Processor) ForceFlush(context.Context) error { return nil }

func levelOf(s log.Severity) slog.Level {
	switch {
	case s >= log.SeverityError:
		return slog.LevelError
	case s >= log.SeverityWarn:
		return slog.LevelWarn
	case s >= log.SeverityInfo:
		return slog.LevelInfo
	default:
		return slog.LevelDebug
	}
}
```

`Handler` must be a plain handler, such as `slog.NewTextHandler`, built
without going through `otelslog`. Pointing it at `slog.Default()` would loop,
since `slog.SetDefault` above made the default handler `otelslog`.

## Parameters

| Param                          | Default                                                                            | Description                                                                                                                               |
| ------------------------------ | ---------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `root_package`                 | `""`                                                                               | Package name declared by the root `doc.go`. When empty, the last non-version element of `import_path`.                                    |
| `root_description`             | `""`                                                                               | Extra paragraph in the root `doc.go` comment. Omitted when empty.                                                                         |
| `import_path`                  | `example.com/semconv`                                                              | Import path of the output directory. Metric packages import attribute packages through it.                                                |
| `header_source`                | `""`                                                                               | What the code was generated from, shown in the `Code generated` header. Omitted when empty.                                               |
| `regenerate_command`           | `""`                                                                               | Command that regenerates the code, shown under the header. Omitted when empty.                                                            |
| `vendor_prefixes`              | `[]`                                                                               | Leading segments that are a vendor prefix rather than a namespace, such as `[acme]`.                                                      |
| `imported_import_paths`        | `{}`                                                                               | Attribute namespace to the Go import path of a dependency registry generated with this template, such as `{error: example.com/upstream}`. |
| `exclude_deprecated`           | `false`                                                                            | Leave deprecated attributes, metrics, spans, and events out.                                                                              |
| `stable_only`                  | `false`                                                                            | Generate only stable attributes, metrics, spans, and events.                                                                              |
| `histogram_boundaries_by_unit` | `{s: [0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1, 2.5, 5, 7.5, 10]}` | Default explicit bucket boundaries by unit, for a histogram that sets no `annotations.aggregation`. Set to `{}` to opt out.               |

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
- Two attributes of one metric, span, or event render to the same parameter or
  option name.
- A required attribute of a span or event renders to a parameter named like
  one its starter or `Emit` function already declares, such as an imported
  `tracer` or `logger`.
- An optional attribute of an event renders to an option named like its
  severity or timestamp option, such as `myapp.severity`.
- An attribute key or metric name has nothing left after its namespace.
- An enum mixes member value types, or `exclude_deprecated` or `stable_only`
  leaves it with no members.
- A metric, span, or event references an imported attribute whose type is not
  supported, or, for one mapped by `imported_import_paths`, an enum with no
  members to build a value from.
- A generated file would import two packages under the same Go package name
  from different import paths, whether both come from `imported_import_paths`
  or one is a local package. Generation fails rather than aliasing one, so an
  import always names the package it resolves to.
- `annotations.aggregation` is set on an instrument other than a histogram,
  uses a method or parameter other than `explicithistogram` and `boundaries`,
  or its boundaries are not a non-empty list of numbers in strictly increasing
  order.
- A histogram falls back to `histogram_boundaries_by_unit` for its unit, and
  that unit's boundaries are not a non-empty list of numbers in strictly
  increasing order.
- Two metrics or refinements in one package render to the same Go
  identifier, such as `myapp.task.started` and `myapp.task_started`, or the
  observable form of one is named like the other, such as
  `myapp.task.started` and `myapp.task.started.observable`.
- A metric refinement sets a different `code_generation.metric_value_type` or
  `annotations.aggregation` than the metric it refines.
- Two spans or refinements in one package render to the same Go identifier,
  such as `myapp.task.run` and `myapp.task_run`.
- Two events or refinements in one package render to the same Go identifier,
  such as `myapp.task.finished` and `myapp.task_finished`.
- Two entities or refinements in one package render to the same Go
  identifier, such as `myapp.host.worker` and `myapp.host_worker`.
- Two attributes of one entity's identity or description render to the same
  parameter or option name.
- A required attribute of an entity renders to a parameter named like one its
  constructor already declares.
- `exclude_deprecated` or `stable_only` keeps a metric, span, event, or entity
  but leaves out one of its required attributes.

## Tests

| Case                                  | Covers                                                                                                                                                                                        |
| ------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `attributes`                          | Every supported type, enums of every member type, notes, deprecated attributes and members.                                                                                                   |
| `metrics`                             | Every instrument, every requirement level, cross-namespace references, bucket boundaries.                                                                                                     |
| `imported`                            | Required and optional attributes imported from a dependency registry, including an enum, a span named from them, and an event.                                                                |
| `imported_typed`                      | An imported namespace mapped with `imported_import_paths`, taken as a typed value alongside a local attribute and an unmapped raw one, across a metric, a span named from them, and an event. |
| `params`                              | Custom root package, root description, import path, header, `vendor_prefixes`, and `exclude_deprecated`.                                                                                      |
| `root_package_from_import_path`       | The root package named after `import_path`.                                                                                                                                                   |
| `root_package_version_segment`        | A version element at the end of `import_path` skipped in the root package name.                                                                                                               |
| `spans`                               | Derived and typed span names, every kind of option, deprecation, refinements, cross-namespace references, and a tracer with no meter package.                                                 |
| `events`                              | Required and optional attributes, opt-in and deprecated events, a refinement, cross-namespace references, and a logger with no meter package.                                                 |
| `entities`                            | Required and optional identity and description attributes, a deprecated opt-in attribute, and a refinement that adds a required attribute.                                                    |
| `histogram_boundaries_by_unit`        | A histogram's unit resolving to default boundaries, a refinement keeping them, explicit boundaries overriding them, and a unit with none.                                                     |
| `histogram_boundaries_by_unit_params` | `histogram_boundaries_by_unit` replaced by a param, covering a custom unit and opting a unit out of the built-in default.                                                                     |
| `no_namespace`                        | A span, an event, and an entity whose name or type has a single segment, rendering unprefixed in their own package.                                                                           |
| `error_*`                             | Each case fails generation with one of the errors above.                                                                                                                                      |

Run them with `mise run weaver:test:templates`, and compile their expected
output with `mise run weaver:test:go`.
