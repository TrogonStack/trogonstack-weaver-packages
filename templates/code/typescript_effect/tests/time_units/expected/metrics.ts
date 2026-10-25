// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";
import * as Duration from "effect/Duration";

export type SampleDurationMsAttributes = Readonly<Record<string, never>>;

export const sampleDurationMs = Metric.mapInput(Metric.histogram("sample.duration.ms", {
  description: "Synthetic metric.",
  attributes: { unit: "ms" },
  boundaries: [...[5,10,25,50,75,100,250,500,750,1000,2500,5000,7500,10000], Infinity],
}), (value: Duration.Duration) => Duration.toMillis(value));

export const recordSampleDurationMs = (
  value: Duration.Duration,
  _attributes: SampleDurationMsAttributes = {},
) =>
  Metric.update(
    sampleDurationMs,
    value,
  );

export type SampleDurationNsAttributes = Readonly<Record<string, never>>;

export const sampleDurationNs = Metric.mapInput(Metric.histogram("sample.duration.ns", {
  description: "Synthetic metric.",
  attributes: { unit: "ns" },
  boundaries: [...[5000000,10000000,25000000,50000000,75000000,100000000,250000000,500000000,750000000,1000000000,2500000000,5000000000,7500000000,10000000000], Infinity],
}), (value: Duration.Duration) => Duration.toMillis(value) * 1_000_000);

export const recordSampleDurationNs = (
  value: Duration.Duration,
  _attributes: SampleDurationNsAttributes = {},
) =>
  Metric.update(
    sampleDurationNs,
    value,
  );

export type SampleDurationSAttributes = Readonly<Record<string, never>>;

export const sampleDurationS = Metric.mapInput(Metric.histogram("sample.duration.s", {
  description: "Synthetic metric.",
  attributes: { unit: "s" },
  boundaries: [...[0.005,0.01,0.025,0.05,0.075,0.1,0.25,0.5,0.75,1,2.5,5,7.5,10], Infinity],
}), (value: Duration.Duration) => Duration.toMillis(value) / 1000);

export const recordSampleDurationS = (
  value: Duration.Duration,
  _attributes: SampleDurationSAttributes = {},
) =>
  Metric.update(
    sampleDurationS,
    value,
  );

export type SampleDurationUsAttributes = Readonly<Record<string, never>>;

export const sampleDurationUs = Metric.mapInput(Metric.histogram("sample.duration.us", {
  description: "Synthetic metric.",
  attributes: { unit: "us" },
  boundaries: [...[5000,10000,25000,50000,75000,100000,250000,500000,750000,1000000,2500000,5000000,7500000,10000000], Infinity],
}), (value: Duration.Duration) => Duration.toMillis(value) * 1000);

export const recordSampleDurationUs = (
  value: Duration.Duration,
  _attributes: SampleDurationUsAttributes = {},
) =>
  Metric.update(
    sampleDurationUs,
    value,
  );

export type SampleSizeAttributes = Readonly<Record<string, never>>;

export const sampleSize = Metric.histogram("sample.size", {
  description: "Synthetic metric.",
  attributes: { unit: "By" },
  boundaries: [...[0,1,5,10,25,50,100,250,500,1000], Infinity],
});

export const recordSampleSize = (
  value: number,
  _attributes: SampleSizeAttributes = {},
) =>
  Metric.update(
    sampleSize,
    value,
  );
