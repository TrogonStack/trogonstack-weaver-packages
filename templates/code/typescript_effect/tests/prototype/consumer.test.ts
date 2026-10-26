import assert from "node:assert/strict";
import { test } from "node:test";
import * as Effect from "effect/Effect";
import * as Metric from "effect/Metric";
import * as Metrics from "./metrics.ts";

test("friendly fields preserve literal telemetry keys", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* Metrics.updateSamplePrototype(1, { proto: "synthetic" });
      const point = (yield* Metric.snapshot).find(
        (item) => item.id === "sample.prototype",
      );
      assert.ok(point?.type === "Counter");
      assert.equal(point.state.count, 1);
      assert.equal(point.attributes?.["__proto__"], "synthetic");
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});
