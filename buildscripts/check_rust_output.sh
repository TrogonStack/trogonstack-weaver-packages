#!/usr/bin/env bash

set -euo pipefail

OTEL_RUST_VERSION="${OTEL_RUST_VERSION:-0.33.0}"
WEAVER="${WEAVER:-weaver}"
ROOT="$(pwd)"
PACKAGE_DIR="${ROOT}/templates/code/rust"
TEMPLATES_ROOT="$(realpath "${PACKAGE_DIR}/../..")"

WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/weaver-rust.XXXXXX")"
trap 'rm -rf "${WORK_DIR}"' EXIT
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-${WORK_DIR}/target}"

for test_dir in "${PACKAGE_DIR}"/tests/*/; do
  test_name="$(basename "${test_dir}")"
  [[ -f "${test_dir}expected-error.txt" ]] && continue
  expected="${test_dir}expected"
  if [[ ! -f "${expected}/mod.rs" ]]; then
    echo "  FAIL: ${test_name} is missing expected/mod.rs."
    exit 1
  fi
  echo "-> Checking Rust [${test_name}] ..."

  crate_dir="${WORK_DIR}/${test_name}"
  mkdir -p "${crate_dir}/src"
  cp -R "${expected}/." "${crate_dir}/src/"
  mv "${crate_dir}/src/mod.rs" "${crate_dir}/src/lib.rs"
  if [[ -f "${test_dir}test.rs" ]]; then
    mkdir -p "${crate_dir}/tests"
    cp "${test_dir}test.rs" "${crate_dir}/tests/attributes.rs"
  fi
  if [[ -d "${test_dir}tests" ]]; then
    mkdir -p "${crate_dir}/tests"
    cp -R "${test_dir}tests/." "${crate_dir}/tests/"
  fi

  cat >"${crate_dir}/Cargo.toml" <<EOF
[package]
name = "generated_semconv"
version = "0.0.0"
edition = "2021"
publish = false

[dependencies]
opentelemetry = { version = "=${OTEL_RUST_VERSION}", features = ["logs", "metrics", "trace"] }
opentelemetry_sdk = { version = "=${OTEL_RUST_VERSION}", features = ["logs", "metrics", "trace", "testing"] }
smallvec = { version = "=1.16.2", features = ["const_generics"] }
EOF

  if [[ -f "${test_dir}dependency-params.yaml" ]]; then
    dependency_dir="${crate_dir}/upstream"
    mkdir -p "${dependency_dir}/src"
    (
      cd "${test_dir}"
      NO_COLOR=1 "${WEAVER}" registry generate -r dependency --v2 --quiet \
        --params dependency-params.yaml --templates="${TEMPLATES_ROOT}" \
        code/rust "${dependency_dir}/src"
    ) || { echo "  FAIL: ${test_name} could not generate its dependency."; exit 1; }
    mv "${dependency_dir}/src/mod.rs" "${dependency_dir}/src/lib.rs"
    cat >"${dependency_dir}/Cargo.toml" <<EOF
[package]
name = "upstream"
version = "0.0.0"
edition = "2021"
publish = false

[dependencies]
opentelemetry = "=${OTEL_RUST_VERSION}"
opentelemetry_sdk = "=${OTEL_RUST_VERSION}"
smallvec = { version = "=1.16.2", features = ["const_generics"] }
EOF
    printf '\nupstream = { path = "upstream" }\n' >>"${crate_dir}/Cargo.toml"
    (
      cd "${dependency_dir}"
      cargo fmt --all || exit 1
      cargo fmt --all --check || exit 1
      cargo clippy --all-targets -- -D warnings || exit 1
    ) || { echo "  FAIL: ${test_name} dependency fails Rust validation."; exit 1; }
  fi

  (
    cd "${crate_dir}"
    cargo fmt --all || exit 1
    cargo fmt --all --check || exit 1
    if [[ "${test_name}" == allocations || -f "${test_dir}tests/allocations.rs" ]]; then
      cargo test --release -- --nocapture || exit 1
    else
      cargo test || exit 1
    fi
    cargo clippy --all-targets -- -D warnings || exit 1
    for invalid_consumer in "${test_dir}"compile-fail/*.rs; do
      [[ -f "${invalid_consumer}" ]] || continue
      expected_error="${invalid_consumer%.rs}.expected-error.txt"
      if [[ ! -s "${expected_error}" ]]; then
        echo "  FAIL: ${invalid_consumer} is missing its expected diagnostic."
        exit 1
      fi
      mkdir -p examples
      cp "${invalid_consumer}" examples/invalid_consumer.rs
      if cargo check --example invalid_consumer >diagnostics.txt 2>&1; then
        echo "  FAIL: ${invalid_consumer} compiled successfully."
        exit 1
      fi
      if ! grep -Fq -f "${expected_error}" diagnostics.txt; then
        cat diagnostics.txt
        echo "  FAIL: ${invalid_consumer} did not fail with its expected diagnostic."
        exit 1
      fi
      rm examples/invalid_consumer.rs
      echo "  PASS: $(basename "${invalid_consumer}") rejects its invalid consumer."
    done
  ) || { echo "  FAIL: ${test_name} does not pass Rust validation."; exit 1; }
  echo "  PASS: ${test_name} formats, tests, and passes Clippy."
done
