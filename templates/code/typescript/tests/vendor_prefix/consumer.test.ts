import assert from "node:assert/strict";
import { test } from "node:test";
import { createRecorder } from "../../support.ts";
import * as Attributes from "./attributes.ts";
import * as Metrics from "./metrics.ts";

test("the vendor prefix leaves the TypeScript names but not the wire keys", async () => {
  const recorder = createRecorder();
  const meter = Metrics.getMeter(recorder.meterProvider);
  new Metrics.TaskDuration(meter).record(1, {
    id: "task_0001",
    state: Attributes.TaskState.Done,
    authMethod: "password",
  });
  new Metrics.Myapp(meter).add(1, { myapp: "a" });

  const duration = await recorder.metric("myapp.task.duration");
  assert.deepEqual(duration.dataPoints[0]?.attributes, {
    "myapp.task.id": "task_0001",
    "myapp.task.state": "done",
    "auth.method": "password",
  });
  assert.equal(Attributes.TaskId.KEY, "myapp.task.id");
  assert.equal(Metrics.TaskDuration.getName(), "myapp.task.duration");
  assert.deepEqual((await recorder.metric("myapp")).dataPoints[0]?.attributes, { myapp: "a" });
});
