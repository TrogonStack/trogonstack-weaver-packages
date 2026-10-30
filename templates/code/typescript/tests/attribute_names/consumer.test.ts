import assert from "node:assert/strict";
import { test } from "node:test";
import { createRecorder } from "../../support.ts";
import * as Attributes from "./attributes.ts";
import * as Metrics from "./metrics.ts";

test("awkward enum values reach the SDK unchanged", async () => {
  const recorder = createRecorder();
  new Metrics.SampleRecorded(new Metrics.Meter(recorder.meterProvider)).add(1, {
    taskId: "a",
    otherName: "b",
    remoteTaskState: "c",
    class: "d",
    quoteState: Attributes.SampleQuoteState.Newline,
    numericCode: Attributes.SampleNumericCode.Two,
    enabled: Attributes.SampleEnabled.No,
  });
  const [point] = (await recorder.metric("sample.recorded")).dataPoints;
  assert.deepEqual(point?.attributes, {
    "sample.task.id": "a",
    "sample.other_name": "b",
    "remote.task.state": "c",
    "sample.class": "d",
    "sample.quote_state": "a\nb",
    "sample.numeric_code": 2,
    "sample.enabled": false,
  });
  assert.deepEqual(
    [Attributes.SampleQuoteState.DoubleQuote, Attributes.SampleQuoteState.Backslash],
    ['a"b', "a\\b"],
  );
});
