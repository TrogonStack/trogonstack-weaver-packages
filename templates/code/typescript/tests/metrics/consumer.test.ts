import assert from "node:assert/strict";
import { test } from "node:test";
import { context, createContextKey, metrics, ROOT_CONTEXT, type Context } from "@opentelemetry/api";
import { MeterProvider, MetricReader } from "@opentelemetry/sdk-metrics";
import { createRecorder } from "../../support.ts";
import { MyappTaskState, MyappTaskId } from "./attributes.ts";
import * as Metrics from "./metrics.ts";
import { SCHEMA_URL } from "./schema.ts";

class ContextReader extends MetricReader {
  protected onShutdown(): Promise<void> {
    return Promise.resolve();
  }

  protected onForceFlush(): Promise<void> {
    return Promise.resolve();
  }
}

function createContextRecorder() {
  const reader = new ContextReader();
  const contextKey = createContextKey("metric-context");
  const explicitContext = ROOT_CONTEXT.setValue(contextKey, "explicit");
  const observedContexts: (Context | undefined)[] = [];
  const meterProvider = new MeterProvider({
    readers: [reader],
    views: [{
      instrumentName: "*",
      attributesProcessors: [{
        process(attributes, recordedContext) {
          observedContexts.push(recordedContext);
          return {
            ...attributes,
            "test.context": recordedContext?.getValue(contextKey) === "explicit" ? "explicit" : "default",
          };
        },
      }],
    }],
  });
  return { reader, meterProvider, explicitContext, observedContexts };
}

const contextCases: readonly {
  readonly name: string;
  record(meter: Metrics.Meter, context?: Context): void;
}[] = [
  {
    name: Metrics.METRIC_AUTH_ATTEMPTS,
    record(meter, context) {
      new Metrics.AuthAttempts(meter).add(1, { method: "password", success: true }, context);
    },
  },
  {
    name: Metrics.METRIC_MYAPP_TASK_STARTED,
    record(meter, context) {
      new Metrics.MyappTaskStarted(meter).add(1, undefined, context);
    },
  },
  {
    name: Metrics.METRIC_MYAPP_TASK_FLOAT_STARTED,
    record(meter, context) {
      new Metrics.MyappTaskFloatStarted(meter).add(1, undefined, context);
    },
  },
  {
    name: Metrics.METRIC_MYAPP_TASK_ACTIVE,
    record(meter, context) {
      new Metrics.MyappTaskActive(meter).add(1, { taskState: MyappTaskState.Queued }, context);
    },
  },
  {
    name: Metrics.METRIC_MYAPP_TASK_FLOAT_ACTIVE,
    record(meter, context) {
      new Metrics.MyappTaskFloatActive(meter).add(1, undefined, context);
    },
  },
  {
    name: Metrics.METRIC_MYAPP_TASK_FLOAT_DEPTH,
    record(meter, context) {
      new Metrics.MyappTaskFloatDepth(meter).record(1, undefined, context);
    },
  },
  {
    name: Metrics.METRIC_MYAPP_TASK_DURATION,
    record(meter, context) {
      new Metrics.MyappTaskDuration(meter).record(1, { taskId: "task_0001" }, context);
    },
  },
  {
    name: Metrics.METRIC_MYAPP_TASK_PAYLOAD,
    record(meter, context) {
      new Metrics.MyappTaskPayload(meter).record(1, undefined, context);
    },
  },
];

test("native instruments pass explicit context to SDK attribute processors", async () => {
  const recorder = createContextRecorder();
  try {
    recorder.meterProvider.getMeter("native").createCounter("native.context").add(1, {}, recorder.explicitContext);
    assert.equal(recorder.observedContexts.at(-1), recorder.explicitContext);
  } finally {
    await recorder.meterProvider.shutdown();
  }
});

