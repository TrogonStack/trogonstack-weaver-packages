package invalidconsumer

import (
	"context"

	"example.com/semconv/myappspan"
	"example.com/semconv/semconvtracer"
)

func invalid() {
	var tracer semconvtracer.Tracer
	myappspan.StartTaskRun(context.Background(), tracer, myappspan.TaskRunName{}, "running")
}
