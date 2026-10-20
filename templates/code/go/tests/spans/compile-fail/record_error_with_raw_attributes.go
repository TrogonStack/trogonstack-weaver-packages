package invalidconsumer

import (
	"errors"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/trace"

	"example.com/semconv/myappspan"
)

func invalid() {
	var s myappspan.TaskRun
	s.RecordError(errors.New("boom"), trace.WithAttributes(attribute.String("k", "v")))
}
