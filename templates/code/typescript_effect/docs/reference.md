# Generated API and parameters

The output is `metrics.ts`. Names retain their full registry namespace:
`worker.task.duration` produces `updateWorkerTaskDuration` and, when it has
attributes, `WorkerTaskDurationAttributes`. Instruments stay private.
Metric names and attribute keys are preserved in recorded telemetry.

## Instruments

| Registry instrument                        | Effect instrument                          | Recording input     |
| ------------------------------------------ | ------------------------------------------ | ------------------- |
| `counter`                                  | `Metric.counter` with `incremental: true`  | `number`            |
| `updowncounter`                            | `Metric.counter` with `incremental: false` | `number`            |
| `gauge`                                    | `Metric.gauge`                             | `number`            |
| `histogram`, unit `s`, `ms`, `us`, or `ns` | `Metric.histogram`                         | `Duration.Duration` |
| Other `histogram`                          | `Metric.histogram`                         | `number`            |

Time histograms convert the supplied duration into their declared unit before
aggregation. Boundaries are expressed in that same unit. Units are carried in
Effect's `unit` attribute for its OTLP exporter.
Nanosecond-backed durations convert directly into the target unit, avoiding
an intermediate millisecond rounding step. Recorded values still have
JavaScript `number` precision.

`code_generation.metric_value_type` may be `int` or `double`; both use JavaScript
`number`. The template does not enforce integer values or exact 64-bit integer
representation.

## Attributes

Each helper takes a value followed by a readonly attribute object. Attributes
with `requirement_level: required` are mandatory. Other requirement levels
produce optional properties; conditional requirements remain the caller's
responsibility. When all attributes are optional, the object defaults to empty.
Metrics without attributes take only the measurement argument.

Attribute fields use camelCase identifiers. A matching leading metric namespace
is removed: `worker.task.id` becomes `taskId` on a `worker` metric. Attributes
from other namespaces retain theirs: `auth.method` becomes `authMethod`.
Colliding field names and fields that shadow JavaScript object-prototype members
fail generation.

Scalar strings, numbers, booleans, and enums are supported. Enum attributes
export a shared constant object and value type named after their full key:
`worker.result` becomes `WorkerResult`, with members such as
`WorkerResult.Success`. Callers can use those constants without spelling wire
values. Each enum is shared across metrics using that attribute.

Helpers pass string values through directly, convert numeric and boolean values
to string labels, and omit undefined optional attributes. Attribute arrays are
rejected. Keys `unit` and `time_unit` are
reserved by Effect and cannot be metric dimensions.
Attributes with the wire key `__proto__` are rejected during generation.

Update functions map friendly fields to the original telemetry keys. Their
attribute types enforce required fields and scalar or enum values. The module
exports no raw instrument handles that bypass this contract.

## Parameters

| Parameter                      | Default                                                | Meaning                                                                                              |
| ------------------------------ | ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------- |
| `exclude_deprecated`           | `false`                                                | Omit deprecated conventions.                                                                         |
| `stable_only`                  | `false`                                                | Keep stable conventions only.                                                                        |
| `histogram_boundaries`         | `[0, 1, 5, 10, 25, 50, 100, 250, 500, 1000]`           | Fallback boundaries for units absent from the unit map.                                              |
| `histogram_boundaries_by_unit` | Unit-specific boundaries for `s`, `ms`, `us`, and `ns` | Overrides the fallback for a matching unit; defaults are defined in [`weaver.yaml`](../weaver.yaml). |

Per-metric `annotations.aggregation` takes precedence over parameter defaults.
Only `method: explicithistogram` and `parameters.boundaries` are supported.
Boundaries must be a nonempty, strictly increasing list of finite numbers.
The generated instrument appends an infinite boundary for overflow observations.

Unsupported instruments, attribute types, metric refinements, identifier
collisions, and filtering that removes a required attribute fail generation.
Instrument identifiers `value`, `attributes`, and `labels` are reserved by
the generated helpers and fail generation.
Other signal kinds do not produce code in this metrics-only package.
