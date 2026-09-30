# TrogonStack Weaver Packages

Template and policy packages for
[OpenTelemetry Weaver](https://github.com/open-telemetry/weaver), the tool that
validates semantic convention registries and generates code and docs from them.

The layout and test harness follow
[opentelemetry-weaver-packages](https://github.com/open-telemetry/opentelemetry-weaver-packages),
so a package from either repository is consumed the same way.

## Packages

### Templates

| Package                        | Description                                                             | Stability   |
| ------------------------------ | ----------------------------------------------------------------------- | ----------- |
| [`code/go`](templates/code/go) | Typed Go attributes and metric instruments on the OpenTelemetry Go API. | Development |

### Policies

| Package                                         | Description                                              | Stability   |
| ----------------------------------------------- | -------------------------------------------------------- | ----------- |
| [`check/go_codegen`](policies/check/go_codegen) | Rejects registries the `code/go` template cannot render. | Development |

## Using a package

Weaver accepts a Git repository as a template or policy location, in the form
`{url}.git@{ref}[{path}]`. Point it at a release tag, `v{version}`, so the
package does not change under you:

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

The `-t` path is the `templates` directory and the positional `code/go` selects
the package inside it. Package params can be set with `--param key=value` or a
YAML file passed to `--params`. Each package README lists its params.

Releases and their tags are listed on the
[releases page](https://github.com/TrogonStack/trogonstack-weaver-packages/releases).

## Development

Tools are pinned in [`mise.toml`](mise.toml). With [mise](https://mise.jdx.dev)
installed:

```bash
mise install
mise run weaver:test
```

| Task                    | What it does                                                                           |
| ----------------------- | -------------------------------------------------------------------------------------- |
| `weaver:test`           | Runs every task below except `weaver:test:update`.                                     |
| `weaver:test:policies`  | Checks each policy test registry against its expected diagnostics.                     |
| `weaver:test:templates` | Generates each template test registry and diffs it against its expected output.        |
| `weaver:test:go`        | Formats, builds, and vets the expected Go output. Needs network access for Go modules. |
| `weaver:test:update`    | Rewrites every expected output from the current packages.                              |
| `markdown:fmt`          | Formats Markdown with Prettier.                                                        |
| `markdown:check`        | Checks Markdown formatting.                                                            |

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to add a package.

## License

[Apache-2.0](LICENSE). The test harness in [`buildscripts`](buildscripts) and
the [`diagnostic_templates`](diagnostic_templates) are adapted from
opentelemetry-weaver-packages, also Apache-2.0.
