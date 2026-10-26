import assert from "node:assert/strict";
import { test } from "node:test";
import * as Duration from "effect/Duration";
import * as Effect from "effect/Effect";
import * as Metric from "effect/Metric";
import { updateMeasurement } from "./metrics.ts";

test("a metric named measurement records durations through its input mapping", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* updateMeasurement(Duration.nanos(123n));
      yield* updateMeasurement(Duration.nanos(456n));
      const snapshot = yield* Metric.snapshot;
      const point = snapshot.find((item) => item.id === "measurement");
      assert.ok(point?.type === "Histogram");
      assert.equal(point.state.sum, 579);
      assert.equal(point.state.count, 2);
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});
