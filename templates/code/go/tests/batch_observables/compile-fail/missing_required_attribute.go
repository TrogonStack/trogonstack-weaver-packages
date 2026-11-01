package invalidconsumer

import (
	"example.com/semconv/myappmetric"
	"example.com/semconv/semconvmeter"
)

func invalid(observer semconvmeter.Observer) {
	depth, _ := myappmetric.NewQueueDepthBatchObservable(semconvmeter.Meter{})
	depth.Observe(observer, int64(1))
}
