import {
  MeterProvider,
  MetricReader,
  type MetricData,
  type ScopeMetrics,
} from "@opentelemetry/sdk-metrics";

class InMemoryReader extends MetricReader {
  protected onShutdown(): Promise<void> {
    return Promise.resolve();
  }

  protected onForceFlush(): Promise<void> {
    return Promise.resolve();
  }
}

export type Recorder = {
  readonly meterProvider: MeterProvider;
  scopes(): Promise<readonly ScopeMetrics[]>;
  metric(name: string): Promise<MetricData>;
};

export function createRecorder(): Recorder {
  const reader = new InMemoryReader();
  const meterProvider = new MeterProvider({ readers: [reader] });
  const scopes = async () => {
    const { resourceMetrics } = await reader.collect();
    return resourceMetrics.scopeMetrics;
  };
  return {
    meterProvider,
    scopes,
    async metric(name) {
      for (const scope of await scopes()) {
        const found = scope.metrics.find((metric) => metric.descriptor.name === name);
        if (found !== undefined) {
          return found;
        }
      }
      throw new Error(`metric ${name} was not recorded`);
    },
  };
}
