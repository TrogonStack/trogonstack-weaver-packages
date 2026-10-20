package invalidconsumer

import (
	"example.com/semconv/myappspan"
)

func invalid() {
	var s myappspan.TaskRun
	_ = s.Span()
}
