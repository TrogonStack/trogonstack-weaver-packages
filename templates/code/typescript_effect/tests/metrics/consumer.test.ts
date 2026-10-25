import assert from "node:assert/strict";
import { test } from "node:test";
import * as Duration from "effect/Duration";
import * as Effect from "effect/Effect";
import * as Metric from "effect/Metric";
import * as Layer from "effect/Layer";
import * as HttpClient from "effect/http/HttpClient";
import * as HttpClientResponse from "effect/http/HttpClientResponse";
import * as OtlpExporter from "effect/observability/OtlpExporter";
import * as OtlpMetrics from "effect/observability/OtlpMetrics";
import * as OtlpSerialization from "effect/observability/OtlpSerialization";

import * as Metrics from "./metrics.ts";

test("friendly fields and enum constants record registry keys and values", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* Metrics.recordAuthAttempts(2, {
        method: Metrics.AuthMethod.Token,
        success: true,
      });
      yield* Metrics.recordMyappTaskStarted(3);
      const snapshot = yield* Metric.snapshot;
      const auth = snapshot.find((item) => item.id === "auth.attempts");
      assert.ok(auth?.type === "Counter");
      assert.equal(auth.state.count, 2);
      assert.equal(auth.state.incremental, true);
      assert.equal(auth.attributes?.["auth.method"], "token");
      assert.equal(auth.attributes?.["auth.success"], "true");
      const started = snapshot.find((item) => item.id === "myapp.task.started");
      assert.ok(started?.type === "Counter");
      assert.equal(started.state.count, 3);
      assert.equal(
        Object.hasOwn(started.attributes ?? {}, "myapp.task.id"),
        false,
      );
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});

test("Duration histograms preserve units, buckets, and optional enum attributes", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* Metrics.recordMyappTaskDuration(Duration.millis(250), {
        taskId: "task-1",
        taskState: Metrics.MyappTaskState.Done,
        authMethod: Metrics.AuthMethod.Token,
      });
      const point = (yield* Metric.snapshot).find(
        (item) => item.id === "myapp.task.duration",
      );
      assert.ok(point?.type === "Histogram");
      assert.equal(point.state.count, 1);
      assert.equal(point.state.sum, 0.25);
      assert.equal(point.state.min, 0.25);
      assert.deepEqual(
        point.state.buckets.map(([boundary]) => boundary),
        [0, 0.005, 0.01, 0.1, 1, 10, Infinity],
      );
      assert.equal(point.attributes?.unit, "s");
      assert.equal(point.attributes?.["myapp.task.id"], "task-1");
      assert.equal(point.attributes?.["myapp.task.state"], "done");
      assert.equal(point.attributes?.["auth.method"], "token");
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});

test("up-down counters decrease, gauges replace, and numeric histograms stay numeric", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* Metrics.recordMyappTaskActive(5, {
        taskState: Metrics.MyappTaskState.Queued,
      });
      yield* Metrics.recordMyappTaskActive(-2, {
        taskState: Metrics.MyappTaskState.Queued,
      });
      yield* Metrics.recordMyappQueueDepth(9);
      yield* Metrics.recordMyappQueueDepth(4);
      yield* Metrics.recordMyappTaskPayload(256);
      const snapshot = yield* Metric.snapshot;
      const active = snapshot.find((item) => item.id === "myapp.task.active");
      assert.ok(active?.type === "Counter");
      assert.equal(active.state.count, 3);
      assert.equal(active.state.incremental, false);
      const depth = snapshot.find((item) => item.id === "myapp.queue.depth");
      assert.ok(depth?.type === "Gauge");
      assert.equal(depth.state.value, 4);
      const payload = snapshot.find((item) => item.id === "myapp.task.payload");
      assert.ok(payload?.type === "Histogram");
      assert.equal(payload.state.sum, 256);
      assert.equal(payload.state.count, 1);
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});

