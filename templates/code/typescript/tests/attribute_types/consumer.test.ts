import assert from "node:assert/strict";
import { test } from "node:test";
import { createRecorder } from "../../support.ts";
import * as Attributes from "./attributes.ts";
import * as Metrics from "./metrics.ts";

test("every attribute type reaches the SDK under its wire key", async () => {
  const recorder = createRecorder();
  const runs = new Metrics.MyappJobRuns(Metrics.getMeter(recorder.meterProvider));
  const attemptNumbers = [1, 2];
  runs.add(1, {
    jobName: "sync",
    jobAttempt: 3,
    jobLoad: 0.5,
    jobRetried: true,
    jobTags: ["nightly", "reports"],
    jobAttemptNumbers: attemptNumbers,
    jobLoads: [0.5, 1.5],
    jobFlags: [true, false],
    jobState: Attributes.MyappJobState.Running,
    jobPriority: Attributes.MyappJobPriority.High,
    jobDryRun: Attributes.MyappJobDryRun.Off,
  });
  attemptNumbers.push(3);

  const [point] = (await recorder.metric("myapp.job.runs")).dataPoints;
  assert.deepEqual(point?.attributes, {
    "myapp.job.name": "sync",
    "myapp.job.attempt": 3,
    "myapp.job.load": 0.5,
    "myapp.job.retried": true,
    "myapp.job.tags": ["nightly", "reports"],
    "myapp.job.attempt_numbers": [1, 2],
    "myapp.job.loads": [0.5, 1.5],
    "myapp.job.flags": [true, false],
    "myapp.job.state": "running",
    "myapp.job.priority": 2,
    "myapp.job.dry_run": false,
  });
});

test("falsy attribute values are kept and undefined ones are dropped", async () => {
  const recorder = createRecorder();
  const pending = new Metrics.MyappJobPending(Metrics.getMeter(recorder.meterProvider));
  pending.add(1, { jobName: "", jobAttempt: 0, jobTags: [] });
  pending.add(1, { jobName: "sync", jobAttempt: undefined, jobState: undefined });

  const points = (await recorder.metric("myapp.job.pending")).dataPoints;
  assert.deepEqual(points.map((point) => point.attributes), [
    { "myapp.job.name": "", "myapp.job.attempt": 0, "myapp.job.tags": [] },
    { "myapp.job.name": "sync" },
  ]);
});

test("enum constants carry the wire values and the key", () => {
  assert.equal(Attributes.MyappJobState.KEY, "myapp.job.state");
  assert.deepEqual(Object.values(Attributes.MyappJobState), [
    "myapp.job.state",
    "queued",
    "running",
    "legacy",
  ]);
  assert.equal(Object.isFrozen(Attributes.MyappJobState), true);
  assert.equal(Attributes.MyappJobPriority.Low, 1);
  assert.equal(Attributes.MyappJobDryRun.On, true);
});

function invalidConsumers() {
  const pending = new Metrics.MyappJobPending(Metrics.getMeter());
  // @ts-expect-error A string enum stays closed.
  pending.add(1, { jobName: "sync", jobState: "unknown" });
  // @ts-expect-error A numeric enum stays closed.
  new Metrics.MyappJobRuns(Metrics.getMeter()).add(1, { ...allRunAttributes, jobPriority: 3 });
  // @ts-expect-error An int attribute is a number.
  pending.add(1, { jobName: "sync", jobAttempt: "1" });
  // @ts-expect-error An array attribute holds one element type.
  pending.add(1, { jobName: "sync", jobTags: [1] });
  // @ts-expect-error A required attribute cannot be omitted.
  pending.add(1, {});
  // @ts-expect-error A required attribute cannot be omitted.
  pending.add(1);
}
void invalidConsumers;

const allRunAttributes: Metrics.MyappJobRunsAttributes = {
  jobName: "sync",
  jobAttempt: 1,
  jobLoad: 1,
  jobRetried: false,
  jobTags: [],
  jobAttemptNumbers: [],
  jobLoads: [],
  jobFlags: [],
  jobState: Attributes.MyappJobState.Queued,
  jobPriority: Attributes.MyappJobPriority.Low,
  jobDryRun: Attributes.MyappJobDryRun.On,
};
