// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";

export type SampleImportedAttributes = {
  readonly "remote.port"?: number;
  readonly "remote.status": "done" | "ready";
};

export const sampleImported = Metric.counter("sample.imported", {
  description: "Synthetic metric.",
  attributes: { unit: "{item}" },
  incremental: true,
});

export const recordSampleImported = (
  value: number,
  attributes: SampleImportedAttributes,
) =>
  Metric.update(
    Metric.withAttributes(sampleImported, {
      ...(!Object.hasOwn(attributes, "remote.port") || attributes["remote.port"] === undefined
        ? {}
        : { ["remote.port"]: String(attributes["remote.port"]) }),
      ["remote.status"]: String(attributes["remote.status"]),
    }),
    value,
  );
