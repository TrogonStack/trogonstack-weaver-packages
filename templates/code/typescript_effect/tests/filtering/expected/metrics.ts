// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";

export const SampleState = {
  Ready: "ready",
} as const;

export type SampleState = (typeof SampleState)[keyof typeof SampleState];

export type SampleKeptAttributes = {
  readonly state: SampleState;
};

const sampleKept = Metric.counter("sample.kept", {
  description: "Kept metric.",
  attributes: { unit: "{item}" },
  incremental: true,
});

export function updateSampleKept(
  value: number,
  attributes: SampleKeptAttributes,
) {
  return Metric.update(
    Metric.withAttributes(sampleKept, {
      ["sample.state"]: String(attributes.state),
    }),
    value,
  );
}
