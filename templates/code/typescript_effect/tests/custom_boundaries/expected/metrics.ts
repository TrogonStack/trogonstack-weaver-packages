// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";
import * as Clock from "effect/Clock";
import * as Duration from "effect/Duration";
import * as Effect from "effect/Effect";

const sampleDuration = Metric.mapInput(
  Metric.histogram("sample.duration", {
    description: "Synthetic metric.",
    attributes: { unit: "s" },
    boundaries: [0.1, 0.5, 1, Infinity],
  }),
  Duration.toSeconds,
);

export function updateSampleDuration(value: Duration.Duration) {
  return Metric.update(sampleDuration, value);
}

export function trackDurationSampleDuration() {
  return <A, E, R>(effect: Effect.Effect<A, E, R>) =>
    Effect.flatMap(Clock.monotonicTimeNanos, (start) =>
      Effect.onExit(effect, () =>
        Effect.flatMap(Clock.monotonicTimeNanos, (end) =>
          updateSampleDuration(
            Duration.nanos(end - start),
          ),
        ),
      ),
    );
}

const sampleSize = Metric.histogram("sample.size", {
  description: "Synthetic metric.",
  attributes: { unit: "By" },
  boundaries: [1, 2, 4, Infinity],
});

export function updateSampleSize(value: number) {
  return Metric.update(sampleSize, value);
}
