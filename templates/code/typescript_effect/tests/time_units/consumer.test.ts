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
      assert.equal((yield* Metric.value(Metrics.sampleDurationS)).sum, 0.125);
      assert.equal((yield* Metric.value(Metrics.sampleDurationMs)).sum, 125);
      assert.equal(
        (yield* Metric.value(Metrics.sampleDurationUs)).sum,
        125_000,
      );
      assert.equal(
        (yield* Metric.value(Metrics.sampleDurationNs)).sum,
        125_000_000,
      );

      yield* Metrics.recordSampleSize(512);
      assert.equal((yield* Metric.value(Metrics.sampleSize)).sum, 512);
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});