test("OTLP exports preserve histogram overflow, bounds, units, and monotonicity", async () => {
  const exports: OtlpMetrics.MetricsData[] = [];
  const client = HttpClient.make((request) =>
    Effect.sync(() => {
      assert.equal(request.body._tag, "Uint8Array");
      if (request.body._tag === "Uint8Array") {
        exports.push(JSON.parse(new TextDecoder().decode(request.body.body)));
      }
      return HttpClientResponse.fromWeb(request, new Response());
    }),
  );
  const layer = OtlpMetrics.layer({
    url: "http://collector.invalid/v1/metrics",
    resource: { serviceName: "synthetic-consumer" },
    exportInterval: "1 hour",
  }).pipe(
    Layer.provide(OtlpSerialization.layerJson),
    Layer.provide(Layer.succeed(HttpClient.HttpClient, client)),
  );
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* Metrics.recordMyappTaskDuration(Duration.seconds(20), {
        taskId: "overflow-task",
      });
      yield* Metrics.recordMyappTaskStarted(2);
      yield* Metrics.recordMyappTaskActive(-1, {
        taskState: Metrics.MyappTaskState.Done,
      });
      yield* (yield* OtlpExporter.Flusher).flush;
    }).pipe(
      Effect.provide(layer),
      Effect.provideService(Metric.MetricRegistry, new Map()),
    ),
  );
  assert.ok(exports.length > 0);
  const metrics = exports[0].resourceMetrics.flatMap((resource) =>
    resource.scopeMetrics.flatMap((scope) => scope.metrics),
  );
  const duration = metrics.find(
    (metric) => metric.name === "myapp.task.duration",
  );
  assert.ok(duration?.histogram);
  assert.equal(duration.unit, "s");
  const point = duration.histogram.dataPoints[0];
  assert.deepEqual(point.explicitBounds, [0, 0.005, 0.01, 0.1, 1, 10]);
  assert.deepEqual(point.bucketCounts, [0, 0, 0, 0, 0, 0, 1]);
  assert.equal(point.count, 1);
  assert.equal(point.sum, 20);
  assert.equal(
    metrics.find((metric) => metric.name === "myapp.task.started")?.sum
      ?.isMonotonic,
    true,
  );
  assert.equal(
    metrics.find((metric) => metric.name === "myapp.task.active")?.sum
      ?.isMonotonic,
    false,
  );
});

test("public exports provide typed recorders and enum constants, without raw instruments", () => {
  assert.equal("authAttempts" in Metrics, false);
  assert.equal("myappTaskDuration" in Metrics, false);
  assert.deepEqual(Metrics.AuthMethod, {
    Password: "password",
    Token: "token",
  });
});

function invalidConsumers() {
  // @ts-expect-error Required fields cannot be omitted.
  Metrics.recordAuthAttempts(1);
  Metrics.recordAuthAttempts(1, {
    // @ts-expect-error Enum values must come from the registry.
    method: "unknown",
    success: true,
  });
  Metrics.recordAuthAttempts(1, {
    method: Metrics.AuthMethod.Token,
    // @ts-expect-error Label scalar types are checked before serialization.
    success: "true",
  });
  // @ts-expect-error Unrelated fields are rejected.
  Metrics.recordMyappTaskStarted(1, { success: true });
  // @ts-expect-error Callers use generated fields rather than telemetry key strings.
  Metrics.recordMyappTaskStarted(1, { "myapp.task.id": "task-1" });
  // @ts-expect-error Duration histograms require an Effect Duration.
  Metrics.recordMyappTaskDuration(0.25, { taskId: "task-1" });
  // @ts-expect-error Numeric histograms do not accept a Duration.
  Metrics.recordMyappTaskPayload(Duration.seconds(1));
  // @ts-expect-error Metrics without attributes have no attribute argument.
  Metrics.recordMyappQueueDepth(1, { arbitrary: "label" });
  // @ts-expect-error Raw instruments cannot bypass the generated contract.
  Metrics.authAttempts;
}
void invalidConsumers;
