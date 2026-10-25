# Generation boundaries

Effect applications already own a metrics runtime. Generating `effect/Metric`
instruments allows existing Effect combinators and OTLP configuration to keep
working while a registry governs metric names, units, descriptions, and
attribute contracts.

The registry belongs to the consuming application. The template contains no
application-specific metric names or business rules. Export endpoints,
resources, collection schedules, operation outcomes, and transaction-sensitive
recording remain application responsibilities.

Time histograms use an input mapping over a numeric histogram so the caller
passes an Effect duration while aggregation uses the registry's unit. This
avoids assuming that every duration uses Effect timers' millisecond buckets.

Exported native handles allow incremental adoption in applications with
existing instrumentation wrappers. Typed recording helpers provide a stricter
boundary for callers ready to adopt registry-defined attribute contracts.
Neither generation nor these types enforce label cardinality; the consuming
registry and application must choose bounded dimensions.

The generated module does not configure an OpenTelemetry SDK provider,
instrumentation scope, or schema URL in the exported OTLP payload. Those
capabilities belong to the consuming Effect exporter.
