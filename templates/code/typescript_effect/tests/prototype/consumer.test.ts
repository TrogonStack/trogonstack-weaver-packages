import assert from "node:assert/strict";
import { test } from "node:test";
import * as Effect from "effect/Effect";
import * as Metric from "effect/Metric";
import * as Metrics from "./metrics.ts";

test("friendly fields preserve unusual wire keys without inherited optional labels", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      const attributes = { proto: "synthetic", constructor: undefined };
      Reflect.deleteProperty(attributes, "constructor");
      yield* Metrics.recordSamplePrototype(1, attributes);
      const point = (yield* Metric.snapshot).find(
        (item) => item.id === "sample.prototype",
      );
      assert.ok(point?.type === "Counter");
      assert.equal(point.state.count, 1);
      assert.equal(point.attributes?.["__proto__"], "synthetic");
      assert.equal(Object.hasOwn(point.attributes ?? {}, "constructor"), false);
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});
