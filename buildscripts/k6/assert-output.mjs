import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const directory = process.argv[2];
const expectFailure = process.argv[3] === "threshold-failure";
const expected = JSON.parse(readFileSync(`${directory}/samples.json`, "utf8"));
const samples = readFileSync(`${directory}/samples-observed.json`, "utf8").trim().split("\n").map(JSON.parse);
const summary = JSON.parse(readFileSync(`${directory}/summary.json`, "utf8"));
for (const [name, metric] of Object.entries(expected)) {
  const definition = samples.find((entry) => entry.type === "Metric" && entry.metric === name);
  assert.ok(definition, `missing native metric ${name}`);
  assert.equal(definition.data.type, metric.kind, `${name} kind`);
  assert.equal(definition.data.contains, metric.contains, `${name} time flag`);
  const points = samples.filter((entry) => entry.type === "Point" && entry.metric === name);
  assert.deepEqual(points.map((entry) => entry.data.value), metric.values, `${name} values`);
  assert.deepEqual(points.map((entry) => entry.data.tags ?? {}), metric.tags ?? metric.values.map(() => ({})), `${name} tags`);
  for (const [statistic, value] of Object.entries(metric.summary)) {
    assert.equal(summary.metrics[name].values[statistic], value, `${name} ${statistic}`);
  }
}
const generatedNames = samples.filter((entry) => entry.type === "Metric" && !(entry.metric in expected)).map((entry) => entry.metric);
assert.ok(generatedNames.every((name) => ["iterations", "iteration_duration", "vus", "vus_max", "data_sent", "data_received"].includes(name)), `unexpected metric: ${generatedNames}`);
for (const [name, metric] of Object.entries(summary.metrics)) {
  for (const [expression, result] of Object.entries(metric.thresholds ?? {})) {
    assert.equal(result.ok, !(expectFailure && name === "sample_count" && expression === "count==999"), `${name} threshold ${expression}`);
  }
}
if (expectFailure) assert.equal(summary.metrics.sample_count.thresholds["count==999"].ok, false);
assert.equal(summary.metrics.iterations.values.count, 1, "one local iteration");
console.log(`PASS: ${directory} native names, kinds, values, tags, time flags and thresholds`);
