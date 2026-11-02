import { createSampleKeep } from "./metrics.js";
// @ts-expect-error Deprecated metrics are excluded.
import { createSampleDeprecated } from "./metrics.js";
// @ts-expect-error Development metrics are excluded.
import { createSampleDevelopment } from "./metrics.js";
// @ts-expect-error Code-generation exclusions apply.
import { createSampleExcluded } from "./metrics.js";
createSampleKeep().add(1, { "sample.state": "ready" });
