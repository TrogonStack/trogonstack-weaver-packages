#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
PACKAGE_DIR="${ROOT}/templates/code/typescript_effect"
scratch_root="${TMPDIR:-/tmp}"
if [[ -d /Volumes/TrashCan ]]; then
  scratch_root=/Volumes/TrashCan
fi
WORK_DIR="$(mktemp -d "${scratch_root%/}/weaver-effect.XXXXXX")"
trap 'rm -rf "${WORK_DIR}"' EXIT

cp "${ROOT}/buildscripts/typescript_effect/"* "${WORK_DIR}/"
mkdir -p "${WORK_DIR}/cases"
for test_dir in "${PACKAGE_DIR}/tests/"*/; do
  [[ -f "${test_dir}expected-error.txt" ]] && continue
  test_name="$(basename "${test_dir}")"
  if [[ ! -d "${test_dir}expected" ]]; then
    echo "FAIL: ${test_name} has no expected output"
    exit 1
  fi
  mkdir -p "${WORK_DIR}/cases/${test_name}"
  cp -R "${test_dir}expected/." "${WORK_DIR}/cases/${test_name}/"
  if [[ -f "${test_dir}consumer.test.ts" ]]; then
    cp "${test_dir}consumer.test.ts" "${WORK_DIR}/cases/${test_name}/"
  fi
done

cd "${WORK_DIR}"
npm ci --ignore-scripts --no-audit --no-fund --cache "${WORK_DIR}/npm-cache"
./node_modules/.bin/tsc --project tsconfig.json
shopt -s nullglob
consumers=(cases/*/consumer.test.ts)
if [[ ${#consumers[@]} -eq 0 ]]; then
  echo 'FAIL: no Effect consumer tests found'
  exit 1
fi
node --test "${consumers[@]}"
