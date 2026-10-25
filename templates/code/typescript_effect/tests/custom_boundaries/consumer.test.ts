import assert from "node:assert/strict";
import { test } from "node:test";
import * as Duration from "effect/Duration";
import * as Effect from "effect/Effect";
import * as Metric from "effect/Metric";

import * as Metrics from "./metrics.ts";

test("unit-specific boundaries override fallback boundaries", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* Metrics.recordSampleDuration(Duration.seconds(2));
      yield* Metrics.recordSampleSize(5);
      const duration = yield* Metric.value(Metrics.sampleDuration);
      const size = yield* Metric.value(Metrics.sampleSize);
      assert.deepEqual(duration.buckets, [
        [0.1, 0],
        [0.5, 0],
        [1, 0],
        [Infinity, 1],
      ]);
      assert.deepEqual(size.buckets, [
        [1, 0],
        [2, 0],
        [4, 0],
        [Infinity, 1],
      ]);
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});
