package invalidconsumer

import (
	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
)

func invalid() {
	set := myappmetric.NewTaskDurationSet(myappattr.NewTaskID("t-1"))
	_ = set.record
}
