import assert from "node:assert/strict";
import { test } from "node:test";
import * as Duration from "effect/Duration";
import * as Effect from "effect/Effect";
import * as Metric from "effect/Metric";
import * as Metrics from "./metrics.ts";

test("unit-specific boundaries override fallback boundaries", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* Metrics.updateSampleDuration(Duration.seconds(2));
      yield* Metrics.updateSampleSize(5);
      const snapshot = yield* Metric.snapshot;
      const duration = snapshot.find((item) => item.id === "sample.duration");
      const size = snapshot.find((item) => item.id === "sample.size");
      assert.ok(duration?.type === "Histogram");
      assert.ok(size?.type === "Histogram");
      assert.deepEqual(duration.state.buckets, [
        [0.1, 0],
        [0.5, 0],
        [1, 0],
        [Infinity, 1],
      ]);
      assert.deepEqual(size.state.buckets, [
        [1, 0],
        [2, 0],
        [4, 0],
        [Infinity, 1],
      ]);
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});
