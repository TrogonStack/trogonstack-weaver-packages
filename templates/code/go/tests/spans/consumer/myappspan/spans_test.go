package myappspan_test

import (
	"context"
	"testing"

	"go.opentelemetry.io/otel/attribute"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	"go.opentelemetry.io/otel/sdk/trace/tracetest"
	"go.opentelemetry.io/otel/trace"

	"example.com/semconv/myappattr"
	"example.com/semconv/myappspan"
	"example.com/semconv/semconvtracer"
)

func newTracer(t *testing.T) (semconvtracer.Tracer, *tracetest.SpanRecorder) {
	t.Helper()
	sr := tracetest.NewSpanRecorder()
	tp := sdktrace.NewTracerProvider(sdktrace.WithSpanProcessor(sr))
	return semconvtracer.New(tp, "test"), sr
}

func TestStartTaskRunRecordsNameKindAndAttributes(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartTaskRun(
		context.Background(),
		tracer,
		myappspan.NewTaskRunName("run t-1"),
		myappattr.TaskStateDone,
		myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-1")),
	)
	span.End()

	ended := sr.Ended()
	if len(ended) != 1 {
		t.Fatalf("len(Ended()) = %d, want 1", len(ended))
	}
	got := ended[0]
	if got.Name() != "run t-1" {
		t.Fatalf("Name() = %q, want %q", got.Name(), "run t-1")
	}
	if got.SpanKind() != trace.SpanKindInternal {
		t.Fatalf("SpanKind() = %v, want %v", got.SpanKind(), trace.SpanKindInternal)
	}
	want := attribute.NewSet(
		myappattr.TaskStateDone.KeyValue(),
		myappattr.NewTaskID("t-1").KeyValue(),
	)
	gotSet := attribute.NewSet(got.Attributes()...)
	if !gotSet.Equals(&want) {
		t.Fatalf("Attributes() = %v, want %v", gotSet, want)
	}
}

func TestStartTaskRunDropsZeroValueRequiredAttribute(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartTaskRun(
		context.Background(),
		tracer,
		myappspan.NewTaskRunName("run t-2"),
		myappattr.TaskState{},
	)
	span.End()

	ended := sr.Ended()
	if len(ended) != 1 {
		t.Fatalf("len(Ended()) = %d, want 1", len(ended))
	}
	if attrs := ended[0].Attributes(); len(attrs) != 0 {
		t.Fatalf("Attributes() = %v, want none, the zero TaskState must not be recorded", attrs)
	}
}

func TestSetAttributesAddsAttributeAfterStart(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartTaskRun(
		context.Background(),
		tracer,
		myappspan.NewTaskRunName("run t-3"),
		myappattr.TaskStateQueued,
	)
	span.SetAttributes(myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-3")))
	span.End()

	ended := sr.Ended()
	want := attribute.NewSet(myappattr.TaskStateQueued.KeyValue(), myappattr.NewTaskID("t-3").KeyValue())
	gotSet := attribute.NewSet(ended[0].Attributes()...)
	if !gotSet.Equals(&want) {
		t.Fatalf("Attributes() = %v, want %v", gotSet, want)
	}
}

func TestZeroSpanEndsWithoutPanic(t *testing.T) {
	var span myappspan.TaskRun
	span.End()
	span.SetAttributes(myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-1")))
	span.SetStatus(0, "")
}
