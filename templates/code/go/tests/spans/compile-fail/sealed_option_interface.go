package invalidconsumer

import (
	"go.opentelemetry.io/otel/attribute"

	"example.com/semconv/myappspan"
)

type fakeAttr struct{}

func (fakeAttr) taskRunStartAttr() attribute.KeyValue { return attribute.KeyValue{} }

var invalid myappspan.TaskRunStartAttr = fakeAttr{}
