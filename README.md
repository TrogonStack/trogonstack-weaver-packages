# TrogonStack Weaver Packages

**Reusable template and policy packages for
[OpenTelemetry Weaver](https://github.com/open-telemetry/weaver).** Each package
is versioned and released from this repository, ready to be referenced by tag.

**The packages turn a semantic convention registry into code and keep that
registry fit for generation.** Templates generate typed telemetry helpers, such
as Go attribute values and metric instruments on the OpenTelemetry Go API.
Policies run during `weaver registry check` and reject conventions a template
cannot render, before any code is written.

**Telemetry written against hand-typed string keys drifts from the conventions
it is meant to follow.** Generating it from the registry makes the conventions
the single source of truth, and sharing the templates means every repository
gets the same generated shape instead of maintaining its own copy. The layout
and test harness follow
[opentelemetry-weaver-packages](https://github.com/open-telemetry/opentelemetry-weaver-packages),
so a package from either repository is consumed the same way.

**These packages are for teams that define their own semantic conventions and
instrument services with OpenTelemetry.** They suit platform and observability
engineers who own a registry, and service developers who want compile-time
checked telemetry instead of stringly typed calls.

## Packages

Templates live under [`templates`](templates) and policies under
[`policies`](policies). Each package directory has a README describing what it
does and the params it accepts.

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

Run `mise tasks` to list every task and what it does.

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to add a package.
