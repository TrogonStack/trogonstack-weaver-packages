import assert from "node:assert/strict";
import { test } from "node:test";
import * as Effect from "effect/Effect";
import * as Metric from "effect/Metric";

import * as Metrics from "./metrics.ts";

test("resolved dependency attributes retain enum types and serialize numeric labels", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* Metrics.recordSampleImported(1, {
        "remote.status": "ready",
        "remote.port": 443,
      });
      const state = yield* Metric.value(
        Metric.withAttributes(Metrics.sampleImported, {
          "remote.status": "ready",
          "remote.port": "443",
        }),
      );
      assert.equal(state.count, 1);
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});

function invalidConsumers() {
  Metrics.recordSampleImported(1, {
    // @ts-expect-error Dependency enum members remain constrained.
    "remote.status": "unknown",
  });
}
void invalidConsumers;
