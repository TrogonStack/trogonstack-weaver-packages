// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";
import * as Duration from "effect/Duration";

const sampleDurationMs = Metric.histogram("sample.duration.ms", {
  description: "Synthetic metric.",
  attributes: { unit: "ms" },
  boundaries: [5, 10, 25, 50, 75, 100, 250, 500, 750, 1000, 2500, 5000, 7500, 10000, Infinity],
});

export const recordSampleDurationMs = (value: Duration.Duration) =>
  Metric.update(sampleDurationMs, Duration.toMillis(value));

const sampleDurationNs = Metric.histogram("sample.duration.ns", {
  description: "Synthetic metric.",
  attributes: { unit: "ns" },
  boundaries: [5000000, 10000000, 25000000, 50000000, 75000000, 100000000, 250000000, 500000000, 750000000, 1000000000, 2500000000, 5000000000, 7500000000, 10000000000, Infinity],
});

export const recordSampleDurationNs = (value: Duration.Duration) =>
  Metric.update(sampleDurationNs, Duration.toMillis(value) * 1_000_000);

const sampleDurationS = Metric.histogram("sample.duration.s", {
  description: "Synthetic metric.",
  attributes: { unit: "s" },
  boundaries: [0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1, 2.5, 5, 7.5, 10, Infinity],
});

export const recordSampleDurationS = (value: Duration.Duration) =>
  Metric.update(sampleDurationS, Duration.toMillis(value) / 1000);

const sampleDurationUs = Metric.histogram("sample.duration.us", {
  description: "Synthetic metric.",
  attributes: { unit: "us" },
  boundaries: [5000, 10000, 25000, 50000, 75000, 100000, 250000, 500000, 750000, 1000000, 2500000, 5000000, 7500000, 10000000, Infinity],
});

export const recordSampleDurationUs = (value: Duration.Duration) =>
  Metric.update(sampleDurationUs, Duration.toMillis(value) * 1000);

const sampleSize = Metric.histogram("sample.size", {
  description: "Synthetic metric.",
  attributes: { unit: "By" },
  boundaries: [0, 1, 5, 10, 25, 50, 100, 250, 500, 1000, Infinity],
});

export const recordSampleSize = (value: number) =>
  Metric.update(sampleSize, value);
