import { createSampleKeep } from "./metrics.ts";
// @ts-expect-error Deprecated metrics are excluded.
import { createSampleDeprecated } from "./metrics.ts";
// @ts-expect-error Development metrics are excluded.
import { createSampleDevelopment } from "./metrics.ts";
// @ts-expect-error Code-generation exclusions apply.
import { createSampleExcluded } from "./metrics.ts";
createSampleKeep().add(1, { "sample.state": "ready" });
