// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";

export type SamplePrototypeAttributes = {
  readonly proto: string;
};

const samplePrototype = Metric.counter("sample.prototype", {
  description: "Synthetic metric with unusual label keys.",
  attributes: { unit: "{item}" },
  incremental: true,
});

export function updateSamplePrototype(
  value: number,
  attributes: SamplePrototypeAttributes,
) {
  return Metric.update(
    Metric.withAttributes(samplePrototype, {
      ["__proto__"]: attributes.proto,
    }),
    value,
  );
}
