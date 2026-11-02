import { createSampleKeep } from "./metrics.ts";
const kept = createSampleKeep();
export default function () {
  kept.add(1, { "sample.state": "ready" });
}

export function handleSummary(data: unknown) {
  return { [__ENV.SUMMARY_PATH]: JSON.stringify(data) };
}
