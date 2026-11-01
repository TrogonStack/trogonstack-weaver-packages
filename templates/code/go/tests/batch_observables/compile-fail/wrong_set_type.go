package invalidconsumer

import (
	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
	"example.com/semconv/semconvmeter"
	"example.com/semconv/workermetric"
)

func invalid(observer semconvmeter.Observer) {
	depth, _ := myappmetric.NewQueueDepthBatchObservable(semconvmeter.Meter{})
	depth.ObserveSet(observer, int64(1), workermetric.NewQueueRatioSet(myappattr.NewQueueName("main")))
}
