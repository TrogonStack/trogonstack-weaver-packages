# Generation boundaries

Effect applications already own a metrics runtime. Generating `effect/Metric`
instruments keeps metric updates in that runtime while a registry governs
metric names, units, descriptions, and attribute contracts.

The registry belongs to the consuming application. The template contains no
application-specific metric names or business rules. Export endpoints,
resources, collection schedules, operation outcomes, and transaction-sensitive
recording remain application responsibilities.

Time histogram recorders convert an Effect duration into the registry's unit
before updating a numeric histogram. This
avoids assuming that every duration uses Effect timers' millisecond buckets.

Each metric exposes a thin recording function with friendly attribute fields.
The function owns telemetry keys and conversion to Effect's string labels;
generated enum constants own the registry's enum values. Private instruments
keep the typed function as the module's recording boundary.
Neither generation nor these types enforce label cardinality; the consuming
registry and application must choose bounded dimensions.

The generated module does not configure an OpenTelemetry SDK provider,
instrumentation scope, or schema URL in the exported OTLP payload. Those
capabilities belong to the consuming Effect exporter.
