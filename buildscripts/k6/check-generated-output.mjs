import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const [directory, samplesPath] = process.argv.slice(2);
const files = readdirSync(directory).sort();
for (const file of files) {
  assert.equal(readFileSync(join(directory, file)).at(-1), 10, `${file} must end with a newline`);
}
const samples = JSON.parse(readFileSync(samplesPath, "utf8"));
assert.deepEqual(files, Object.keys(samples).length === 0 ? ["schema.ts"] : ["metrics.ts", "schema.ts"],
  "k6 generation must produce only TypeScript source modules");
