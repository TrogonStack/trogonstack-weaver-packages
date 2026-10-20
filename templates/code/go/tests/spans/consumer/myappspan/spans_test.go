package myappspan_test

import (
	"context"
	"errors"
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
	span.AddEvent("event", myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-1")))
	span.AddLink(trace.SpanContext{}, myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-1")))
	span.RecordError(errors.New("boom"))
	span.SetName(myappspan.NewTaskRunName("renamed"))
	if span.IsRecording() {
		t.Fatal("IsRecording() of the zero TaskRun is true")
	}
	if span.SpanContext().IsValid() {
		t.Fatal("SpanContext() of the zero TaskRun is valid")
	}
}

func TestAddEventRecordsTypedAttributes(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartTaskRun(context.Background(), tracer, myappspan.NewTaskRunName("run t-4"), myappattr.TaskStateQueued)
	span.AddEvent("retry", myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-4")))
	span.End()

	events := sr.Ended()[0].Events()
	if len(events) != 1 {
		t.Fatalf("len(Events()) = %d, want 1", len(events))
	}
	if events[0].Name != "retry" {
		t.Fatalf("Name = %q, want %q", events[0].Name, "retry")
	}
	want := attribute.NewSet(myappattr.NewTaskID("t-4").KeyValue())
	got := attribute.NewSet(events[0].Attributes...)
	if !got.Equals(&want) {
		t.Fatalf("Attributes = %v, want %v", got, want)
	}
}

func TestRecordErrorRecordsTypedAttributes(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartTaskRun(context.Background(), tracer, myappspan.NewTaskRunName("run t-5"), myappattr.TaskStateQueued)
	span.RecordError(errors.New("boom"), myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-5")))
	span.End()

	events := sr.Ended()[0].Events()
	if len(events) != 1 {
		t.Fatalf("len(Events()) = %d, want 1", len(events))
	}
	if events[0].Name != "exception" {
		t.Fatalf("Name = %q, want %q", events[0].Name, "exception")
	}
	got := attribute.NewSet(events[0].Attributes...)
	for _, kv := range []attribute.KeyValue{
		attribute.String("exception.message", "boom"),
		myappattr.NewTaskID("t-5").KeyValue(),
	} {
		if v, ok := got.Value(kv.Key); !ok || v != kv.Value {
			t.Fatalf("Attributes = %v, want %v among them", got, kv)
		}
	}
}

func TestAddLinkRecordsTypedAttributes(t *testing.T) {
	tracer, sr := newTracer(t)

	_, linked := myappspan.StartTaskRun(context.Background(), tracer, myappspan.NewTaskRunName("linked"), myappattr.TaskStateQueued)
	_, span := myappspan.StartTaskRun(context.Background(), tracer, myappspan.NewTaskRunName("run t-6"), myappattr.TaskStateQueued)
	span.AddLink(linked.SpanContext(), myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-6")))
	span.End()
	linked.End()

	var links []sdktrace.Link
	for _, ended := range sr.Ended() {
		if ended.Name() == "run t-6" {
			links = ended.Links()
		}
	}
	if len(links) != 1 {
		t.Fatalf("len(Links()) = %d, want 1", len(links))
	}
	if links[0].SpanContext.SpanID() != linked.SpanContext().SpanID() {
		t.Fatalf("link span ID = %v, want %v", links[0].SpanContext.SpanID(), linked.SpanContext().SpanID())
	}
	want := attribute.NewSet(myappattr.NewTaskID("t-6").KeyValue())
	got := attribute.NewSet(links[0].Attributes...)
	if !got.Equals(&want) {
		t.Fatalf("Attributes = %v, want %v", got, want)
	}
}

func TestSpanContextAndIsRecordingReflectTheStartedSpan(t *testing.T) {
	tracer, _ := newTracer(t)

	ctx, span := myappspan.StartTaskRun(context.Background(), tracer, myappspan.NewTaskRunName("run t-7"), myappattr.TaskStateQueued)
	if !span.IsRecording() {
		t.Fatal("IsRecording() = false, want true before End")
	}
	if got, want := span.SpanContext(), trace.SpanContextFromContext(ctx); !got.Equal(want) {
		t.Fatalf("SpanContext() = %v, want %v", got, want)
	}
	span.End()
	if span.IsRecording() {
		t.Fatal("IsRecording() = true, want false after End")
	}
}

func TestSetNameRenamesTheSpan(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartTaskRun(context.Background(), tracer, myappspan.NewTaskRunName("run t-8"), myappattr.TaskStateQueued)
	span.SetName(myappspan.NewTaskRunName("renamed"))
	span.End()

	if got := sr.Ended()[0].Name(); got != "renamed" {
		t.Fatalf("Name() = %q, want %q", got, "renamed")
	}
}

func TestSpanWithoutLateAttributesAddsEventAndError(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartQueueDrain(context.Background(), tracer, myappspan.QueueDrainName{})
	span.AddEvent("drained")
	span.RecordError(errors.New("boom"))
	span.End()

	events := sr.Ended()[0].Events()
	if len(events) != 2 || events[0].Name != "drained" || events[1].Name != "exception" {
		t.Fatalf("Events() = %v, want drained then exception", events)
	}
}
