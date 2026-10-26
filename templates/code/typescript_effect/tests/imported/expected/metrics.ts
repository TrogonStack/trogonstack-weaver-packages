// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";

export const RemoteStatus = {
  Done: "done",
  Ready: "ready",
} as const;

export type RemoteStatus = (typeof RemoteStatus)[keyof typeof RemoteStatus];

export type SampleImportedAttributes = {
  readonly remotePort?: number;
  readonly remoteStatus: RemoteStatus;
};

const sampleImported = Metric.counter("sample.imported", {
  description: "Synthetic metric.",
  attributes: { unit: "{item}" },
  incremental: true,
});

export function updateSampleImported(
  value: number,
  attributes: SampleImportedAttributes,
) {
  const labels: Record<string, string> = {
    "remote.status": attributes.remoteStatus,
  };
  if (attributes.remotePort !== undefined) {
    labels["remote.port"] = String(attributes.remotePort);
  }
  return Metric.update(
    Metric.withAttributes(sampleImported, labels),
    value,
  );
}
