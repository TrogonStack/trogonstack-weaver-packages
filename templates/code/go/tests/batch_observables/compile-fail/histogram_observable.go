package invalidconsumer

import (
	"example.com/semconv/myappmetric"
	"example.com/semconv/semconvmeter"
)

func invalid() {
	_, _ = myappmetric.NewTaskDurationBatchObservable(semconvmeter.Meter{})
}
