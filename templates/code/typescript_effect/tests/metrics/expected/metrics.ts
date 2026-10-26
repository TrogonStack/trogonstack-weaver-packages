// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";
import * as Duration from "effect/Duration";

export const AuthMethod = {
  Password: "password",
  Token: "token",
} as const;

export type AuthMethod = (typeof AuthMethod)[keyof typeof AuthMethod];

export const MyappTaskState = {
  Done: "done",
  Queued: "queued",
} as const;

export type MyappTaskState = (typeof MyappTaskState)[keyof typeof MyappTaskState];

export type AuthAttemptsAttributes = {
  readonly method: AuthMethod;
  readonly success: boolean;
};

const authAttempts = Metric.counter("auth.attempts", {
  description: "Number of authentication attempts.",
  attributes: { unit: "{attempt}" },
  incremental: true,
});

export function updateAuthAttempts(
  value: number,
  attributes: AuthAttemptsAttributes,
) {
  return Metric.update(
    Metric.withAttributes(authAttempts, {
      "auth.method": attributes.method,
      "auth.success": String(attributes.success),
    }),
    value,
  );
}

const myappQueueDepth = Metric.gauge("myapp.queue.depth", {
  description: "Number of tasks waiting in the queue.",
  attributes: { unit: "{task}" },
});

export function updateMyappQueueDepth(value: number) {
  return Metric.update(myappQueueDepth, value);
}

export type MyappTaskActiveAttributes = {
  readonly taskState: MyappTaskState;
};

const myappTaskActive = Metric.counter("myapp.task.active", {
  description: "Number of tasks currently running.",
  attributes: { unit: "{task}" },
  incremental: false,
});

export function updateMyappTaskActive(
  value: number,
  attributes: MyappTaskActiveAttributes,
) {
  return Metric.update(
    Metric.withAttributes(myappTaskActive, {
      "myapp.task.state": attributes.taskState,
    }),
    value,
  );
}

export type MyappTaskDurationAttributes = {
  readonly authMethod?: AuthMethod;
  readonly taskId: string;
  readonly taskState?: MyappTaskState;
};

const myappTaskDuration = Metric.mapInput(
  Metric.histogram("myapp.task.duration", {
    description: "Time a task took from start to finish.",
    attributes: { unit: "s" },
    boundaries: [0, 0.005, 0.01, 0.1, 1, 10, Infinity],
  }),
  Duration.toSeconds,
);

export function updateMyappTaskDuration(
  value: Duration.Duration,
  attributes: MyappTaskDurationAttributes,
) {
  const labels: Record<string, string> = {
    "myapp.task.id": attributes.taskId,
  };
  if (attributes.authMethod !== undefined) {
    labels["auth.method"] = attributes.authMethod;
  }
  if (attributes.taskState !== undefined) {
    labels["myapp.task.state"] = attributes.taskState;
  }
  return Metric.update(
    Metric.withAttributes(myappTaskDuration, labels),
    value,
  );
}

const myappTaskFloatActive = Metric.counter("myapp.task.float.active", {
  description: "Fractional active task adjustment.",
  attributes: { unit: "{task}" },
  incremental: false,
});

export function updateMyappTaskFloatActive(value: number) {
  return Metric.update(myappTaskFloatActive, value);
}

const myappTaskFloatDepth = Metric.gauge("myapp.task.float.depth", {
  description: "Fractional queue depth.",
  attributes: { unit: "{task}" },
});

export function updateMyappTaskFloatDepth(value: number) {
  return Metric.update(myappTaskFloatDepth, value);
}

const myappTaskFloatStarted = Metric.counter("myapp.task.float.started", {
  description: "Fractional tasks started.",
  attributes: { unit: "{task}" },
  incremental: true,
});

export function updateMyappTaskFloatStarted(value: number) {
  return Metric.update(myappTaskFloatStarted, value);
}

const myappTaskPayload = Metric.histogram("myapp.task.payload", {
  description: "Task payload size.",
  attributes: { unit: "By" },
  boundaries: [128, 256, 1024, Infinity],
});

export function updateMyappTaskPayload(value: number) {
  return Metric.update(myappTaskPayload, value);
}

export type MyappTaskStartedAttributes = {
  readonly taskId?: string;
};

const myappTaskStarted = Metric.counter("myapp.task.started", {
  description: "Number of tasks started.",
  attributes: { unit: "{task}" },
  incremental: true,
});

export function updateMyappTaskStarted(
  value: number,
  attributes: MyappTaskStartedAttributes = {},
) {
  const labels: Record<string, string> = {};
  if (attributes.taskId !== undefined) {
    labels["myapp.task.id"] = attributes.taskId;
  }
  return Metric.update(
    Metric.withAttributes(myappTaskStarted, labels),
    value,
  );
}
