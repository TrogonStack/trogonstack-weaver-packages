#!/usr/bin/env bash
# Adapted from opentelemetry-weaver-packages (Apache-2.0):
# https://github.com/open-telemetry/opentelemetry-weaver-packages/blob/main/buildscripts/test_weaver_policies.sh

set -uo pipefail

test_filter=()
DEBUG=false
COVERAGE=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --debug)
      DEBUG=true
      shift
      ;;
    --coverage)
      COVERAGE=true
      shift
      ;;
    --test)
      if [[ -n "${2:-}" && "$2" != --* ]]; then
        test_filter+=("$2")
        shift 2
      else
        echo "Error: --test requires an argument."
        exit 1
      fi
      ;;
    -h | --help)
      echo "Usage: test_weaver_policies.sh [--debug] [--coverage] [--test {name}]"
      exit 0
      ;;
    *)
      echo "Invalid option: $1" >&2
      exit 1
      ;;
  esac
done

[[ "${DEBUG}" == "true" ]] && set -x

WEAVER="${WEAVER:-weaver}"
UPDATE_EXPECTED="${UPDATE_EXPECTED:-}"
ROOT="$(pwd)"

if ! command -v "${WEAVER}" >/dev/null 2>&1; then
  echo "weaver not found. Set the WEAVER environment variable or add it to your PATH."
  exit 1
fi

log_err() {
  echo "$1"
  exit 1
}

