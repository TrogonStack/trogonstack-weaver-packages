import { SCHEMA_URL } from "./schema.js";
export default function () {
  if (SCHEMA_URL !== "https://example.com/schemas/synthetic/1.0.0") throw new Error("Schema metadata changed");
}

export function handleSummary(data) {
  return { [__ENV.SUMMARY_PATH]: JSON.stringify(data) };
}
