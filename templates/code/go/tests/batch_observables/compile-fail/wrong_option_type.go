package invalidconsumer

import (
	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
	"example.com/semconv/semconvmeter"
)

func invalid(observer semconvmeter.Observer) {
	depth, _ := myappmetric.NewQueueDepthBatchObservable(semconvmeter.Meter{})
	depth.Observe(observer, int64(1), myappattr.NewQueueName("main"),
		myappmetric.WithTasksCompletedTaskState(myappattr.TaskStateQueued))
}
