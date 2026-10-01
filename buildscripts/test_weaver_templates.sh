#!/usr/bin/env bash
# Adapted from opentelemetry-weaver-packages (Apache-2.0):
# https://github.com/open-telemetry/opentelemetry-weaver-packages/blob/main/buildscripts/test_weaver_templates.sh

set -uo pipefail

WEAVER="${WEAVER:-weaver}"
UPDATE_EXPECTED="${UPDATE_EXPECTED:-}"

if ! command -v "${WEAVER}" >/dev/null 2>&1; then
  echo "weaver not found. Set the WEAVER environment variable or add it to your PATH."
  exit 1
fi

log_err() {
  echo "$1"
  exit 1
}

# With UPDATE_EXPECTED=1 the observed output replaces the expected one instead
# of being diffed against it.
check_output() {
  local observed="$1"
  local expected="$2"
  local test_name="$3"
  if [[ -n "${UPDATE_EXPECTED}" ]]; then
    rm -rf "${expected}"
    cp -R "${observed}" "${expected}"
    echo "  UPDATED: ${test_name} expected output."
    return 0
  fi
  if [[ -e "${expected}" ]]; then
    if diff -r "${observed}" "${expected}" >/dev/null; then
      echo "  PASS: ${test_name} matches expected output."
    else
      echo "  FAIL: ${test_name} differences found!"
      diff -r -u "${expected}" "${observed}"
      exit 1
    fi
  else
    echo "  SKIPPED: Missing expected file or directory: ${expected}"
  fi
}

# The test directory is the working directory so that file provenance stays
# relative and weaver discovers a test's own `.weaver.toml`.
run_generate_test() {
  local test_dir="$1"
  local package_dir="$2"
  local observed_dir="$3"
  local templates_root
  templates_root="$(realpath "${package_dir}/../..")"
  local target="${package_dir#"${templates_root}"/}"
  local params_arg=()
  if [[ -f "${test_dir}/params.yaml" ]]; then
    params_arg=(--params "${test_dir}/params.yaml")
  fi
  (
    cd "${test_dir}" || exit 1
    NO_COLOR=1 "${WEAVER}" registry generate \
      -r registry \
      --v2 \
      --quiet \
      ${params_arg[@]+"${params_arg[@]}"} \
      --templates="${templates_root}" \
      "${target}" \
      "${observed_dir}"
  ) || log_err "  FAIL: weaver registry generate exited with an error."
}

# A case with `expected-error.txt` instead of `expected/` passes when generation
# fails and one of its diagnostics contains the file's text.
run_error_test() {
  local test_dir="$1"
  local package_dir="$2"
  local observed_dir="$3"
  local test_name="$4"
  local templates_root
  templates_root="$(realpath "${package_dir}/../..")"
  local target="${package_dir#"${templates_root}"/}"
  local params_arg=()
  if [[ -f "${test_dir}/params.yaml" ]]; then
    params_arg=(--params "${test_dir}/params.yaml")
  fi
  local diagnostics
  if diagnostics="$(
    cd "${test_dir}" || exit 1
    NO_COLOR=1 "${WEAVER}" registry generate \
      -r registry \
      --v2 \
      --quiet \
      --diagnostic-format json \
      ${params_arg[@]+"${params_arg[@]}"} \
      --templates="${templates_root}" \
      "${target}" \
      "${observed_dir}" 2>&1
  )"; then
    log_err "  FAIL: ${test_name} generated output, but it must fail."
  fi
  local expected
  expected="$(<"${test_dir}/expected-error.txt")"
  if jq -e --arg expected "${expected}" 'any(.[]; .diagnostic.message | contains($expected))' <<<"${diagnostics}" >/dev/null 2>&1; then
    echo "  PASS: ${test_name} fails with the expected error."
  else
    echo "  FAIL: ${test_name} does not fail with: ${expected}"
    echo "${diagnostics}"
    exit 1
  fi
}

run_template_test() {
  local test_dir="$1"
  local package_dir="$2"
  local test_name="${test_dir#"${package_dir}"/}"
  echo "-> Running test [${test_name}] ..."
  local observed_dir="${package_dir}/observed-output/${test_name}"
  rm -rf "${observed_dir}"
  mkdir -p "${observed_dir}"
  if [[ -f "${test_dir}/expected-error.txt" ]]; then
    run_error_test "${test_dir}" "${package_dir}" "${observed_dir}" "${test_name}"
    return
  fi
  run_generate_test "${test_dir}" "${package_dir}" "${observed_dir}"
  check_output "${observed_dir}" "${test_dir}/expected" "${test_name} - Template Output"
}

run_tests() {
  local package_dir="$1"
  local tests_dir="${package_dir}/tests"
  [[ -d "${tests_dir}" ]] || log_err "Error: Tests not found in '${tests_dir}' for template package: ${package_dir}"
  for dir in "${tests_dir}"/*; do
    [[ -d "${dir}" ]] && run_template_test "${dir}" "${package_dir}"
  done
}

run_all_template_tests() {
  local root="$1"
  for package in "${root}"/templates/*/*; do
    [[ -d "${package}" ]] || continue
    echo "---==== Template Package - ${package#"${root}"/templates/} ====---"
    [[ -f "${package}/README.md" ]] || echo "Missing README"
    if [[ -d "${package}/tests" ]]; then
      run_tests "${package}"
    else
      echo "SKIPPED TESTS: No tests directory"
    fi
  done
}

# Run from inside a package to test only that package.
if [[ -d "tests" ]]; then
  echo "Running tests for ${PWD}..."
  run_tests "${PWD}"
else
  run_all_template_tests "${PWD}"
fi