for (const contextCase of contextCases) {
  test(`${contextCase.name} preserves explicit context and the SDK default`, async () => {
    const recorder = createContextRecorder();
    try {
      const meter = Metrics.getMeter(recorder.meterProvider);
      contextCase.record(meter, recorder.explicitContext);
      assert.equal(recorder.observedContexts.at(-1), recorder.explicitContext);
      contextCase.record(meter);
      assert.equal(recorder.observedContexts.at(-1), context.active());

      const { resourceMetrics } = await recorder.reader.collect();
      const metric = resourceMetrics.scopeMetrics.flatMap((scope) => scope.metrics).find((metric) => metric.descriptor.name === contextCase.name);
      assert.deepEqual(metric?.dataPoints.map((point) => point.attributes["test.context"]).sort(), ["default", "explicit"]);
    } finally {
      await recorder.meterProvider.shutdown();
    }
  });
}

test("instruments carry the registry name, unit, description, scope and schema URL", async () => {
  const recorder = createRecorder();
  const meter = Metrics.getMeter(recorder.meterProvider);
  new Metrics.MyappTaskDuration(meter).record(1.5, { taskId: "task_0001" });

  const metric = await recorder.metric("myapp.task.duration");
  assert.equal(Metrics.METRIC_MYAPP_TASK_DURATION, "myapp.task.duration");
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
  const observable = new Metrics.MyappTaskActiveObservable(meter, (observe) => {
    observe(4, { taskState: MyappTaskState.Done });
  });
  const first = await recorder.metric("myapp.task.active");
  assert.equal(first.dataPoints[0]?.value, 4);
  assert.deepEqual(first.dataPoints[0]?.attributes, { "myapp.task.state": "done" });

  const depth = new Metrics.MyappQueueDepthObservable(meter, async (observe) => {
    await Promise.resolve();
    observe(7);
  });
  assert.equal((await recorder.metric("myapp.queue.depth")).dataPoints[0]?.value, 7);
  observable.stop();
  depth.stop();
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

test("METRIC_ constants hold the name the metric records under", () => {
  assert.equal(Metrics.METRIC_AUTH_ATTEMPTS, "auth.attempts");
});

function invalidConsumers() {
  const meter = Metrics.getMeter();
  const attempts = new Metrics.AuthAttempts(meter);
  const explicitContext = ROOT_CONTEXT;
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
  // @ts-expect-error Context does not replace required attributes.
  attempts.add(1, undefined, explicitContext);
  // @ts-expect-error Required attributes remain required with context.
  attempts.add(1, { method: "password" }, explicitContext);
  // @ts-expect-error Enum values remain closed with context.
  attempts.add(1, { method: "unknown", success: true }, explicitContext);
  // @ts-expect-error Attribute types remain checked with context.
  attempts.add(1, { method: "password", success: "yes" }, explicitContext);
  // @ts-expect-error Unknown attributes remain rejected with context.
  attempts.add(1, { method: "password", success: true, extra: 1 }, explicitContext);
  // @ts-expect-error A context must implement the OpenTelemetry Context API.
  attempts.add(1, { method: "password", success: true }, {});
  // @ts-expect-error No-attribute instruments reject undeclared attributes.
  new Metrics.MyappTaskFloatStarted(meter).add(1, { extra: 1 }, explicitContext);
  // @ts-expect-error No-attribute instruments keep context in the third argument.
  new Metrics.MyappTaskFloatStarted(meter).add(1, explicitContext);
  // @ts-expect-error No-attribute instruments still require a proper context.
  new Metrics.MyappTaskFloatStarted(meter).add(1, undefined, {});
  // @ts-expect-error Optional-only attributes do not replace context.
  new Metrics.MyappTaskStarted(meter).add(1, explicitContext);
  // @ts-expect-error Observable observations have no context argument.
  new Metrics.MyappTaskActiveObservable(meter, (observe) => observe(1, { taskState: "queued" }, explicitContext));
  // @ts-expect-error The generated members are read-only.
  MyappTaskState.Done = "queued";
  // @ts-expect-error A gauge has no add method.
  new Metrics.MyappTaskFloatDepth(meter).add(1);
  // @ts-expect-error A histogram has no observable form.
  new Metrics.MyappTaskDurationObservable(meter, () => {});
  // @ts-expect-error A meter without the registry scope is rejected.
  new Metrics.AuthAttempts(metrics.getMeter("other"));
  // @ts-expect-error The name is the literal registry name.
  const other: "other" = Metrics.METRIC_AUTH_ATTEMPTS;
  void other;
}
void invalidConsumers;
