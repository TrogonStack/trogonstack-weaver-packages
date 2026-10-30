import assert from "node:assert/strict";
import { test } from "node:test";
import { metrics } from "@opentelemetry/api";
import { createRecorder } from "../../support.ts";
import { MyappTaskState, MyappTaskId } from "./attributes.ts";
import * as Metrics from "./metrics.ts";
import { SCHEMA_URL } from "./schema.ts";

test("instruments carry the registry name, unit, description, scope and schema URL", async () => {
  const recorder = createRecorder();
  const meter = Metrics.getMeter(recorder.meterProvider);
  new Metrics.MyappTaskDuration(meter).record(1.5, { taskId: "task_0001" });

  const metric = await recorder.metric("myapp.task.duration");
  assert.equal(Metrics.MyappTaskDuration.NAME, "myapp.task.duration");
  assert.equal(metric.descriptor.unit, "s");
  assert.equal(metric.descriptor.description, "Time a task took from start to finish.");
  assert.equal(metric.descriptor.valueType, 1);
  const [scope] = await recorder.scopes();
  assert.equal(scope?.scope.name, Metrics.INSTRUMENTATION_NAME);
  assert.equal(scope?.scope.schemaUrl, SCHEMA_URL);
  assert.equal(SCHEMA_URL, "https://example.com/schemas/metrics/1.0.0");
});

test("a histogram records required and optional attributes under their wire keys", async () => {
  const recorder = createRecorder();
  const duration = new Metrics.MyappTaskDuration(Metrics.getMeter(recorder.meterProvider));
  duration.record(0.05, {
    taskId: "task_0001",
    taskState: MyappTaskState.Done,
    authMethod: "password",
  });
  duration.record(2, { taskId: "task_0002", taskState: undefined });

  const metric = await recorder.metric("myapp.task.duration");
  const keys = metric.dataPoints.map((point) => Object.keys(point.attributes).sort());
  assert.deepEqual(keys, [
    ["auth.method", "myapp.task.id", "myapp.task.state"],
    ["myapp.task.id"],
  ]);
  assert.equal(metric.dataPoints[0]?.attributes["myapp.task.state"], "done");
});

test("histogram bucket boundaries come from the registry aggregation", async () => {
  const recorder = createRecorder();
  new Metrics.MyappTaskDuration(Metrics.getMeter(recorder.meterProvider)).record(0.05, {
    taskId: MyappTaskId.KEY,
  });
  const metric = await recorder.metric("myapp.task.duration");
  const [point] = metric.dataPoints;
  assert.deepEqual((point?.value as { buckets: { boundaries: number[] } }).buckets.boundaries, [
    0, 0.005, 0.01, 0.1, 1, 10,
  ]);
});

test("a counter whose attributes are all optional can be added without them", async () => {
  const recorder = createRecorder();
  const started = new Metrics.MyappTaskStarted(Metrics.getMeter(recorder.meterProvider));
  started.add(1);
  started.add(2, { taskId: "task_0001" });
  const metric = await recorder.metric("myapp.task.started");
  assert.equal(metric.descriptor.valueType, 0);
  assert.deepEqual(
    metric.dataPoints.map((point) => [point.attributes, point.value]),
    [
      [{}, 1],
      [{ "myapp.task.id": "task_0001" }, 2],
    ],
  );
});

test("an up-down counter and a gauge record their values", async () => {
  const recorder = createRecorder();
  const meter = Metrics.getMeter(recorder.meterProvider);
  const active = new Metrics.MyappTaskActive(meter);
  active.add(2, { taskState: MyappTaskState.Queued });
  active.add(-1, { taskState: MyappTaskState.Queued });
  new Metrics.MyappTaskFloatDepth(meter).record(2.5);

  assert.equal((await recorder.metric("myapp.task.active")).dataPoints[0]?.value, 1);
  assert.equal((await recorder.metric("myapp.task.float.depth")).dataPoints[0]?.value, 2.5);
  assert.equal((await recorder.metric("myapp.task.float.depth")).descriptor.valueType, 1);
});

test("an observable instrument reports when the SDK collects and stops on request", async () => {
  const recorder = createRecorder();
  const meter = Metrics.getMeter(recorder.meterProvider);
  const observable = new Metrics.MyappTaskActiveObservable(meter, (observer) => {
    observer.observe(4, { taskState: MyappTaskState.Done });
  });
  const first = await recorder.metric("myapp.task.active");
  assert.equal(first.dataPoints[0]?.value, 4);
  assert.deepEqual(first.dataPoints[0]?.attributes, { "myapp.task.state": "done" });

  const depth = new Metrics.MyappQueueDepthObservable(meter, async (observer) => {
    await Promise.resolve();
    observer.observe(7);
  });
  assert.equal((await recorder.metric("myapp.queue.depth")).dataPoints[0]?.value, 7);
  observable.stop();
  depth.stop();
  assert.equal(Metrics.MyappTaskActiveObservable.NAME, Metrics.MyappTaskActive.NAME);
});

test("a meter without a provider is harmless until a global provider is set", async () => {
  const meter = Metrics.getMeter();
  new Metrics.AuthAttempts(meter).add(1, { method: "password", success: true });

  const recorder = createRecorder();
  metrics.setGlobalMeterProvider(recorder.meterProvider);
  const late = Metrics.getMeter();
  new Metrics.AuthAttempts(late).add(1, { method: "token", success: false });
  const [point] = (await recorder.metric("auth.attempts")).dataPoints;
  assert.deepEqual(point?.attributes, { "auth.method": "token", "auth.success": false });
  metrics.disable();
});

test("NAME is the name the metric records under", () => {
  assert.equal(Metrics.AuthAttempts.NAME, "auth.attempts");
  assert.equal(Metrics.AuthAttemptsObservable.NAME, "auth.attempts");
});

function invalidConsumers() {
  const meter = Metrics.getMeter();
  const attempts = new Metrics.AuthAttempts(meter);
  // @ts-expect-error A required attribute is missing.
  attempts.add(1, { method: "password" });
  // @ts-expect-error Enum values stay closed.
  attempts.add(1, { method: "unknown", success: true });
  // @ts-expect-error Attribute types are checked.
  attempts.add(1, { method: "password", success: "yes" });
  // @ts-expect-error Unknown attributes are rejected.
  attempts.add(1, { method: "password", success: true, extra: 1 });
  // @ts-expect-error A required attribute list cannot be omitted.
  attempts.add(1);
  // @ts-expect-error The generated frozen members cannot be reassigned.
  MyappTaskState.Done = "queued";
  // @ts-expect-error A gauge has no add method.
  new Metrics.MyappTaskFloatDepth(meter).add(1);
  // @ts-expect-error A histogram has no observable form.
  new Metrics.MyappTaskDurationObservable(meter, () => {});
  // @ts-expect-error A meter without the registry scope is rejected.
  new Metrics.AuthAttempts(metrics.getMeter("other"));
  // @ts-expect-error The metric name cannot be reassigned.
  Metrics.AuthAttempts.NAME = "other";
}
void invalidConsumers;
