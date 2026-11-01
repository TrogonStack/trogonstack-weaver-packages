#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
PACKAGE_DIR="${ROOT}/templates/code/python"
TOOLING_DIR="${ROOT}/buildscripts/python"
WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/weaver-python.XXXXXX")"
trap 'rm -rf "${WORK_DIR}"' EXIT
export PYTHONDONTWRITEBYTECODE=1

for test_dir in "${PACKAGE_DIR}"/tests/*/; do
  test_name="$(basename "${test_dir}")"
  [[ -f "${test_dir}expected-error.txt" ]] && continue
  if [[ ! -f "${test_dir}expected/__init__.py" ]]; then
    echo "  FAIL: ${test_name} is missing expected/__init__.py."
    exit 1
  fi
  if [[ ! -f "${test_dir}test_consumer.py" ]]; then
    echo "  FAIL: ${test_name} is missing test_consumer.py."
    exit 1
  fi
  mkdir -p "${WORK_DIR}/cases/${test_name}/semconv"
  cp -R "${test_dir}expected/." "${WORK_DIR}/cases/${test_name}/semconv/"
  cp "${test_dir}test_consumer.py" "${WORK_DIR}/cases/${test_name}/"
done

uv venv --quiet --python "$(command -v python)" "${WORK_DIR}/venv"
uv pip install --quiet --python "${WORK_DIR}/venv/bin/python" --require-hashes --no-deps -r "${TOOLING_DIR}/requirements.txt"
BIN="${WORK_DIR}/venv/bin"

echo "-> Checking ruff format ..."
"${BIN}/ruff" format --check --config "${TOOLING_DIR}/ruff.toml" "${WORK_DIR}/cases"
echo "-> Checking ruff check ..."
"${BIN}/ruff" check --no-cache --config "${TOOLING_DIR}/ruff.toml" "${WORK_DIR}/cases"

shopt -s nullglob
cases=("${WORK_DIR}"/cases/*/)
if [[ ${#cases[@]} -eq 0 ]]; then
  echo 'FAIL: no Python cases found'
  exit 1
fi
for case_dir in "${cases[@]}"; do
  echo "-> Checking Python [$(basename "${case_dir}")] ..."
  (
    cd "${case_dir}"
    "${BIN}/mypy" --config-file "${TOOLING_DIR}/mypy.ini" --cache-dir "${WORK_DIR}/mypy-cache" semconv test_consumer.py
    "${BIN}/python" -m pytest --quiet --disable-warnings -p no:cacheprovider test_consumer.py
  )
done
