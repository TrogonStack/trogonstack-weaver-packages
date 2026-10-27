import assert from "node:assert/strict";
import { test } from "node:test";
import * as Data from "effect/Data";
import * as Effect from "effect/Effect";
import * as Fiber from "effect/Fiber";
import * as Metric from "effect/Metric";
import { TestClock } from "effect/testing";

import * as Metrics from "./metrics.ts";

class Timeout extends Data.TaggedError("Timeout")<{}> {}

function recorded(id: string) {
  return Effect.map(Metric.snapshot, (snapshot) =>
    snapshot.flatMap((point) =>
      point.id === id && point.type === "Histogram" ? [point] : [],
    ),
  );
}

function run(effect: Effect.Effect<void>) {
  return Effect.runPromise(
    effect.pipe(
      Effect.provideService(Metric.MetricRegistry, new Map()),
      Effect.provide(TestClock.layer()),
    ),
  );
}

test("track records the elapsed time and omits error.type on success", () =>
  run(
    Effect.gen(function* () {
      const fiber = yield* Effect.sleep("2 seconds").pipe(
        Metrics.trackMyappJobDuration({ jobName: "sync" }),
        Effect.forkChild,
      );
      yield* TestClock.adjust("2 seconds");
      yield* Fiber.join(fiber);
      const [point, ...rest] = yield* recorded("myapp.job.duration");
      assert.equal(rest.length, 0);
      assert.equal(point?.state.count, 1);
      assert.equal(point?.state.sum, 2);
      assert.equal(point?.attributes?.["myapp.job.name"], "sync");
      assert.equal(point?.attributes?.["error.type"], undefined);
    }),
  ));

for (const [name, effect, errorType] of [
  ["a tagged failure", Effect.fail(new Timeout()), "Timeout"],
  ["an untagged failure", Effect.fail("boom"), "_OTHER"],
  ["a defect", Effect.die(new Error("boom")), "_OTHER"],
  ["an interruption", Effect.interrupt, "interrupted"],
] as const) {
  test(`track fills error.type from ${name}`, () =>
    run(
      Effect.gen(function* () {
        yield* Effect.exit(
          (effect as Effect.Effect<never, unknown>).pipe(
            Metrics.trackMyappJobDuration({ jobName: "sync" }),
          ),
        );
        const [point, ...rest] = yield* recorded("myapp.job.duration");
        assert.equal(rest.length, 0);
        assert.equal(point?.state.count, 1);
        assert.equal(point?.attributes?.["myapp.job.name"], "sync");
        assert.equal(point?.attributes?.["error.type"], errorType);
      }),
    ));
}

test("track drops a caller-supplied error.type on success", () =>
  run(
    Effect.gen(function* () {
      const attributes: Metrics.MyappJobDurationAttributes = {
        jobName: "sync",
        errorType: "stale",
      };
      yield* Effect.void.pipe(Metrics.trackMyappJobDuration(attributes));
      const [point] = yield* recorded("myapp.job.duration");
      assert.equal(point?.attributes?.["error.type"], undefined);
    }),
  ));

test("track reads attributes exposed through getters", () =>
  run(
    Effect.gen(function* () {
      class Job {
        get jobName() {
          return "sync";
        }
      }
      yield* Effect.void.pipe(Metrics.trackMyappJobDuration(new Job()));
      const [point] = yield* recorded("myapp.job.duration");
      assert.equal(point?.attributes?.["myapp.job.name"], "sync");
    }),
  ));

test("track leaves a required error.type to the caller", () =>
  run(
    Effect.gen(function* () {
      yield* Effect.exit(
        Effect.fail(new Timeout()).pipe(
          Metrics.trackMyappJobRequiredErrorDuration({ errorType: "custom" }),
        ),
      );
      const [point] = yield* recorded("myapp.job.required_error.duration");
      assert.equal(point?.attributes?.["error.type"], "custom");
    }),
  ));

test("track works for metrics without attributes", () =>
  run(
    Effect.gen(function* () {
      const value = yield* Effect.succeed(42).pipe(
        Metrics.trackMyappJobUntaggedDuration(),
      );
      assert.equal(value, 42);
      const [point] = yield* recorded("myapp.job.untagged.duration");
      assert.equal(point?.state.count, 1);
    }),
  ));

function invalidConsumers() {
  // @ts-expect-error error.type is filled from the exit.
  Metrics.trackMyappJobDuration({ jobName: "sync", errorType: "Timeout" });
  // @ts-expect-error Required attributes stay required.
  Metrics.trackMyappJobDuration({});
}
void invalidConsumers;
