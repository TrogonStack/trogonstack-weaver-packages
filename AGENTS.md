# Agent Guide

This repository holds packages for
[OpenTelemetry Weaver](https://github.com/open-telemetry/weaver), split into
templates and policies.

## Project structure

- `templates/`: Jinja template packages, grouped by kind.
  - `templates/code/`: code generation packages.
- `policies/check/`: Rego policy packages for `weaver registry check`. See
  [policies/check/AGENTS.md](policies/check/AGENTS.md).
- `diagnostic_templates/`: renders policy findings for the policy tests.
- `buildscripts/`: the test harness.
- `.config/mise/tasks/`: mise tasks that wrap the harness.

## Tooling

Use the versions pinned in `mise.toml`, and run everything through mise:

- `mise run weaver:test` runs every test.
- `mise run weaver:test:update` rewrites expected output after an intended
  change. Review the diff before keeping it.
- `mise run markdown:fmt` formats Markdown.

Templates target the version 2 resolved schema and are run with `--v2`.

## Template packages

Each template package includes:

- `weaver.yaml`: params, templates, filters, and text maps.
- `*.j2`: Jinja templates.
- `README.md`: usage and params.
- `tests/{case}/registry/` and `tests/{case}/expected/`, plus an optional
  `tests/{case}/params.yaml`.

Generated Go must pass `gofmt`, `go build`, and `go vet`, which
`mise run weaver:test:go` checks.

Generated Rust must pass formatting, consumer tests, and Clippy, which
`mise run weaver:test:rust` checks against the pinned OpenTelemetry Rust API
and SDK.

## Contribution guidelines

- Keep changes to the package being modified.
- Every package has a `README.md`.
- Every param has a default that works for any registry. Do not encode a
  particular product, vendor, or domain in a package.
- Keep test registries synthetic.
- Pull request titles follow Conventional Commits. See
  [CONTRIBUTING.md](CONTRIBUTING.md).
