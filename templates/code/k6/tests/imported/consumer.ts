import { createSampleImported } from "./metrics.ts";
const imported = createSampleImported();
export default function () {
  imported.add(3, { "remote.state": "ready", "remote.count": 2 });
}

export function handleSummary(data: unknown) {
  return { [__ENV.SUMMARY_PATH]: JSON.stringify(data) };
}
