import assert from "node:assert/strict";
import { test } from "node:test";
import * as Effect from "effect/Effect";
import * as Metric from "effect/Metric";
import * as Metrics from "./metrics.ts";

test("dependency enums and friendly fields retain exact wire keys", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* Metrics.updateSampleImported(1, {
        remoteStatus: Metrics.RemoteStatus.Ready,
        remotePort: 443,
      });
      const point = (yield* Metric.snapshot).find(
        (item) => item.id === "sample.imported",
      );
      assert.ok(point?.type === "Counter");
      assert.equal(point.state.count, 1);
      assert.equal(point.attributes?.["remote.status"], "ready");
      assert.equal(point.attributes?.["remote.port"], "443");
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});

test("optional numeric labels omit undefined and preserve zero", async () => {
  for (const remotePort of [undefined, 0]) {
    await Effect.runPromise(
      Effect.gen(function* () {
        yield* Metrics.updateSampleImported(1, {
          remoteStatus: Metrics.RemoteStatus.Ready,
          remotePort,
        });
        const point = (yield* Metric.snapshot).find(
          (item) => item.id === "sample.imported",
        );
        assert.ok(point?.type === "Counter");
        assert.equal(
          point.attributes?.["remote.port"],
          remotePort === undefined ? undefined : "0",
        );
      }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
    );
  }
});

function invalidConsumers() {
  Metrics.updateSampleImported(1, {
    // @ts-expect-error Imported enum members remain constrained.
    remoteStatus: "unknown",
  });
}
void invalidConsumers;
