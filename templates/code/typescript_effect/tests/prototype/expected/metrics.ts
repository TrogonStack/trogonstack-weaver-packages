// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";

export type SamplePrototypeAttributes = {
  readonly proto: string;
  readonly constructor?: string;
};

const samplePrototype = Metric.counter("sample.prototype", {
  description: "Synthetic metric with unusual label keys.",
  attributes: { unit: "{item}" },
  incremental: true,
});

export const recordSamplePrototype = (
  value: number,
  attributes: SamplePrototypeAttributes,
) =>
  Metric.update(
    Metric.withAttributes(samplePrototype, {
      ["__proto__"]: String(attributes.proto),
      ...(!Object.hasOwn(attributes, "constructor") || attributes.constructor === undefined
        ? {}
        : { ["constructor"]: String(attributes.constructor) }),
    }),
    value,
  );
