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

test("required and optional labels reach the same native Effect instrument", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* Metrics.recordAuthAttempts(2, {
        "auth.method": "token",
        "auth.success": true,
      });
      const state = yield* Metric.value(
        Metric.withAttributes(Metrics.authAttempts, {
          "auth.method": "token",
          "auth.success": "true",
        }),
      );
      assert.equal(state.count, 2);
      assert.equal(state.incremental, true);

      yield* Metrics.recordMyappTaskStarted(3);
      assert.equal((yield* Metric.value(Metrics.myappTaskStarted)).count, 3);
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});

test("duration values and explicit histogram buckets use the registry unit", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      const labels = {
        "myapp.task.id": "task-1",
        "myapp.task.state": "done",
      } as const;
      yield* Metrics.recordMyappTaskDuration(Duration.millis(250), labels);
      const state = yield* Metric.value(
        Metric.withAttributes(Metrics.myappTaskDuration, labels),
      );
      assert.equal(state.count, 1);
      assert.equal(state.sum, 0.25);
      assert.equal(state.min, 0.25);
      assert.deepEqual(
        state.buckets.map(([boundary]) => boundary),
        [0, 0.005, 0.01, 0.1, 1, 10, Infinity],
      );
      const snapshot = yield* Metric.snapshot;
      const metric = snapshot.find((item) => item.id === "myapp.task.duration");
      assert.ok(metric);
      assert.equal(metric.attributes?.unit, "s");
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});

test("up-down counters decrease, gauges replace, and size histograms stay numeric", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      const labels = { "myapp.task.state": "queued" } as const;
      yield* Metrics.recordMyappTaskActive(5, labels);
      yield* Metrics.recordMyappTaskActive(-2, labels);
      const active = yield* Metric.value(
        Metric.withAttributes(Metrics.myappTaskActive, labels),
      );
      assert.equal(active.count, 3);
      assert.equal(active.incremental, false);

      yield* Metrics.recordMyappQueueDepth(9);
      yield* Metrics.recordMyappQueueDepth(4);
      assert.equal((yield* Metric.value(Metrics.myappQueueDepth)).value, 4);

      yield* Metrics.recordMyappTaskPayload(256);
      const payload = yield* Metric.value(Metrics.myappTaskPayload);
      assert.equal(payload.sum, 256);
      assert.equal(payload.count, 1);
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
        "myapp.task.id": "overflow-task",
      });
      yield* Metrics.recordMyappTaskStarted(2);
      yield* Metrics.recordMyappTaskActive(-1, { "myapp.task.state": "done" });
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

function invalidConsumers() {
  // @ts-expect-error Required attributes cannot be omitted.
  Metrics.recordAuthAttempts(1);
  Metrics.recordAuthAttempts(1, {
    // @ts-expect-error Enum values must come from the registry.
    "auth.method": "unknown",
    "auth.success": true,
  });
  Metrics.recordAuthAttempts(1, {
    "auth.method": "token",
    // @ts-expect-error Attribute scalar types are preserved before label serialization.
    "auth.success": "true",
  });
  // @ts-expect-error Unrelated attributes are not accepted by typed recording helpers.
  Metrics.recordMyappTaskStarted(1, { "auth.success": true });
  // @ts-expect-error Duration histograms require an Effect Duration.
  Metrics.recordMyappTaskDuration(0.25, { "myapp.task.id": "task-1" });
  // @ts-expect-error Numeric histograms do not accept a Duration.
  Metrics.recordMyappTaskPayload(Duration.seconds(1));
  // @ts-expect-error Metrics with no dimensions reject arbitrary attributes.
  Metrics.recordMyappQueueDepth(1, { arbitrary: "label" });
}
void invalidConsumers;

const nativeTimer: Metric.Metric<Duration.Duration, unknown> =
  Metrics.myappTaskDuration;
const nativeCounter: Metric.Metric<number, unknown> = Metrics.authAttempts;
void nativeTimer;
void nativeCounter;
