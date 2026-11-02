# k6 Metrics Template Package

Generates plain JavaScript helpers for native `k6/metrics`, with adjacent
TypeScript declarations. Stability: Development. The generated output is
verified against k6 2.3.0. It needs no OpenTelemetry SDK or bundler.

Only metrics and their tags are rendered. Other signals and unreferenced
attributes are ignored. Imported attributes are resolved into the metric's tag
type and validators.

## Generate and use helpers

From a checkout of this repository:

```sh
weaver registry generate \
  --v2 \
  -r {your registry} \
  -t templates \
  code/k6 \
  {your test}/semconv
```

For a released package, replace `-t templates` with
`-t 'https://github.com/TrogonStack/trogonstack-weaver-packages.git@v{version}[templates]'`.
Weaver writes only the files it produces. Empty the output directory before
regenerating after removing conventions.

A synthetic registry metric can select its native wire name explicitly:

```yaml
metrics:
  - name: sample.duration
    instrument: histogram
    unit: ms
    stability: stable
    requirement_level: recommended
    brief: Duration of a synthetic operation.
    annotations:
      k6:
        name: sample_duration
```

```js
import {
  createSampleDuration,
  METRIC_SAMPLE_DURATION,
} from "./semconv/metrics.js";

const duration = createSampleDuration();

export const options = {
  thresholds: { [METRIC_SAMPLE_DURATION]: ["p(95)<250"] },
};

export default function () {
  duration.record(125);
}
```

