# Agent Guide for Check Policy Packages

This directory holds `weaver registry check` policy packages, used with
`weaver registry check -p {package}`.

## Package layout

- `*.rego`: policy logic.
- `README.md`: each finding ID and what triggers it.
- `tests/{case}/current/`: the registry to check.
- `tests/{case}/base/`: an optional baseline registry.
- `tests/{case}/expected-diagnostic-output.json`: the expected findings.

## Running tests

```bash
mise run weaver:test:policies
mise run weaver:test:policies -- --test {case}
mise run weaver:test:policies -- --test {case} --coverage
```

`--coverage` prints which lines of each `.rego` file ran. `--debug` traces the
harness script. Observed output lands in
`policies/check/{package}/observed-output/tests/{case}/`, which is ignored by
Git.

After an intended change, run `mise run weaver:test:update` and review the new
`expected-diagnostic-output.json` files.

## Rego style

- Read the version 2 resolved schema: `input.registry.attributes` and
  `input.registry.metrics`.
- When a finding is about a signal, set the top-level `signal_type` and
  `signal_name`. Keep `context` for details beyond the signal identity.
- Give every finding an `id` prefixed with the package name.
- To inspect input while debugging, add a temporary `deny` rule whose message
  prints the value, and remove it before committing.
