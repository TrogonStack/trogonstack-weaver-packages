#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
PACKAGE_DIR="${ROOT}/templates/code/k6"
WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/weaver-k6.XXXXXX")"
trap 'rm -rf "${WORK_DIR}"' EXIT

if [[ "$(k6 version)" != k6\ v2.3.0* ]]; then
  echo 'FAIL: k6 v2.3.0 is required'
  exit 1
fi
cp "${ROOT}/buildscripts/k6/"* "${WORK_DIR}/"
mkdir -p "${WORK_DIR}/cases"
for test_dir in "${PACKAGE_DIR}/tests/"*/; do
  [[ -f "${test_dir}expected-error.txt" ]] && continue
  test_name="$(basename "${test_dir}")"
  if [[ ! -d "${test_dir}expected" || ! -f "${test_dir}consumer.ts" || ! -f "${test_dir}samples.json" ]]; then
    echo "FAIL: ${test_name} needs generated output, a real consumer and sample assertions"
    exit 1
  fi
  node "${ROOT}/buildscripts/k6/check-generated-output.mjs" "${test_dir}expected" "${test_dir}samples.json"
  mkdir -p "${WORK_DIR}/cases/${test_name}"
  cp -R "${test_dir}expected/." "${WORK_DIR}/cases/${test_name}/"
  cp "${test_dir}consumer.ts" "${test_dir}samples.json" "${WORK_DIR}/cases/${test_name}/"
  if [[ -f "${test_dir}consumer.types.ts" ]]; then
    cp "${test_dir}consumer.types.ts" "${WORK_DIR}/cases/${test_name}/"
  fi
done

cd "${WORK_DIR}"
npm ci --ignore-scripts --no-audit --no-fund --cache "${WORK_DIR}/npm-cache"
./node_modules/.bin/tsc --project tsconfig.json
for test_dir in cases/*; do
  echo "-> Running real k6 consumer: ${test_dir}"
  K6_NO_USAGE_REPORT=true K6_NO_EXTENSIONS=true SUMMARY_PATH="${WORK_DIR}/${test_dir}/summary.json" \
    k6 run --quiet --address 127.0.0.1:0 --no-usage-report --system-tags '' --vus 1 --iterations 1 \
    --out "json=${WORK_DIR}/${test_dir}/samples-observed.json" "${test_dir}/consumer.ts" \
    > "${test_dir}/run.log" 2>&1 || { cat "${test_dir}/run.log"; exit 1; }
  if grep -q 'level=error' "${test_dir}/run.log"; then
    cat "${test_dir}/run.log"
    echo "FAIL: ${test_dir} logged a runtime error"
    exit 1
  fi
  node assert-output.mjs "${test_dir}"
done
node check-tag-assertions.mjs cases/instruments

threshold_status=0
K6_NO_USAGE_REPORT=true K6_NO_EXTENSIONS=true EXPECT_THRESHOLD_FAILURE=true \
  SUMMARY_PATH="${WORK_DIR}/cases/instruments/summary.json" \
  k6 run --quiet --address 127.0.0.1:0 --no-usage-report --system-tags '' --vus 1 --iterations 1 \
  --out "json=${WORK_DIR}/cases/instruments/samples-observed.json" cases/instruments/consumer.ts \
  > cases/instruments/threshold-failure.log 2>&1 || threshold_status=$?
if [[ "${threshold_status}" != 99 ]]; then
  cat cases/instruments/threshold-failure.log
  echo "FAIL: native threshold rejection must exit 99, observed ${threshold_status}"
  exit 1
fi
node assert-output.mjs cases/instruments threshold-failure
K6_NO_USAGE_REPORT=true K6_NO_EXTENSIONS=true k6 run --quiet --address 127.0.0.1:0 \
  --no-usage-report --vus 1 --iterations 1 runtime-contract.ts

echo 'PASS: generated TypeScript source and native k6 consumers'
