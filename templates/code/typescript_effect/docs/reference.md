# Generated API and parameters

The output is `metrics.ts`. Names retain their full registry namespace:
`worker.task.duration` produces `workerTaskDuration`,
`WorkerTaskDurationAttributes`, and `recordWorkerTaskDuration`.
Metric names and attribute keys are preserved in recorded telemetry.

## Instruments

| Registry instrument                        | Effect instrument                          | Recording input     |
| ------------------------------------------ | ------------------------------------------ | ------------------- |
| `counter`                                  | `Metric.counter` with `incremental: true`  | `number`            |
| `updowncounter`                            | `Metric.counter` with `incremental: false` | `number`            |
| `gauge`                                    | `Metric.gauge`                             | `number`            |
| `histogram`, unit `s`, `ms`, `us`, or `ns` | Duration-mapped `Metric.histogram`         | `Duration.Duration` |
| Other `histogram`                          | `Metric.histogram`                         | `number`            |

Time histograms convert the supplied duration into their declared unit before
aggregation. Boundaries are expressed in that same unit. Units are carried in
Effect's `unit` attribute for its OTLP exporter.

`code_generation.metric_value_type` may be `int` or `double`; both use JavaScript
`number`. The template does not enforce integer values or exact 64-bit integer
representation.

## Attributes

Each helper takes a value followed by a readonly attribute object. Attributes
with `requirement_level: required` are mandatory. Other requirement levels
produce optional properties; conditional requirements remain the caller's
responsibility. When no attribute is required, the object defaults to empty.

Scalar strings, numbers, booleans, and enum literal unions are supported.
Helpers convert supplied values to string labels and omit absent optional
attributes. Attribute arrays are rejected. Keys `unit` and `time_unit` are
reserved by Effect and cannot be metric dimensions.

Native exported handles remain compatible with Effect combinators. Updating a
handle directly bypasses the generated attribute contract. Use the recording
helpers where compile-time attribute validation is needed.

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
Other signal kinds do not produce code in this metrics-only package.
