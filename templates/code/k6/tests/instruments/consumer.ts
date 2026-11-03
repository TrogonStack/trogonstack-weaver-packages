import {
  createSampleCount, createSampleDepth, createSampleLatency,
  createSampleFraction, createSampleSeconds, createSampleFractionalCount,
  METRIC_SAMPLE_COUNT, METRIC_SAMPLE_LATENCY, METRIC_SAMPLE_FRACTION,
} from "./metrics.ts";

const count = createSampleCount();
const depth = createSampleDepth();
const latency = createSampleLatency();
const fraction = createSampleFraction();
const seconds = createSampleSeconds();
const fractionalCount = createSampleFractionalCount();

export const options = {
  thresholds: {
    [METRIC_SAMPLE_COUNT]: [__ENV.EXPECT_THRESHOLD_FAILURE === "true" ? "count==999" : "count==3"],
    [METRIC_SAMPLE_LATENCY]: ["avg==37.5"],
    [METRIC_SAMPLE_FRACTION]: ["rate==0.5"],
  },
};

function rejects(action: () => unknown, text: string): void {
  try { action(); } catch (error) {
    if (error instanceof TypeError && error.message.includes(text)) return;
    throw error;
  }
  throw new Error("Expected rejection: " + text);
}

export default function () {
  const required = { "sample.label": "demo", "sample.attempt": 1 };
  for (const value of [NaN, Infinity, -Infinity, "2", true, [], null]) {
    // @ts-expect-error Runtime validation rejects unchecked nonnumeric values.
    rejects(() => count.add(value, required), "finite numbers");
  }
  rejects(() => count.add(Number.MAX_SAFE_INTEGER + 1, required), "safe integer");
  rejects(() => count.add(-1, required), "nonnegative");
  rejects(() => count.add(0.25, required), "safe integer");
  // @ts-expect-error Runtime validation rejects omitted required tags.
  rejects(() => count.add(1), "plain object");
  // @ts-expect-error Runtime validation rejects missing required properties.
  rejects(() => count.add(1, {}), "Missing required");
  // @ts-expect-error Runtime validation rejects extra tags.
  rejects(() => count.add(1, { ...required, extra: "value" }), "Unknown tag");
  rejects(() => count.add(1, { ...required, "sample.attempt": 0.5 }), "safe integer");
  rejects(() => count.add(1, { ...required, "sample.attempt": Number.MAX_SAFE_INTEGER + 1 }), "safe integer");
  // @ts-expect-error Runtime validation preserves the string enum kind.
  rejects(() => count.add(1, { ...required, "sample.state": 1 }), "Wrong value type");
  // @ts-expect-error Runtime validation rejects undeclared string enum members.
  rejects(() => count.add(1, { ...required, "sample.state": "missing" }), "Unknown enum");
  // @ts-expect-error Runtime validation preserves the numeric enum kind.
  rejects(() => count.add(1, { ...required, "sample.priority": "1" }), "finite numbers");
  // @ts-expect-error Runtime validation rejects undeclared numeric enum members.
  rejects(() => count.add(1, { ...required, "sample.priority": 3 }), "Unknown enum");
  // @ts-expect-error Runtime validation preserves the boolean enum kind.
  rejects(() => count.add(1, { ...required, "sample.switch": "true" }), "Wrong value type");
  // @ts-expect-error Runtime validation rejects array values.
  rejects(() => count.add(1, { ...required, "sample.label": ["demo"] }), "Wrong value type");
  rejects(() => count.add(1, { ...required, "sample.weight": Infinity }), "finite numbers");
  rejects(() => count.add(1, { ...required, get "sample.weight"() { return 1; } }), "data properties");
  rejects(() => count.add(1, Object.create(required)), "plain object");
  // @ts-expect-error Runtime validation rejects tags on an untagged metric.
  rejects(() => depth.record(1, { arbitrary: "tag" }), "Unknown tag");
  // @ts-expect-error Runtime validation rejects string coercion for rates.
  rejects(() => fraction.add("1"), "finite numbers");
  if (Object.keys(count).join() !== "add" || !Object.isFrozen(count)) throw new Error("Raw native handle exposed");
  try { createSampleDepth(); throw new Error("Accepted VU construction"); }
  catch (error) { if (!(error instanceof Error) || !error.message.includes("init context")) throw error; }

  if (count.add(2, { ...required, "sample.weight": 1e100, "sample.enabled": false,
    'sample.quoted"key': "quoted", "sample.state": "ready", "sample.priority": 2.5, "sample.switch": false }) !== true) throw new Error("Counter recording must return true");
  count.add(1, { ...required, "sample.state": "quote\" slash\\ line\nseparator\u2028end", "sample.weight": undefined });
  if (depth.record(7.5) !== true) throw new Error("Gauge recording must return true");
  depth.record(-2);
  depth.record(1e100);
  if (latency.record(25) !== true) throw new Error("Trend recording must return true");
  latency.record(50);
  if (fraction.add(0) !== true) throw new Error("Rate recording must return true");
  fraction.add(2);
  fraction.add(false);
  fraction.add(true);
  seconds.record(0.25);
  fractionalCount.add(0.5);
  fractionalCount.add(1.25);
}

export function handleSummary(data: unknown) {
  return { [__ENV.SUMMARY_PATH]: JSON.stringify(data) };
}
