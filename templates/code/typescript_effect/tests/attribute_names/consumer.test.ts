import assert from "node:assert/strict";
import { test } from "node:test";
import * as Effect from "effect/Effect";
import * as Metric from "effect/Metric";

import * as Metrics from "./metrics.ts";

test("friendly identifiers preserve enum wire values and namespaced attribute keys", async () => {
  await Effect.runPromise(
    Effect.gen(function* () {
      yield* Metrics.updateSampleRecorded(1, {
        taskId: "synthetic-task",
        remoteTaskState: "ready",
        class: "synthetic-class",
        otherName: "synthetic-name",
        enabled: Metrics.SampleEnabled.Yes,
        numericCode: Metrics.SampleNumericCode.Two,
        quoteState: Metrics.SampleQuoteState.DoubleQuote,
      });
      const point = (yield* Metric.snapshot).find(
        (item) => item.id === "sample.recorded",
      );
      assert.ok(point?.type === "Counter");
      assert.equal(point.state.count, 1);
      assert.equal(point.attributes?.["sample.task.id"], "synthetic-task");
      assert.equal(point.attributes?.["remote.task.state"], "ready");
      assert.equal(point.attributes?.["sample.class"], "synthetic-class");
      assert.equal(point.attributes?.["sample.other_name"], "synthetic-name");
      assert.equal(point.attributes?.["sample.enabled"], "true");
      assert.equal(point.attributes?.["sample.numeric_code"], "2");
      assert.equal(point.attributes?.["sample.quote_state"], 'a"b');
    }).pipe(Effect.provideService(Metric.MetricRegistry, new Map())),
  );
});

function invalidConsumers() {
  // @ts-expect-error Enum constants are readonly.
  Metrics.SampleEnabled.Yes = false;
  // @ts-expect-error Numeric enum values are constrained to the registry.
  const invalidCode: Metrics.SampleNumericCode = 3;
  void invalidCode;
}
void invalidConsumers;
