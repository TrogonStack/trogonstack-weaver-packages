// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";
import * as Duration from "effect/Duration";

const sampleDuration = Metric.histogram("sample.duration", {
  description: "Synthetic metric.",
  attributes: { unit: "s" },
  boundaries: [0.1, 0.5, 1, Infinity],
});

export function recordSampleDuration(value: Duration.Duration) {
  return Metric.update(sampleDuration, Duration.toMillis(value) / 1000);
}

const sampleSize = Metric.histogram("sample.size", {
  description: "Synthetic metric.",
  attributes: { unit: "By" },
  boundaries: [1, 2, 4, Infinity],
});

export function recordSampleSize(value: number) {
  return Metric.update(sampleSize, value);
}
