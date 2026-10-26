import assert from "node:assert/strict";
import { test } from "node:test";
import * as Duration from "effect/Duration";
import * as Effect from "effect/Effect";
import * as Metric from "effect/Metric";
import * as Metrics from "./metrics.ts";

test("time histograms convert Duration into their declared unit", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      const duration = Duration.millis(125);
      yield* Metrics.updateSampleDurationS(duration);
      yield* Metrics.updateSampleDurationMs(duration);
      yield* Metrics.updateSampleDurationUs(duration);
      yield* Metrics.updateSampleDurationNs(duration);
      yield* Metrics.updateSampleSize(512);
      const snapshot = yield* Metric.snapshot;
      for (const [name, sum] of [
        ["sample.duration.s", 0.125],
        ["sample.duration.ms", 125],
        ["sample.duration.us", 125_000],
        ["sample.duration.ns", 125_000_000],
        ["sample.size", 512],
      ] as const) {
        const point = snapshot.find((item) => item.id === name);
        assert.ok(point?.type === "Histogram");
        assert.equal(point.state.sum, sum);
      }
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});

for (const [name, duration, nanoseconds] of [
  ["positive single nanosecond", Duration.nanos(1n), 1],
  ["negative single nanosecond", Duration.nanos(-1n), -1],
  ["positive nanoseconds", Duration.nanos(123n), 123],
  ["negative nanoseconds", Duration.nanos(-123n), -123],
  ["fractional milliseconds", Duration.millis(0.000123), 123],
] as const) {
  test(`time histograms preserve ${name} without a millisecond roundtrip`, async () => {
    await Effect.runPromise(
      Effect.gen(function* () {
        yield* Metrics.updateSampleDurationS(duration);
        yield* Metrics.updateSampleDurationMs(duration);
        yield* Metrics.updateSampleDurationUs(duration);
        yield* Metrics.updateSampleDurationNs(duration);
        yield* Metrics.updateSampleDurationNsTagged(duration, {
          run: "sample",
        });
        const snapshot = yield* Metric.snapshot;
        for (const [metric, expected] of [
          ["sample.duration.us", nanoseconds / 1_000],
          ["sample.duration.ns", nanoseconds],
          ["sample.duration.ns.tagged", nanoseconds],
          ["sample.duration.s", nanoseconds / 1_000_000_000],
          ["sample.duration.ms", nanoseconds / 1_000_000],
        ] as const) {
          const point = snapshot.find((item) => item.id === metric);
          assert.ok(point?.type === "Histogram");
          assert.equal(point.state.sum, expected, metric);
        }
      }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
    );
  });
}

for (const millis of [
  Number.MAX_SAFE_INTEGER,
  -Number.MAX_SAFE_INTEGER,
  Infinity,
  -Infinity,
]) {
  test(`time histograms preserve millisecond-backed scaling for ${millis}`, async () => {
    await Effect.runPromise(
      Effect.gen(function* () {
        const duration = Duration.millis(millis);
        yield* Metrics.updateSampleDurationS(duration);
        yield* Metrics.updateSampleDurationMs(duration);
        yield* Metrics.updateSampleDurationUs(duration);
        yield* Metrics.updateSampleDurationNs(duration);
        yield* Metrics.updateSampleDurationNsTagged(duration, {
          run: "sample",
        });
        const snapshot = yield* Metric.snapshot;
        for (const [metric, expected] of [
          ["sample.duration.s", millis / 1_000],
          ["sample.duration.ms", millis],
          ["sample.duration.us", millis * 1_000],
          ["sample.duration.ns", millis * 1_000_000],
          ["sample.duration.ns.tagged", millis * 1_000_000],
        ] as const) {
          const point = snapshot.find((item) => item.id === metric);
          assert.ok(point?.type === "Histogram");
          assert.equal(point.state.sum, expected, metric);
        }
      }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
    );
  });
}
