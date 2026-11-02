import { createSampleDuration, createSampleRetryDuration, METRIC_SAMPLE_RETRY_DURATION } from "./metrics.ts";
const duration = createSampleDuration();
const retry = createSampleRetryDuration();
export const options = { thresholds: { [METRIC_SAMPLE_RETRY_DURATION]: ["avg==15"] } };
export default function () {
  duration.record(10);
  retry.record(20);
}

export function handleSummary(data: unknown) {
  return { [__ENV.SUMMARY_PATH]: JSON.stringify(data) };
}
