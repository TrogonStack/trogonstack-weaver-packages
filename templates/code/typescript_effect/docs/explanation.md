# Generation boundaries

Effect applications already own a metrics runtime. Generating `effect/Metric`
instruments keeps metric updates in that runtime while a registry governs
metric names, units, descriptions, and attribute contracts.

The registry belongs to the consuming application. The template contains no
application-specific metric names or business rules. Export endpoints,
resources, collection schedules, operation outcomes, and transaction-sensitive
recording remain application responsibilities.

Time histograms use `Metric.mapInput` to convert an Effect duration into the
registry's unit before aggregation. Update functions pass the duration directly
to the instrument. This avoids assuming that every duration uses Effect timers'
millisecond buckets.

Each metric exposes a thin update function with friendly attribute fields.
The function owns telemetry keys and conversion to Effect's string labels;
generated enum constants own the registry's enum values. Private instruments
keep the typed function as the module's recording boundary.
Neither generation nor these types enforce label cardinality; the consuming
registry and application must choose bounded dimensions.

Tracking wrappers derive `error.type` from the Effect exit because the exit is
the only place that knows how the work ended. The error's `_tag` matches the
attribute's intent in OpenTelemetry semantic conventions when it is a stable
class name. The wrapper records every string `_tag` as is, so keeping those tags
bounded is the application's responsibility, like any other dimension. Defects
and untagged failures fall back to `_OTHER`. Applications whose
notion of failure differs from the exit, such as a process that succeeds with
a non-zero status, keep timing themselves with the update helper.

The generated module does not configure an OpenTelemetry SDK provider,
instrumentation scope, or schema URL in the exported OTLP payload. Those
capabilities belong to the consuming Effect exporter.
