package invalidconsumer

import (
	"context"

	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
)

func invalid() {
	var m myappmetric.TaskDuration
	m.Record(context.Background(), 0.1, myappattr.NewTaskID("t-1"), "oops")
}
