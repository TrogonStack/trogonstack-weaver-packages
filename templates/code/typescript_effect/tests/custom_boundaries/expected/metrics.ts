// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";
import * as Duration from "effect/Duration";

export type SampleDurationAttributes = Readonly<Record<string, never>>;

export const sampleDuration = Metric.mapInput(Metric.histogram("sample.duration", {
  description: "Synthetic metric.",
  attributes: { unit: "s" },
  boundaries: [...[0.1,0.5,1], Infinity],
}), (value: Duration.Duration) => Duration.toMillis(value) / 1000);

export const recordSampleDuration = (
  value: Duration.Duration,
  _attributes: SampleDurationAttributes = {},
) =>
  Metric.update(
    sampleDuration,
    value,
  );

export type SampleSizeAttributes = Readonly<Record<string, never>>;

export const sampleSize = Metric.histogram("sample.size", {
  description: "Synthetic metric.",
  attributes: { unit: "By" },
  boundaries: [...[1,2,4], Infinity],
});

export const recordSampleSize = (
  value: number,
  _attributes: SampleSizeAttributes = {},
) =>
  Metric.update(
    sampleSize,
    value,
  );
