package invalidconsumer

import (
	"context"

	"go.opentelemetry.io/otel/metric"

	"example.com/semconv/semconvmeter"
)

func invalid(meter semconvmeter.Meter, instrument metric.Int64ObservableGauge) {
	_, _ = meter.RegisterCallback(func(context.Context, semconvmeter.Observer) error {
		return nil
	}, instrument)
}
