import assert from "node:assert/strict";
import { test } from "node:test";
import * as Attributes from "./attributes.ts";
import * as Metrics from "./metrics.ts";

test("only stable metrics and enum members are generated", () => {
  assert.equal("SampleUnstable" in Metrics, false);
  assert.equal("SampleExcluded" in Metrics, false);
  assert.equal("SampleKept" in Metrics, true);
  assert.equal("SampleDeprecated" in Metrics, true);
  assert.equal("Pending" in Attributes.SampleState, false);
});
