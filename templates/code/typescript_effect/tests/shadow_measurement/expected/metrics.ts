// Generated from a semantic convention registry. Do not edit.
import * as Metric from "effect/Metric";
import * as Duration from "effect/Duration";

const measurement = Metric.mapInput(
  Metric.histogram("measurement", {
    description: "Synthetic duration.",
    attributes: { unit: "ns" },
    boundaries: [5000000, 10000000, 25000000, 50000000, 75000000, 100000000, 250000000, 500000000, 750000000, 1000000000, 2500000000, 5000000000, 7500000000, 10000000000, Infinity],
  }),
  function (value: Duration.Duration) {
    const measurement =
      value.value._tag === "Nanos"
        ? Number(value.value.nanos)
        : Duration.toMillis(value) * 1_000_000;
    return measurement;
  },
);

export function updateMeasurement(value: Duration.Duration) {
  return Metric.update(measurement, value);
}
