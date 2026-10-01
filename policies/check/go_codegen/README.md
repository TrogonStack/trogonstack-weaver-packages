# Go Codegen Policy Package

Rejects registries that the [Go template package](../../../templates/code/go/README.md)
cannot render, so the problem is reported against the convention instead of as
Go that does not compile.

Stability: Development

## Usage

Run it on its own:

```bash
weaver registry check \
  --v2 \
  -r {your registry} \
  -p 'https://github.com/TrogonStack/trogonstack-weaver-packages.git@v{version}[policies/check/go_codegen]'
```

Or pass the same `-p` to `weaver registry generate`, which stops before writing
any file when a policy finds a violation.

## Rules

| ID                                         | Signal    | Fails when                                                                                                                                                                                                                                                                                                |
| ------------------------------------------ | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `go_codegen_invalid_schema_url`            | Registry  | The manifest declares no `schema_url`, or it does not end in the schema version, such as `https://example.com/schemas/1.0.0`.                                                                                                                                                                             |
| `go_codegen_unsupported_attribute_type`    | Attribute | The type is not a string, int, double, or boolean, an array of one, or an enum.                                                                                                                                                                                                                           |
| `go_codegen_mixed_enum_members`            | Attribute | The enum's members are not all strings, all numbers, or all booleans, so no single Go type can hold them.                                                                                                                                                                                                 |
| `go_codegen_attribute_without_namespace`   | Attribute | The key has a single segment, so there is no package to put it in.                                                                                                                                                                                                                                        |
| `go_codegen_metric_without_namespace`      | Metric    | The name has a single segment, so there is no package to put it in.                                                                                                                                                                                                                                       |
| `go_codegen_invalid_metric_value_type`     | Metric    | `annotations.code_generation.metric_value_type` is missing or is not `int` or `double`.                                                                                                                                                                                                                   |
| `go_codegen_invalid_aggregation`           | Metric    | `annotations.aggregation` is set on a non-histogram, uses anything but `method: explicithistogram` with `parameters.boundaries`, or the boundaries are not a non-empty list of numbers in strictly increasing order. Experimental, see [weaver#844](https://github.com/open-telemetry/weaver/issues/844). |
| `go_codegen_refinement_changes_instrument` | Metric    | A metric refinement sets a different `code_generation.metric_value_type` or `annotations.aggregation` than the metric it refines.                                                                                                                                                                         |

Every finding is a `violation`. Metric findings set `signal_type` and
`signal_name`.

The template checks what depends on its params, such as identifier collisions
after `vendor_prefixes` is applied, at generation time, since a policy cannot
read template params.

## Tests

Run them with `mise run weaver:test:policies`, or a single case with
`mise run weaver:test:policies -- --test {name}`.
