import { SCHEMA_URL } from "./schema.ts";
export default function () {
  if (SCHEMA_URL !== "https://example.com/schemas/synthetic/1.0.0") throw new Error("Schema metadata changed");
}

export function handleSummary(data: unknown) {
  return { [__ENV.SUMMARY_PATH]: JSON.stringify(data) };
}
