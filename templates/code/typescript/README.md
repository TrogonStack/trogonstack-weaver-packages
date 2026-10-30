# TypeScript Template Package

Generates typed attributes and metric instruments on the vanilla
`@opentelemetry/api` from a Weaver version 2 semantic convention registry. The
generated code does not depend on Effect or on any SDK package.

Stability: Development. Generated code is tested against `@opentelemetry/api`
1.9.1 and `@opentelemetry/sdk-metrics` 2.12.0, and it needs
`@opentelemetry/api` `>=1.9.0 <1.10.0`.

Only attributes and metrics are rendered. Spans, events, and entities in the
registry are skipped, and they do not fail generation. For Effect, see the
[Effect TypeScript metrics](../typescript_effect/README.md) package.

## Generate and use a module

From a checkout of this repository:

```sh
weaver registry generate \
  --v2 \
  -r {your registry} \
  -t templates \
  code/typescript \
  {your app}/src/semconv
```

For a released package, replace `-t templates` with
`-t 'https://github.com/TrogonStack/trogonstack-weaver-packages.git@v{version}[templates]'`.
The consuming application needs `@opentelemetry/api`. The generated code uses
only erasable TypeScript syntax and `.ts` import specifiers, so Node.js can run
it with type stripping and `tsc` accepts it with `allowImportingTsExtensions`.
Weaver writes only the files it produces, so empty the generated directory
before regenerating after removing conventions.

```ts
import { metrics } from "@opentelemetry/api";
import { MyappTaskState } from "./semconv/attributes.ts";
import {
  getMeter,
  MyappTaskDuration,
  MyappTaskStarted,
} from "./semconv/metrics.ts";

const meter = getMeter(metrics.getMeterProvider());

const started = new MyappTaskStarted(meter);
started.add(1, { taskId: "task-1" });

const duration = new MyappTaskDuration(meter);
duration.record(0.42, { taskId: "task-1", taskState: MyappTaskState.Done });
```

## Output layout

| File            | Content                                                                          |
| --------------- | -------------------------------------------------------------------------------- |
| `schema.ts`     | `SCHEMA_URL`, the schema URL of the registry. Always generated.                  |
| `attributes.ts` | One constant and one type per attribute. Generated when attributes are rendered. |
| `metrics.ts`    | `getMeter`, and a class per metric. Generated when the registry holds a metric.  |

The three modules keep the dependencies one way. `metrics.ts` imports the
attribute types from `attributes.ts` with `import type`, so code that only
names an attribute does not load the OpenTelemetry API. `schema.ts` holds one
constant, so a tool that only needs the schema URL can import it alone. An
empty registry produces only `schema.ts`, and a registry without metrics
produces no `metrics.ts`.

Every top-level statement in the three modules is free of side effects, so a
bundler drops each class, function, and constant that the application does not
import. The test suite holds that line for esbuild, which is stricter about it
than Rollup. The classes use no static fields, static blocks, or `#private`
fields, because a compiler that targets a release before ES2022 turns them into
top-level statements, and `#private` fields into a `WeakMap` lookup on every
call.

Each name starts with the registry name in PascalCase after the first vendor
prefix in `vendor_prefixes` is removed. For example, `myapp.task.id` becomes
`MyappTaskId`, and with `vendor_prefixes: [myapp]` it becomes `TaskId`.

## Generated API

### Attributes

Each attribute becomes a frozen object and a type of the same name. The type is
the value type of the attribute, a literal union for an enum, or `readonly T[]`
for an array. The object carries `KEY`, the wire name, and one member per enum
value:

```ts
export const AuthMethod = /* @__PURE__ */ Object.freeze({
  KEY: "auth.method",
  Password: "password",
  Token: "token",
} as const);

export type AuthMethod = "password" | "token";
```

Supported types are `string`, `int`, `double`, `boolean`, their array forms, and
enums with string, integer, double, or boolean members. Attributes of type
`template` or `any` fail generation, because they have no fixed value type.

An attribute imported from a dependency registry is inlined as a type and has
no constant, because this package does not know where the dependency lives.
Enums from a dependency become literal unions.

### Metrics

`getMeter` returns the `Meter` that creates the instruments. It calls the
`getMeter` of the provider with `instrumentation_name`, the optional
`instrumentation_version`, and the registry schema URL. The provider argument
defaults to the global meter provider. `Meter` is an OpenTelemetry meter with a
type-only brand, so an instrument does not accept a meter of another scope.

Each metric becomes a class with a static `NAME`, the name it records under. A
histogram has `record(value, attributes)`. A counter and an up-down counter have
`add(value, attributes)`. A gauge has `record(value, attributes)`.

