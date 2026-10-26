// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";

export const SampleEnabled = {
  No: false,
  Yes: true,
} as const;

export type SampleEnabled = (typeof SampleEnabled)[keyof typeof SampleEnabled];

export const SampleNumericCode = {
  One: 1,
  Two: 2,
} as const;

export type SampleNumericCode = (typeof SampleNumericCode)[keyof typeof SampleNumericCode];

export const SampleQuoteState = {
  Backslash: "a\\b",
  DoubleQuote: "a\"b",
  Newline: "a\nb",
} as const;

export type SampleQuoteState = (typeof SampleQuoteState)[keyof typeof SampleQuoteState];

export type SampleRecordedAttributes = {
  readonly remoteTaskState: string;
  readonly class: string;
  readonly enabled: SampleEnabled;
  readonly numericCode: SampleNumericCode;
  readonly otherName: string;
  readonly quoteState: SampleQuoteState;
  readonly taskId: string;
};

const sampleRecorded = Metric.counter("sample.recorded", {
  description: "Synthetic metric.",
  attributes: { unit: "{item}" },
  incremental: true,
});

export function updateSampleRecorded(
  value: number,
  attributes: SampleRecordedAttributes,
) {
  return Metric.update(
    Metric.withAttributes(sampleRecorded, {
      ["remote.task.state"]: String(attributes.remoteTaskState),
      ["sample.class"]: String(attributes.class),
      ["sample.enabled"]: String(attributes.enabled),
      ["sample.numeric_code"]: String(attributes.numericCode),
      ["sample.other_name"]: String(attributes.otherName),
      ["sample.quote_state"]: String(attributes.quoteState),
      ["sample.task.id"]: String(attributes.taskId),
    }),
    value,
  );
}
