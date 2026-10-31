import assert from "node:assert/strict";
import { test } from "node:test";
import * as Attributes from "./attributes.ts";
import * as Metrics from "./metrics.ts";

test("deprecated and excluded metrics are not generated", () => {
  assert.equal("SampleDeprecated" in Metrics, false);
  assert.equal("SampleExcluded" in Metrics, false);
  assert.equal("SampleKept" in Metrics, true);
  assert.equal("SampleUnstable" in Metrics, true);
});

test("members of other stabilities are kept", () => {
  assert.equal(Attributes.SampleState.Pending, "pending");
});
