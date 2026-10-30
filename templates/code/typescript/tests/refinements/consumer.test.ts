import assert from "node:assert/strict";
import { test } from "node:test";
import { createRecorder } from "../../support.ts";
import * as Metrics from "./metrics.ts";

test("a refinement has its own attribute type and records into the base instrument", async () => {
  const recorder = createRecorder();
  const meter = Metrics.getMeter(recorder.meterProvider);
  new Metrics.MyappTaskRetryDuration(meter).record(2, {
    taskId: "task_0001",
    taskState: "done",
  });
  new Metrics.WorkerTaskDuration(meter).record(3, { taskId: "task_0001" });

  const duration = await recorder.metric("myapp.task.duration");
  assert.equal(duration.descriptor.unit, "s");
  assert.equal(Metrics.MyappTaskRetryDuration.getName(), Metrics.MyappTaskDuration.getName());
  assert.deepEqual(
    duration.dataPoints.map((point) => point.attributes),
    [
      { "myapp.task.id": "task_0001", "myapp.task.state": "done" },
      { "myapp.task.id": "task_0001" },
    ],
  );
});

function invalidConsumers() {
  const meter = Metrics.getMeter();
  // @ts-expect-error The refinement requires the task state.
  new Metrics.MyappTaskRetryDuration(meter).record(1, { taskId: "task_0001" });
}
void invalidConsumers;