```ts
export class AuthAttempts {
  static get NAME(): "auth.attempts";

  constructor(meter: Meter);
  add(value: number, attributes: AuthAttemptsAttributes): void;
}
```

Each metric has its own attribute type. A required attribute is a required
property, and any other attribute is an optional property, which also accepts
`undefined`. A metric without a required attribute defaults the argument to
`{}`, so `queueDepth.record(3)` is valid. Attributes are copied into a plain
object, including array values, so later changes by the caller do not reach the
SDK.

Every metric except a histogram also gets an observable form: a class named
like the metric with an `Observable` suffix. Its constructor takes the `Meter`
and a callback that receives an observer. The callback may be asynchronous.
`stop()` removes the callback from the SDK.

```ts
const depth = new MyappQueueDepthObservable(meter, (observer) => {
  observer.observe(queue.length);
});
depth.stop();
```

A metric refinement gets its own class and attribute type, and it records into
the instrument of its base metric. Both classes therefore write one time
series, and the refinement only narrows the attributes. A refinement must keep
the value type and the aggregation of its base.

`metric_value_type` is optional in the registry. When it is set, the generator
passes `valueType` to the instrument. When it is unset, the SDK default applies.
The generator does not check that an `int` metric receives an integer, and it
does not reject a negative counter value. The OpenTelemetry SDK does both.

### Histogram boundaries

A histogram with the `annotations.aggregation` annotation passes its boundaries
to the instrument as `advice.explicitBucketBoundaries`:

```yaml
annotations:
  aggregation:
    method: explicithistogram
    parameters:
      boundaries: [0.1, 1, 10]
```

Boundaries must be a non-empty list of numbers in strictly increasing order.
The annotation is valid only on a histogram, and `method` and `parameters` are
the only fields it accepts. A histogram without the annotation takes the
boundaries that `histogram_boundaries_by_unit` lists for its unit. A unit
missing from that map leaves the SDK default.

## Parameters

Set parameters with `--param key=value` or with `--params {file}`.

| Parameter                      | Default             | Purpose                                                                                     |
| ------------------------------ | ------------------- | ------------------------------------------------------------------------------------------- |
| `instrumentation_name`         | `generated-semconv` | Instrumentation scope name passed to `getMeter`. Must be a nonempty string.                 |
| `instrumentation_version`      | `""`                | Instrumentation scope version. An empty value passes no version.                            |
| `header_source`                | `""`                | Names what the code was generated from, in a `// Source:` header line. Omitted when empty.  |
| `regenerate_command`           | `""`                | The command that refreshes the code, in a `// Regenerate:` header line. Omitted when empty. |
| `vendor_prefixes`              | `[]`                | Leading name segments that are a vendor and not a namespace.                                |
| `exclude_deprecated`           | `false`             | Leaves out deprecated attributes and metrics.                                               |
| `stable_only`                  | `false`             | Renders only stable attributes and metrics.                                                 |
| `histogram_boundaries_by_unit` | boundaries for `s`  | Maps a unit to default boundaries for histograms without the aggregation annotation.        |

The default boundaries for `s` are the ones the OpenTelemetry semantic
conventions advise for a duration histogram in seconds. Set
`histogram_boundaries_by_unit` to `{}` to leave every histogram on the SDK
default.

## Generation errors

Generation fails with a message that names the convention, and it never emits
code that does not compile. The failures are:

- an `instrumentation_name`, `instrumentation_version`, or `vendor_prefixes`
  value of the wrong type
- a registry schema URL that does not name a version
- an unsupported attribute type or instrument, or an unusable `metric_value_type`
- two attributes or metrics that render to one name, one object field, or one
  identifier, and a name that clashes with a generated name such as `Meter`
- an attribute key that is `__proto__` or a field of `Object.prototype`
- an enum with colliding members, members that cannot be identifiers, mixed
  value types, or no members left after filtering
- a required attribute that filtering removed
- an invalid aggregation annotation, or a refinement whose value type or
  aggregation differs from its base

## Tests

Each directory under `tests/` holds a registry, optional `params.yaml`, and the
expected output, or an `expected-error.txt` for a failing case. `mise run
weaver:test:update` regenerates the expected files.

`mise run weaver:test:typescript` installs the locked dependencies from
`buildscripts/typescript/` in a temporary directory. It type-checks every
expected module with strict compiler options, including the `@ts-expect-error`
lines that prove the types reject wrong input. It then runs each
`consumer.test.ts` with `node --test` against an in-memory metric reader, so the
tests assert the names, units, scope, schema URL, attributes, boundaries, and
observable callbacks that the SDK exports.
