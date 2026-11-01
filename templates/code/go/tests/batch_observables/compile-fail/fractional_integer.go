package invalidconsumer

import (
	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
	"example.com/semconv/semconvmeter"
)

func invalid(observer semconvmeter.Observer) {
	depth, _ := myappmetric.NewQueueDepthBatchObservable(semconvmeter.Meter{})
	depth.Observe(observer, 0.5, myappattr.NewQueueName("main"))
}
