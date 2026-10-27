// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";
import * as Cause from "effect/Cause";
import * as Clock from "effect/Clock";
import * as Duration from "effect/Duration";
import * as Effect from "effect/Effect";
import * as Exit from "effect/Exit";
import * as Option from "effect/Option";

function errorTypeOf(cause: Cause.Cause<unknown>): string {
  const error = Cause.findErrorOption(cause);
  if (Option.isNone(error)) {
    return Cause.hasInterruptsOnly(cause) ? "interrupted" : "_OTHER";
  }
  const failure = error.value;
  return typeof failure === "object" && failure !== null && "_tag" in failure && typeof failure._tag === "string"
    ? failure._tag
    : "_OTHER";
}

export type MyappJobDurationAttributes = {
  readonly errorType?: string;
  readonly jobName: string;
};

const myappJobDuration = Metric.mapInput(
  Metric.histogram("myapp.job.duration", {
    description: "Synthetic job duration.",
    attributes: { unit: "s" },
    boundaries: [0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1, 2.5, 5, 7.5, 10, Infinity],
  }),
  Duration.toSeconds,
);

export function updateMyappJobDuration(
  value: Duration.Duration,
  attributes: MyappJobDurationAttributes,
) {
  const labels: Record<string, string> = {
    "myapp.job.name": attributes.jobName,
  };
  if (attributes.errorType !== undefined) {
    labels["error.type"] = attributes.errorType;
  }
  return Metric.update(
    Metric.withAttributes(myappJobDuration, labels),
    value,
  );
}

export function trackMyappJobDuration(
  attributes: Omit<MyappJobDurationAttributes, "errorType">,
) {
  return <A, E, R>(effect: Effect.Effect<A, E, R>) =>
    Effect.flatMap(Clock.monotonicTimeNanos, (start) =>
      Effect.onExit(effect, (exit) =>
        Effect.flatMap(Clock.monotonicTimeNanos, (end) =>
          updateMyappJobDuration(
            Duration.nanos(end - start),
            Exit.isSuccess(exit) ? attributes : { ...attributes, errorType: errorTypeOf(exit.cause) },
          ),
        ),
      ),
    );
}

export type MyappJobRequiredErrorDurationAttributes = {
  readonly errorType: string;
};

const myappJobRequiredErrorDuration = Metric.mapInput(
  Metric.histogram("myapp.job.required_error.duration", {
    description: "Synthetic duration whose error type the caller always sets.",
    attributes: { unit: "ms" },
    boundaries: [5, 10, 25, 50, 75, 100, 250, 500, 750, 1000, 2500, 5000, 7500, 10000, Infinity],
  }),
  Duration.toMillis,
);

export function updateMyappJobRequiredErrorDuration(
  value: Duration.Duration,
  attributes: MyappJobRequiredErrorDurationAttributes,
) {
  return Metric.update(
    Metric.withAttributes(myappJobRequiredErrorDuration, {
      "error.type": attributes.errorType,
    }),
    value,
  );
}

export function trackMyappJobRequiredErrorDuration(
  attributes: MyappJobRequiredErrorDurationAttributes,
) {
  return <A, E, R>(effect: Effect.Effect<A, E, R>) =>
    Effect.flatMap(Clock.monotonicTimeNanos, (start) =>
      Effect.onExit(effect, () =>
        Effect.flatMap(Clock.monotonicTimeNanos, (end) =>
          updateMyappJobRequiredErrorDuration(
            Duration.nanos(end - start),
            attributes,
          ),
        ),
      ),
    );
}

const myappJobUntaggedDuration = Metric.mapInput(
  Metric.histogram("myapp.job.untagged.duration", {
    description: "Synthetic duration without attributes.",
    attributes: { unit: "ms" },
    boundaries: [5, 10, 25, 50, 75, 100, 250, 500, 750, 1000, 2500, 5000, 7500, 10000, Infinity],
  }),
  Duration.toMillis,
);

export function updateMyappJobUntaggedDuration(value: Duration.Duration) {
  return Metric.update(myappJobUntaggedDuration, value);
}

export function trackMyappJobUntaggedDuration() {
  return <A, E, R>(effect: Effect.Effect<A, E, R>) =>
    Effect.flatMap(Clock.monotonicTimeNanos, (start) =>
      Effect.onExit(effect, () =>
        Effect.flatMap(Clock.monotonicTimeNanos, (end) =>
          updateMyappJobUntaggedDuration(
            Duration.nanos(end - start),
          ),
        ),
      ),
    );
}
