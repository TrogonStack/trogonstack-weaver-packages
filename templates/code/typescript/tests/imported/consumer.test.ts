import assert from "node:assert/strict";
import { test } from "node:test";
import { createRecorder } from "../../support.ts";
import * as Metrics from "./metrics.ts";
import { SCHEMA_URL } from "./schema.ts";

test("attributes of a dependency keep their wire keys and closed value types", async () => {
  const recorder = createRecorder();
  const imported = new Metrics.SampleImported(new Metrics.Meter(recorder.meterProvider));
  imported.add(1, { remoteStatus: "ready", remotePort: 443 });
  imported.add(1, { remoteStatus: "done", remotePort: 0 });
  imported.add(1, { remoteStatus: "done" });

  const points = (await recorder.metric("sample.imported")).dataPoints;
  assert.deepEqual(points.map((point) => point.attributes), [
    { "remote.status": "ready", "remote.port": 443 },
    { "remote.status": "done", "remote.port": 0 },
    { "remote.status": "done" },
  ]);
  assert.equal(SCHEMA_URL, "https://example.com/schemas/synthetic/1.0.0");
});

function invalidConsumers() {
  const imported = new Metrics.SampleImported(new Metrics.Meter());
  // @ts-expect-error Imported enums stay closed.
  imported.add(1, { remoteStatus: "unknown" });
}
void invalidConsumers;
