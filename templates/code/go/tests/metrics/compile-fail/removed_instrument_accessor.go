package invalidconsumer

import (
	"example.com/semconv/myappmetric"
)

func invalid() {
	var m myappmetric.TaskDuration
	_ = m.Inst()
}
