// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";
import * as Duration from "effect/Duration";

export type AuthAttemptsAttributes = {
  readonly "auth.method": "password" | "token";
  readonly "auth.success": boolean;
};

export const authAttempts = Metric.counter("auth.attempts", {
  description: "Number of authentication attempts.",
  attributes: { unit: "{attempt}" },
  incremental: true,
});

export const recordAuthAttempts = (
  value: number,
  attributes: AuthAttemptsAttributes,
) =>
  Metric.update(
    Metric.withAttributes(authAttempts, {
      ["auth.method"]: String(attributes["auth.method"]),
      ["auth.success"]: String(attributes["auth.success"]),
    }),
    value,
  );

export type MyappQueueDepthAttributes = Readonly<Record<string, never>>;

export const myappQueueDepth = Metric.gauge("myapp.queue.depth", {
  description: "Number of tasks waiting in the queue.",
  attributes: { unit: "{task}" },
});

export const recordMyappQueueDepth = (
  value: number,
  _attributes: MyappQueueDepthAttributes = {},
) =>
  Metric.update(
    myappQueueDepth,
    value,
  );

export type MyappTaskActiveAttributes = {
  readonly "myapp.task.state": "done" | "queued";
};

export const myappTaskActive = Metric.counter("myapp.task.active", {
  description: "Number of tasks currently running.",
  attributes: { unit: "{task}" },
  incremental: false,
});

export const recordMyappTaskActive = (
  value: number,
  attributes: MyappTaskActiveAttributes,
) =>
  Metric.update(
    Metric.withAttributes(myappTaskActive, {
      ["myapp.task.state"]: String(attributes["myapp.task.state"]),
    }),
    value,
  );

export type MyappTaskDurationAttributes = {
  readonly "auth.method"?: "password" | "token";
  readonly "myapp.task.id": string;
  readonly "myapp.task.state"?: "done" | "queued";
};

export const myappTaskDuration = Metric.mapInput(Metric.histogram("myapp.task.duration", {
  description: "Time a task took from start to finish.",
  attributes: { unit: "s" },
  boundaries: [...[0,0.005,0.01,0.1,1,10], Infinity],
}), (value: Duration.Duration) => Duration.toMillis(value) / 1000);

export const recordMyappTaskDuration = (
  value: Duration.Duration,
  attributes: MyappTaskDurationAttributes,
) =>
  Metric.update(
    Metric.withAttributes(myappTaskDuration, {
      ...(!Object.hasOwn(attributes, "auth.method") || attributes["auth.method"] === undefined
        ? {}
        : { ["auth.method"]: String(attributes["auth.method"]) }),
      ["myapp.task.id"]: String(attributes["myapp.task.id"]),
      ...(!Object.hasOwn(attributes, "myapp.task.state") || attributes["myapp.task.state"] === undefined
        ? {}
        : { ["myapp.task.state"]: String(attributes["myapp.task.state"]) }),
    }),
    value,
  );

export type MyappTaskFloatActiveAttributes = Readonly<Record<string, never>>;

export const myappTaskFloatActive = Metric.counter("myapp.task.float.active", {
  description: "Fractional active task adjustment.",
  attributes: { unit: "{task}" },
  incremental: false,
});

export const recordMyappTaskFloatActive = (
  value: number,
  _attributes: MyappTaskFloatActiveAttributes = {},
) =>
  Metric.update(
    myappTaskFloatActive,
    value,
  );

export type MyappTaskFloatDepthAttributes = Readonly<Record<string, never>>;

export const myappTaskFloatDepth = Metric.gauge("myapp.task.float.depth", {
  description: "Fractional queue depth.",
  attributes: { unit: "{task}" },
});

export const recordMyappTaskFloatDepth = (
  value: number,
  _attributes: MyappTaskFloatDepthAttributes = {},
) =>
  Metric.update(
    myappTaskFloatDepth,
    value,
  );

export type MyappTaskFloatStartedAttributes = Readonly<Record<string, never>>;

export const myappTaskFloatStarted = Metric.counter("myapp.task.float.started", {
  description: "Fractional tasks started.",
  attributes: { unit: "{task}" },
  incremental: true,
});

export const recordMyappTaskFloatStarted = (
  value: number,
  _attributes: MyappTaskFloatStartedAttributes = {},
) =>
  Metric.update(
    myappTaskFloatStarted,
    value,
  );

export type MyappTaskPayloadAttributes = Readonly<Record<string, never>>;

export const myappTaskPayload = Metric.histogram("myapp.task.payload", {
  description: "Task payload size.",
  attributes: { unit: "By" },
  boundaries: [...[128,256,1024], Infinity],
});

export const recordMyappTaskPayload = (
  value: number,
  _attributes: MyappTaskPayloadAttributes = {},
) =>
  Metric.update(
    myappTaskPayload,
    value,
  );

export type MyappTaskStartedAttributes = {
  readonly "myapp.task.id"?: string;
};

export const myappTaskStarted = Metric.counter("myapp.task.started", {
  description: "Number of tasks started.",
  attributes: { unit: "{task}" },
  incremental: true,
});

export const recordMyappTaskStarted = (
  value: number,
  attributes: MyappTaskStartedAttributes = {},
) =>
  Metric.update(
    Metric.withAttributes(myappTaskStarted, {
      ...(!Object.hasOwn(attributes, "myapp.task.id") || attributes["myapp.task.id"] === undefined
        ? {}
        : { ["myapp.task.id"]: String(attributes["myapp.task.id"]) }),
    }),
    value,
  );
