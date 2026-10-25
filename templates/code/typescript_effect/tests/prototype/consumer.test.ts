import assert from "node:assert/strict";
import { test } from "node:test";
import * as Effect from "effect/Effect";
import * as Metric from "effect/Metric";

import * as Metrics from "./metrics.ts";

test("prototype-related keys are labels and inherited optional properties are omitted", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      const attributes = { ["__proto__"]: "synthetic", constructor: undefined };
      Reflect.deleteProperty(attributes, "constructor");
      yield* Metrics.recordSamplePrototype(1, attributes);
      const snapshots = yield* Metric.snapshot;
      const point = snapshots.find((item) => item.id === "sample.prototype");
      assert.ok(point);
      assert.equal(point.attributes?.["__proto__"], "synthetic");
      assert.equal(Object.hasOwn(point.attributes ?? {}, "constructor"), false);
      const state = yield* Metric.value(
        Metric.withAttributes(Metrics.samplePrototype, [
          ["__proto__", "synthetic"],
        ]),
      );
      assert.equal(state.count, 1);
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});
