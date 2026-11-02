import {
  createSampleCount, createSampleDepth, createSampleLatency,
  createSampleFraction, createSampleSeconds, createSampleFractionalCount,
  METRIC_SAMPLE_COUNT, METRIC_SAMPLE_LATENCY, METRIC_SAMPLE_FRACTION,
} from "./metrics.js";

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

function rejects(action, text) {
  try { action(); } catch (error) {
    if (error instanceof TypeError && error.message.includes(text)) return;
    throw error;
  }
  throw new Error("Expected rejection: " + text);
}

export default function () {
  const required = { "sample.label": "demo", "sample.attempt": 1 };
  for (const value of [NaN, Infinity, -Infinity, "2", true, [], null]) {
    rejects(() => count.add(value, required), "finite numbers");
  }
  rejects(() => count.add(Number.MAX_SAFE_INTEGER + 1, required), "safe integer");
  rejects(() => count.add(-1, required), "nonnegative");
  rejects(() => count.add(0.25, required), "safe integer");
  rejects(() => count.add(1), "plain object");
  rejects(() => count.add(1, {}), "Missing required");
  rejects(() => count.add(1, { ...required, extra: "value" }), "Unknown tag");
  rejects(() => count.add(1, { ...required, "sample.attempt": 0.5 }), "safe integer");
  rejects(() => count.add(1, { ...required, "sample.attempt": Number.MAX_SAFE_INTEGER + 1 }), "safe integer");
  rejects(() => count.add(1, { ...required, "sample.state": 1 }), "Wrong value type");
  rejects(() => count.add(1, { ...required, "sample.state": "missing" }), "Unknown enum");
  rejects(() => count.add(1, { ...required, "sample.priority": "1" }), "finite numbers");
  rejects(() => count.add(1, { ...required, "sample.priority": 3 }), "Unknown enum");
  rejects(() => count.add(1, { ...required, "sample.switch": "true" }), "Wrong value type");
  rejects(() => count.add(1, { ...required, "sample.label": ["demo"] }), "Wrong value type");
  rejects(() => count.add(1, { ...required, "sample.weight": Infinity }), "finite numbers");
  rejects(() => count.add(1, { ...required, get "sample.weight"() { return 1; } }), "data properties");
  rejects(() => count.add(1, Object.create(required)), "plain object");
  rejects(() => depth.record(1, { arbitrary: "tag" }), "Unknown tag");
  rejects(() => fraction.add("1"), "finite numbers");
  if (Object.keys(count).join() !== "add" || !Object.isFrozen(count)) throw new Error("Raw native handle exposed");
  try { createSampleDepth(); throw new Error("Accepted VU construction"); }
  catch (error) { if (!error.message.includes("init context")) throw error; }

  count.add(2, { ...required, "sample.weight": 1e100, "sample.enabled": false,
    'sample.quoted"key': "quoted", "sample.state": "ready", "sample.priority": 2.5, "sample.switch": false });
  count.add(1, { ...required, "sample.state": "quote\" slash\\ line\nseparator\u2028end", "sample.weight": undefined });
  depth.record(7.5);
  depth.record(-2);
  depth.record(1e100);
  latency.record(25);
  latency.record(50);
  fraction.add(0);
  fraction.add(2);
  fraction.add(false);
  fraction.add(true);
  seconds.record(0.25);
  fractionalCount.add(0.5);
  fractionalCount.add(1.25);
}

export function handleSummary(data) {
  return { [__ENV.SUMMARY_PATH]: JSON.stringify(data) };
}
