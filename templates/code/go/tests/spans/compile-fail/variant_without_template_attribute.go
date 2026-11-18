package invalidconsumer

import (
	"context"

	"example.com/semconv/myappattr"
	"example.com/semconv/myappspan"
	"example.com/semconv/semconvtracer"
)

func invalid() {
	var tracer semconvtracer.Tracer
	myappspan.StartTaskRunByTaskID(context.Background(), tracer, myappattr.TaskStateQueued)
}
