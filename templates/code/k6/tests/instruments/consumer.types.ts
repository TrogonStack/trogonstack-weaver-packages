import {
  createSampleCount, createSampleDepth, createSampleFraction, METRIC_SAMPLE_COUNT,
} from "./metrics.ts";
import type { SampleCountTags } from "./metrics.ts";
const count = createSampleCount();
const required: SampleCountTags = { "sample.label": "demo", "sample.attempt": 1 };
count.add(1, required);
count.add(1, { ...required, "sample.state": "ready", "sample.priority": 2.5, "sample.switch": false, "sample.weight": undefined });
const name: "sample_count" = METRIC_SAMPLE_COUNT;
void name;
// @ts-expect-error Required tags cannot be omitted.
count.add(1);
// @ts-expect-error Every required tag must be present.
count.add(1, { "sample.label": "demo" });
// @ts-expect-error Metric values have a numeric kind.
count.add("1", required);
// @ts-expect-error Counters do not accept boolean values.
count.add(true, required);
// @ts-expect-error String enum tags reject numeric members.
count.add(1, { ...required, "sample.state": 1 });
// @ts-expect-error String enum tags reject undeclared strings.
count.add(1, { ...required, "sample.state": "unknown" });
// @ts-expect-error Numeric enum tags reject strings.
count.add(1, { ...required, "sample.priority": "1" });
// @ts-expect-error Numeric enum tags reject undeclared numbers.
count.add(1, { ...required, "sample.priority": 3 });
// @ts-expect-error Boolean enum tags reject strings.
count.add(1, { ...required, "sample.switch": "false" });
// @ts-expect-error Tags reject arrays.
count.add(1, { ...required, "sample.label": ["demo"] });
// @ts-expect-error Unknown tags are not part of the API.
count.add(1, { ...required, arbitrary: "tag" });
// @ts-expect-error Native instruments are private to the closure.
count.instrument.add(1, { arbitrary: "tag" });
// @ts-expect-error Native handles are not exposed.
count.native;
// @ts-expect-error Zero-attribute metrics do not offer arbitrary tags.
createSampleDepth().record(1, { arbitrary: "tag" });
createSampleDepth().record(-1);
createSampleFraction().add(true);
// @ts-expect-error Rate does not accept string coercion.
createSampleFraction().add("1");

import { Counter } from "k6/metrics";
import type { SampleCount, SampleLatency } from "./metrics.ts";
// @ts-expect-error Native instruments do not have the generated metric identity.
const raw: SampleCount = new Counter("raw_count");
// @ts-expect-error Structurally identical helpers retain distinct metric identities.
const wrongMetric: SampleLatency = createSampleDepth();
void raw;
void wrongMetric;

// @ts-expect-error The metric identity is private to the generated module.
import { metricIdentity } from "./metrics.ts";
void metricIdentity;