Call factories in the script's init context. Native k6 constructors reject
calls during a VU iteration. Recording must happen outside init context, as
[native custom metrics require](https://grafana.com/docs/k6/latest/using-k6/metrics/create-custom-metrics/).
A factory returns a frozen helper with a private metric identity. Its native
handle remains in a closure. Helper types distinguish raw native instruments,
different metrics, and refinements.
There is no raw instrument or unrestricted tag argument.

The `.d.ts` files describe the `.js` imports for TypeScript and JavaScript editor
tooling. Use TypeScript `strict` checking, or `checkJs` in JavaScript consumers,
for required tags and literal enum checks. Runtime validation also protects
unchecked JavaScript calls.

## Output and API reference

| File           | Content                                                             |
| -------------- | ------------------------------------------------------------------- |
| `schema.js`    | `SCHEMA_URL`, informational registry provenance. Always generated.  |
| `schema.d.ts`  | The literal type of `SCHEMA_URL`.                                   |
| `metrics.js`   | `METRIC_*` native name constants and `create<Metric>()` factories.  |
| `metrics.d.ts` | Readonly helper interfaces and required or optional exact tag keys. |

A registry without selected metrics produces only the schema files. k6 has no
OpenTelemetry meter, provider, instrumentation scope, or schema binding API.
`SCHEMA_URL` is local metadata. It is not bound to native metrics or transported
as an OpenTelemetry schema URL.

Names use PascalCase for helper types and factories, and uppercase underscores
for `METRIC_*` constants. Dots and underscores separate identifier words.
`vendor_prefixes` removes a matching leading dot-separated vendor segment from
identifiers. Wire names and tag keys remain unchanged. Identifier and wire-name
collisions fail generation.

| Registry instrument | Native kind | Helper method         |
| ------------------- | ----------- | --------------------- |
| `counter`           | `Counter`   | `add(value, tags)`    |
| `gauge`             | `Gauge`     | `record(value, tags)` |
| `histogram`         | `Trend`     | `record(value, tags)` |

Native Trend reports statistics and percentiles. It does not preserve histogram
buckets. Any explicit `annotations.aggregation` fails generation. Up-down
counters and other instruments also fail because no faithful native mapping is
available. Native kind semantics are documented by
[k6/metrics](https://grafana.com/docs/k6/latest/javascript-api/k6-metrics/).

### Native annotations

`annotations.k6` accepts only these fields:

| Field     | Default                       | Meaning                                               |
| --------- | ----------------------------- | ----------------------------------------------------- |
| `name`    | The registry metric name      | Explicit native wire name. Never sanitized.           |
| `kind`    | The mapping above             | `counter`, `gauge`, `trend`, or explicit `rate`.      |
| `is_time` | `true` for `ms`, else `false` | Native time formatting. Never converts sample values. |

The pinned k6 [name validator](https://github.com/grafana/k6/blob/v2.3.0/metrics/registry.go)
requires `^[a-zA-Z_][a-zA-Z0-9_]{1,128}$`. This accepts names from 2 through 129
characters. Dotted registry names need an explicit legal `name`. The built-in
names from k6 2.3.0's
[metric registry](https://github.com/grafana/k6/blob/v2.3.0/metrics/builtin.go)
are reserved to prevent mixing custom and runtime samples. Avoid collisions with
metrics registered by other scripts or extensions too. k6 shares its registry
by native name across modules.

Only a dimensionless `gauge` with unit `1` may explicitly select `kind: rate`.
This is a deliberate change to native observation semantics. A
[Rate](https://grafana.com/docs/k6/latest/javascript-api/k6-metrics/rate/)
reports the fraction of added samples that are nonzero. For example, adding `0`
and `2` yields `0.5`. It does not sum to `2`. The helper's `add` accepts numbers
and booleans. Other explicit kinds must match their registry instrument.

Native [time thresholds](https://grafana.com/docs/k6/latest/javascript-api/k6-metrics/trend/)
use milliseconds. `is_time: true` requires unit `ms` and cannot apply to Rate.
The untyped units `1`, an empty string, and count annotations such as `{item}`
use no native time formatting by default. Other units require explicit
`is_time: false`. For example, a seconds histogram can retain `0.25` as an
untyped Trend sample with that setting. It is never relabeled or converted to
milliseconds. Native k6 does not transport arbitrary registry units.

A metric refinement gets its own factory, tag type, and name constant. It
records into the base metric's native name. It must preserve the base
instrument, unit, and annotations. A filtered base also filters its refinements.

### Values and tags

Metric values must be finite numbers. `code_generation.metric_value_type: int`
requires a safe integer within `Number.MAX_SAFE_INTEGER` in absolute value.
The default is `double`, which preserves finite JavaScript floating-point values.
This API does not represent the full signed 64-bit integer range exactly. Counters reject negative values.
Rates additionally accept booleans as native zero or nonzero observations.

Tags use exact quoted wire keys. Required attributes are required properties.
Other attributes are optional. An absent or `undefined` optional tag is omitted.
Required tags cannot disappear through filtering. A metric without required
tags accepts an omitted tag object. A metric without tags accepts only an empty
object or an omitted argument.

Supported tag types are `string`, `int`, `double`, `boolean`, and homogeneous
string, numeric, or boolean enums. Enums retain their literal value kinds and
filter their members. Numeric tags must be finite. Integer tags and integer enum literals must be
safe integers. Arrays, other types, mixed enums,
empty filtered enums, and unsafe numeric enum literals fail generation.

Native k6 tags are strings. After validation, numeric and boolean tags use the
canonical JavaScript `String(value)` representation. Strings remain unchanged.
Objects and arrays never receive string coercion. Unknown keys, accessor
properties, missing required values, and wrong scalar or enum kinds fail before
native recording. Plain objects with either the standard or null prototype are
accepted. k6 may also add its configured global and system tags.

## Parameters

Set params with `--param key=value` or a YAML `params:` object through `--params`.

| Parameter            | Default | Purpose                                                       |
| -------------------- | ------- | ------------------------------------------------------------- |
| `header_source`      | `""`    | Source text in a JSON-escaped single-line generated comment.  |
| `regenerate_command` | `""`    | Regeneration instruction in the same escaped comment format.  |
| `vendor_prefixes`    | `[]`    | Leading vendor segments removed from helper identifiers only. |
| `exclude_deprecated` | `false` | Omit deprecated metrics, attributes, and enum members.        |
| `stable_only`        | `false` | Select stable metrics, attributes, and enum members.          |

## Verification

`mise run weaver:test:templates` compares generated files with exact expected
output and checks negative diagnostics. `mise run weaver:test:k6` installs the
locked TypeScript test dependencies, checks positive and negative type controls,
and executes every generated fixture with real k6 2.3.0. Each consumer uses one
VU and one iteration, no HTTP calls, and disabled usage reporting. System tags
are disabled so assertions compare complete native tag objects. Mutation controls
verify that unexpected tags fail for tagged and untagged metrics. Assertions
also read native JSON samples and summaries for names, kinds, values, time flags,
and thresholds. A failing threshold must produce exit status 99.
