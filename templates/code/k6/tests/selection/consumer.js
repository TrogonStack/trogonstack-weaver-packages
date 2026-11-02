import { createSampleKeep } from "./metrics.js";
const kept = createSampleKeep();
export default function () {
  kept.add(1, { "sample.state": "ready" });
}

export function handleSummary(data) {
  return { [__ENV.SUMMARY_PATH]: JSON.stringify(data) };
}