matches_test_filter() {
  local name="$1"
  [[ ${#test_filter[@]} -eq 0 ]] && return 0
  local e
  for e in "${test_filter[@]}"; do
    [[ "$e" == "$name" ]] && return 0
  done
  return 1
}

# With UPDATE_EXPECTED=1 the observed output replaces the expected one instead
# of being diffed against it. Weaver's unstable-format warnings are dropped from the observed output, and
# an empty or missing context is normalized away, so expected files stay terse.
check_output() {
  local observed="$1"
  local expected="$2"
  local test_name="$3"
  local version_filter='map(select(.diagnostic.message | contains("is not yet stable") | not))'
  if [[ -n "${UPDATE_EXPECTED}" ]]; then
    jq -S "${version_filter} | sort_by(.diagnostic.message)" "${observed}" >"${expected}"
    echo "  UPDATED: ${test_name} expected output."
    return
  fi
  if [[ ! -f "${expected}" ]]; then
    echo "  SKIPPED: Missing expected file: ${expected}"
    return
  fi
  local normalize_filter='
    map(
      if (.error.violation | (has("context") | not))
          or .error.violation.context == {}
          or .error.violation.context == null
      then
        del(.error.violation.context) |
        .diagnostic.message |= gsub(", context=\\{}"; "")
      else . end
    )
  '
  local tmp_observed tmp_expected
  tmp_observed="$(mktemp)"
  tmp_expected="$(mktemp)"
  jq -S "${version_filter} | ${normalize_filter} | sort_by(.diagnostic.message)" "${observed}" >"${tmp_observed}"
  jq -S "${normalize_filter} | sort_by(.diagnostic.message)" "${expected}" >"${tmp_expected}"
  if diff "${tmp_observed}" "${tmp_expected}" >/dev/null; then
    echo "  PASS: ${test_name} matches expected output."
    rm -f "${tmp_observed}" "${tmp_expected}"
  else
    echo "  FAIL: ${test_name} differences found!"
    diff -u "${tmp_expected}" "${tmp_observed}"
    rm -f "${tmp_observed}" "${tmp_expected}"
    exit 1
  fi
}

# Signal identity belongs in the top-level signal_type and signal_name fields,
# never in context.
validate_signal_conventions() {
  local observed="$1"
  local test_name="$2"
  [[ -f "${observed}" ]] || return

  local violations
  violations="$(jq -r --arg pattern '^(metric|event|span|entity)[_.](name|type)$' '
    [.[] |
      select(.error.violation.context != null) |
      . as $finding |
      (.error.violation.context | keys | map(select(test($pattern)))) as $bad_keys |
      select($bad_keys | length > 0) |
      "  - id=\($finding.error.violation.id), forbidden context keys=\($bad_keys)"
    ] | .[]
  ' "${observed}")"
  if [[ -n "${violations}" ]]; then
    echo "  FAIL: ${test_name} - signal identifiers found in context (use top-level signal_type/signal_name instead):"
    echo "${violations}"
    exit 1
  fi

  local missing
  missing="$(jq -r '
    [.[] |
      select(.error.violation.signal_type != null and .error.violation.signal_name == null) |
      "  - id=\(.error.violation.id), signal_type=\(.error.violation.signal_type)"
    ] | .[]
  ' "${observed}")"
  if [[ -n "${missing}" ]]; then
    echo "  FAIL: ${test_name} - findings have signal_type set but signal_name is missing:"
    echo "${missing}"
    exit 1
  fi

  echo "  PASS: ${test_name} - signal conventions."
}

run_policy_test() {
  local test_dir="$1"
  local package_dir="$2"
  local test_name="${test_dir#"${package_dir}"/}"
  matches_test_filter "${test_name#tests/}" || return 0

  local diagnostic_templates="${ROOT}/diagnostic_templates"
  local coverage_flag=()
  [[ "${COVERAGE}" == "true" ]] && coverage_flag=(--display-policy-coverage)

  echo "-> Running test [${test_name}] ..."
  local observed_dir="${package_dir}/observed-output/${test_name}"
  rm -rf "${observed_dir}"
  mkdir -p "${observed_dir}"

  local baseline_flag=()
  [[ -d "${test_dir}/base" ]] && baseline_flag=(--baseline-registry base)

  # The test directory is the working directory so that file provenance stays
  # relative. The model is checked without the policy first, so a broken model
  # is reported as such instead of as a policy mismatch.
  (
    cd "${test_dir}" || exit 1
    if ! "${WEAVER}" registry check -r current ${baseline_flag[@]+"${baseline_flag[@]}"} --quiet --v2 >"${observed_dir}/model-check.stdout" 2>&1; then
      echo "Test model in \"base\" or \"current\" is incorrect. Invalid test configuration."
      cat "${observed_dir}/model-check.stdout"
      exit 1
    fi
    NO_COLOR=1 "${WEAVER}" registry check \
      -r current \
      ${baseline_flag[@]+"${baseline_flag[@]}"} \
      -p "${package_dir}" \
      --v2 \
      --quiet \
      ${coverage_flag[@]+"${coverage_flag[@]}"} \
      --diagnostic-template="${diagnostic_templates}" \
      --diagnostic-format json \
      --diagnostic-stdout \
      >"${observed_dir}/diagnostic-output.raw" \
      2>"${observed_dir}/stderr"
    exit 0
  ) || exit 1

  sed -n '/^\[/,$p' "${observed_dir}/diagnostic-output.raw" |
    jq -S 'map(select(.error.violation.type == "PolicyFinding"))' >"${observed_dir}/diagnostic-output.json"
  check_output "${observed_dir}/diagnostic-output.json" "${test_dir}/expected-diagnostic-output.json" "${test_name} - Diagnostic Output"
  validate_signal_conventions "${observed_dir}/diagnostic-output.json" "${test_name}"
}

run_tests() {
  local package_dir="$1"
  local tests_dir="${package_dir}/tests"
  [[ -d "${tests_dir}" ]] || log_err "Error: Tests not found in '${tests_dir}' for policy package: ${package_dir}"
  for dir in "${tests_dir}"/*; do
    [[ -d "${dir}" ]] && run_policy_test "${dir}" "${package_dir}"
  done
}

run_all_policy_package_tests() {
  for package in "${ROOT}"/policies/check/*; do
    [[ -d "${package}" ]] || continue
    echo "---==== Policy Package - ${package#"${ROOT}"/policies/check/} ====---"
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
  ROOT="$(realpath "${PWD}/../../..")"
  run_tests "${PWD}"
else
  run_all_policy_package_tests
fi
