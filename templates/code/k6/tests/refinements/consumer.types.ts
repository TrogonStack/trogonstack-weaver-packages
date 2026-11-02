import { createSampleRetryDuration, METRIC_SAMPLE_RETRY_DURATION } from "./metrics.js";
const name: "sample_duration" = METRIC_SAMPLE_RETRY_DURATION;
createSampleRetryDuration().record(20);
void name;

import type { SampleDuration } from "./metrics.js";
// @ts-expect-error A refinement has its own identity even when the native name is shared.
const wrongRefinement: SampleDuration = createSampleRetryDuration();
void wrongRefinement;
