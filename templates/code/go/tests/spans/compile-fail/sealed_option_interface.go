package invalidconsumer

import (
	"go.opentelemetry.io/otel/attribute"

	"example.com/semconv/myappspan"
)

type fakeAttr struct{}

func (fakeAttr) taskRunStartAttr() attribute.KeyValue { return attribute.KeyValue{} }

func (fakeAttr) taskRunByStateStartAttr() {}

var invalid myappspan.TaskRunByStateStartAttr = fakeAttr{}
