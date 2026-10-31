package invalidconsumer

import (
	"context"

	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
)

func invalid() {
	var m myappmetric.TaskDuration
	set := myappmetric.NewTaskRetryDurationSet(myappattr.NewTaskID("t-1"), myappattr.TaskStateDone)
	m.RecordSet(context.Background(), 0.1, set)
}
