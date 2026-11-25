package invalidconsumer

import (
	"context"

	"example.com/semconv/myappspan"
	"example.com/semconv/semconvtracer"
)

func invalid() {
	var tracer semconvtracer.Tracer
	myappspan.StartTaskRunByState(context.Background(), tracer, "running")
}
