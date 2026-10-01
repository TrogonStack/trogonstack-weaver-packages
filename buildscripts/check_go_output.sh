#!/usr/bin/env bash
# Compiles the expected output of every Go template test, so a template change
# that renders code which does not build fails even when the diff was accepted.

set -euo pipefail

OTEL_GO_VERSION="${OTEL_GO_VERSION:-v1.46.0}"
OTEL_GO_LOG_VERSION="${OTEL_GO_LOG_VERSION:-v0.22.0}"
WEAVER="${WEAVER:-weaver}"
ROOT="$(pwd)"
PACKAGE_DIR="${ROOT}/templates/code/go"
TEMPLATES_ROOT="$(realpath "${PACKAGE_DIR}/../..")"
DEFAULT_IMPORT_PATH="$(sed -n 's/^  import_path: *//p' "${PACKAGE_DIR}/weaver.yaml")"

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "${WORK_DIR}"' EXIT

for test_dir in "${PACKAGE_DIR}"/tests/*/; do
  test_name="$(basename "${test_dir}")"
  expected="${test_dir}expected"
  [[ -f "${test_dir}expected-error.txt" ]] && continue
  echo "-> Compiling [${test_name}] ..."
  if [[ ! -d "${expected}" ]]; then
    echo "  SKIPPED: Missing expected directory: ${expected}"
    continue
  fi

  import_path="${DEFAULT_IMPORT_PATH}"
  if [[ -f "${test_dir}params.yaml" ]]; then
    override="$(sed -n 's/^  import_path: *//p' "${test_dir}params.yaml")"
    import_path="${override:-${import_path}}"
  fi

  # The module is rooted at the import path's host, so an import path that
  # ends in an element Go rejects as a module path, such as `v1.2.0`, still
  # builds as a package inside it.
  module_path="${import_path%%/*}"
  module_dir="${WORK_DIR}/${test_name}"
  package_dir="${module_dir}${import_path#"${module_path}"}"
  mkdir -p "${package_dir}"
  cp -R "${expected}/." "${package_dir}/"

  # A test that maps an attribute namespace to a dependency registry, via
  # `imported_import_paths`, needs that dependency actually generated into the
  # module too, so the typed references it emits resolve to a real package.
  dependency_params="${test_dir}dependency-params.yaml"
  if [[ -f "${dependency_params}" ]]; then
    dependency_import_path="$(sed -n 's/^  import_path: *//p' "${dependency_params}")"
    dependency_dir="${module_dir}${dependency_import_path#"${module_path}"}"
    dependency_observed="${WORK_DIR}/${test_name}-dependency"
    mkdir -p "${dependency_dir}" "${dependency_observed}"
    (
      cd "${test_dir}"
      NO_COLOR=1 "${WEAVER}" registry generate \
        -r dependency \
        --v2 \
        --quiet \
        --params "dependency-params.yaml" \
        --templates="${TEMPLATES_ROOT}" \
        code/go \
        "${dependency_observed}"
    ) || { echo "  FAIL: ${test_name} could not generate its dependency."; exit 1; }
    cp -R "${dependency_observed}/." "${dependency_dir}/"
  fi

  unformatted="$(gofmt -l "${module_dir}")"
  if [[ -n "${unformatted}" ]]; then
    echo "  FAIL: ${test_name} is not gofmt clean:"
    echo "${unformatted}"
    exit 1
  fi

  (
    cd "${module_dir}"
    go mod init "${module_path}" >/dev/null 2>&1
    go get "go.opentelemetry.io/otel@${OTEL_GO_VERSION}" "go.opentelemetry.io/otel/metric@${OTEL_GO_VERSION}" "go.opentelemetry.io/otel/trace@${OTEL_GO_VERSION}" "go.opentelemetry.io/otel/log@${OTEL_GO_LOG_VERSION}" "go.opentelemetry.io/otel/sdk@${OTEL_GO_VERSION}" 2>&1 | grep -v "^go: " || true
    go mod tidy
    go build ./...
    go vet ./...
  ) || { echo "  FAIL: ${test_name} does not build."; exit 1; }
  echo "  PASS: ${test_name} builds and vets."
done
