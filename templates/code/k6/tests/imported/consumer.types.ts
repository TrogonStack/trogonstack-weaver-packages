import { createSampleImported } from "./metrics.ts";
createSampleImported().add(1, { "remote.state": "ready", "remote.count": 1 });
// @ts-expect-error Imported enum filtering applies to the tag type.
createSampleImported().add(1, { "remote.state": "retired" });
// @ts-expect-error Imported required tags remain required.
createSampleImported().add(1, {});
