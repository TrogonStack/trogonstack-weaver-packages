import assert from "node:assert/strict";
import { copyFileSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const source = process.argv[2];
const samples = readFileSync(join(source, "samples-observed.json"), "utf8").trim().split("\n").map(JSON.parse);
const directory = mkdtempSync(join(tmpdir(), "weaver-k6-tags-"));
try {
  for (const file of ["samples.json", "summary.json"]) {
    copyFileSync(join(source, file), join(directory, file));
  }
  for (const name of ["sample_count", "sample_depth"]) {
    const mutated = structuredClone(samples);
    const point = mutated.find((entry) => entry.type === "Point" && entry.metric === name);
    assert.ok(point, `missing native mutation sample ${name}`);
    point.data.tags.unexpected_custom_tag = "unexpected";
    writeFileSync(join(directory, "samples-observed.json"), mutated.map(JSON.stringify).join("\n") + "\n");
    const result = spawnSync(process.execPath, [resolve("assert-output.mjs"), directory], { encoding: "utf8" });
    assert.equal(result.status, 1, `${name} must reject an unexpected native tag`);
    assert.match(result.stderr, new RegExp(`${name} tags`));
  }
} finally {
  rmSync(directory, { recursive: true, force: true });
}
console.log("PASS: exact tag assertions reject mutations of real native samples");
