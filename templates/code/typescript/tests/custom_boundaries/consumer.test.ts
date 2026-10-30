import assert from "node:assert/strict";
import { test } from "node:test";
import { createRecorder } from "../../support.ts";
import * as Metrics from "./metrics.ts";

test("the params replace the unit defaults", async () => {
  const recorder = createRecorder();
  const meter = Metrics.getMeter(recorder.meterProvider);
  new Metrics.SampleDuration(meter).record(0.2);
  new Metrics.SampleSize(meter).record(3);

  const boundaries = async (name: string) => {
    const [point] = (await recorder.metric(name)).dataPoints;
    return (point?.value as { buckets: { boundaries: number[] } }).buckets.boundaries;
  };
  assert.deepEqual(await boundaries("sample.duration"), [0.1, 0.5, 1]);
  assert.deepEqual(await boundaries("sample.size"), [1, 2, 4]);
});
