#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
PACKAGE_DIR="${ROOT}/templates/code/typescript"
WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/weaver-typescript.XXXXXX")"
trap 'rm -rf "${WORK_DIR}"' EXIT

cp "${ROOT}/buildscripts/typescript/"* "${WORK_DIR}/"
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
  echo 'FAIL: no TypeScript consumer tests found'
  exit 1
fi
node --test "${consumers[@]}"

# A bundler drops an unused class or constant only when no top-level statement
# has a side effect. esbuild is stricter about that than Rollup. A consumer that
# compiles with tsc for a release before ES2022 also turns static fields into
# top-level assignments, and #private fields into a WeakMap lookup per call.
./node_modules/.bin/tsc --project tsconfig.json --noEmit false --outDir downlevel --target ES2020 \
  --useDefineForClassFields false --rewriteRelativeImportExtensions true --declaration false
for module in cases/*/*.ts downlevel/cases/*/*.js; do
  [[ "${module}" == *consumer.test.* ]] && continue
  printf 'import "./%s";\n' "${module}" > side-effects.mjs
  kept="$(./node_modules/.bin/esbuild side-effects.mjs --bundle --format=esm --target=es2020 --log-level=error)"
  if [[ -n "${kept}" ]]; then
    echo "FAIL: ${module} keeps code when nothing is imported from it:"
    echo "${kept}"
    exit 1
  fi
  printf 'export * from "./%s";\n' "${module}" > everything.mjs
  if ./node_modules/.bin/esbuild everything.mjs --bundle --format=esm --target=es2020 --log-level=error | grep -qE 'WeakMap|__private'; then
    echo "FAIL: ${module} reaches private fields through a WeakMap below ES2022"
    exit 1
  fi
done
echo 'PASS: every generated module is free of side effects'
