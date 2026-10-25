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
      yield* Metrics.recordSampleDurationS(duration);
      yield* Metrics.recordSampleDurationMs(duration);
      yield* Metrics.recordSampleDurationUs(duration);
      yield* Metrics.recordSampleDurationNs(duration);
      yield* Metrics.recordSampleSize(512);
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
