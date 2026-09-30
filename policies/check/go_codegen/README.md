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

| ID                                       | Signal    | Fails when                                                                      |
| ---------------------------------------- | --------- | ------------------------------------------------------------------------------- |
| `go_codegen_unsupported_attribute_type`  | Attribute | The type is not a string, int, double, or boolean, an array of one, or an enum. |
| `go_codegen_attribute_without_namespace` | Attribute | The key has a single segment, so there is no package to put it in.              |
| `go_codegen_metric_without_namespace`    | Metric    | The name has a single segment, so there is no package to put it in.             |
| `go_codegen_invalid_metric_value_type`   | Metric    | `annotations.go.value_type` is missing or is not `int64` or `float64`.          |

Every finding is a `violation`. Metric findings set `signal_type` and
`signal_name`.

The template checks what depends on its params, such as identifier collisions
after `vendor_prefixes` is applied, at generation time, since a policy cannot
read template params.

## Tests

Run them with `mise run weaver:test:policies`, or a single case with
`mise run weaver:test:policies -- --test {name}`.
