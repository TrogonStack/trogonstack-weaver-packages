package invalidconsumer

import (
	"go.opentelemetry.io/otel/metric"

	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
	"example.com/semconv/semconvmeter"
)

func invalid(observer metric.Observer) {
	depth, _ := myappmetric.NewQueueDepthBatchObservable(semconvmeter.Meter{})
	depth.Observe(observer, int64(1), myappattr.NewQueueName("main"))
}
