// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";

export type SampleKeptAttributes = {
  readonly "sample.state": "ready";
};

export const sampleKept = Metric.counter("sample.kept", {
  description: "Kept metric.",
  attributes: { unit: "{item}" },
  incremental: true,
});

export const recordSampleKept = (
  value: number,
  attributes: SampleKeptAttributes,
) =>
  Metric.update(
    Metric.withAttributes(sampleKept, {
      ["sample.state"]: String(attributes["sample.state"]),
    }),
    value,
  );
