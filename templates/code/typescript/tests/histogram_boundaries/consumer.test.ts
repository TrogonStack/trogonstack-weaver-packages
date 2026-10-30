import assert from "node:assert/strict";
import { test } from "node:test";
import { createRecorder } from "../../support.ts";
import * as Metrics from "./metrics.ts";

async function boundariesOf(name: string, record: (meter: Metrics.Meter) => void) {
  const recorder = createRecorder();
  record(new Metrics.Meter(recorder.meterProvider));
  const [point] = (await recorder.metric(name)).dataPoints;
  return (point?.value as { buckets: { boundaries: number[] } }).buckets.boundaries;
}

test("an annotation wins over the unit default", async () => {
  assert.deepEqual(
    await boundariesOf("myapp.task.latency", (meter) =>
      new Metrics.MyappTaskLatency(meter).record(5, { taskId: "task_0001" }),
    ),
    [1, 10, 100],
  );
});

test("a second-based histogram gets the unit default", async () => {
  const expected = [0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1, 2.5, 5, 7.5, 10];
  assert.deepEqual(
    await boundariesOf("myapp.task.duration", (meter) =>
      new Metrics.MyappTaskDuration(meter).record(1, { taskId: "task_0001" }),
    ),
    expected,
  );
  assert.deepEqual(
    await boundariesOf("myapp.task.duration", (meter) =>
      new Metrics.MyappTaskRetryDuration(meter).record(1, { taskId: "task_0001" }),
    ),
    expected,
  );
});

test("a unit without a default leaves the boundaries to the SDK", async () => {
  const boundaries = await boundariesOf("myapp.task.size", (meter) =>
    new Metrics.MyappTaskSize(meter).record(1, { taskId: "task_0001" }),
  );
  assert.notDeepEqual(boundaries, [1, 10, 100]);
  assert.ok(boundaries.length > 0);
});
