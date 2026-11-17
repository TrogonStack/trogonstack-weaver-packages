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
	"example.com/semconv/workerattr"
)

func newTracer(t *testing.T) (semconvtracer.Tracer, *tracetest.SpanRecorder) {
	t.Helper()
	sr := tracetest.NewSpanRecorder()
	tp := sdktrace.NewTracerProvider(sdktrace.WithSpanProcessor(sr))
	return semconvtracer.New(tp, "test"), sr
}

func TestStartTaskRunByTaskIDRecordsNameKindAndAttributes(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartTaskRunByTaskID(
		context.Background(),
		tracer,
		myappattr.NewTaskID("t-1"),
		myappattr.TaskStateDone,
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

func TestStartTaskRunByStateDropsZeroValueRequiredAttribute(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartTaskRunByState(
		context.Background(),
		tracer,
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

	_, span := myappspan.StartTaskRunByState(
		context.Background(),
		tracer,
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
	if span.IsRecording() {
		t.Fatal("IsRecording() of the zero TaskRun is true")
	}
	if span.SpanContext().IsValid() {
		t.Fatal("SpanContext() of the zero TaskRun is valid")
	}
}

func TestAddEventRecordsTypedAttributes(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartTaskRunByState(context.Background(), tracer, myappattr.TaskStateQueued)
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

	_, span := myappspan.StartTaskRunByState(context.Background(), tracer, myappattr.TaskStateQueued)
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

	_, linked := myappspan.StartTaskRunByState(context.Background(), tracer, myappattr.TaskStateQueued)
	_, span := myappspan.StartTaskRunByTaskID(context.Background(), tracer, myappattr.NewTaskID("t-6"), myappattr.TaskStateQueued)
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

	ctx, span := myappspan.StartTaskRunByState(context.Background(), tracer, myappattr.TaskStateQueued)
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

	_, span := myappspan.StartQueueDrain(context.Background(), tracer, myappspan.NewQueueDrainName("drain reports"))
	span.SetName(myappspan.NewQueueDrainName("renamed"))
	span.End()

	if got := sr.Ended()[0].Name(); got != "renamed" {
		t.Fatalf("Name() = %q, want %q", got, "renamed")
	}
}

func TestEmptySpanNameFallsBackToTheSpanType(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartQueueDrain(context.Background(), tracer, myappspan.QueueDrainName{})
	span.End()

	if got := sr.Ended()[0].Name(); got != "myapp.queue.drain" {
		t.Fatalf("Name() = %q, want %q", got, "myapp.queue.drain")
	}
}

func TestStartTaskRunByStateNamesTheSpanFromTheStateTemplateFirst(t *testing.T) {
	tests := []struct {
		name  string
		state myappattr.TaskState
		opts  []myappspan.TaskRunStartAttr
		want  string
	}{
		{"state set", myappattr.TaskStateDone, []myappspan.TaskRunStartAttr{myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-9"))}, "run done"},
		{"other enum member falls back to the task ID", myappattr.TaskStateOther, []myappspan.TaskRunStartAttr{myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-9"))}, "run t-9"},
		{"zero enum falls back to the task ID", myappattr.TaskState{}, []myappspan.TaskRunStartAttr{myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-9"))}, "run t-9"},
		{"last optional attribute wins", myappattr.TaskStateOther, []myappspan.TaskRunStartAttr{myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-9")), myappspan.WithTaskRunTaskID(myappattr.NewTaskID("t-10"))}, "run t-10"},
		{"other enum member and empty task ID", myappattr.TaskStateOther, []myappspan.TaskRunStartAttr{myappspan.WithTaskRunTaskID(myappattr.NewTaskID(""))}, "myapp.task.run"},
		{"other enum member and no task ID", myappattr.TaskStateOther, nil, "myapp.task.run"},
		{"zero enum and no task ID", myappattr.TaskState{}, nil, "myapp.task.run"},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			tracer, sr := newTracer(t)

			_, span := myappspan.StartTaskRunByState(context.Background(), tracer, test.state, test.opts...)
			span.End()

			if got := sr.Ended()[0].Name(); got != test.want {
				t.Fatalf("Name() = %q, want %q", got, test.want)
			}
		})
	}
}

func TestStartTaskRunByTaskIDNamesTheSpanFromTheTaskIDTemplateFirst(t *testing.T) {
	tests := []struct {
		name   string
		taskID string
		state  myappattr.TaskState
		want   string
	}{
		{"task ID set", "t-9", myappattr.TaskStateDone, "run t-9"},
		{"empty task ID falls back to the state", "", myappattr.TaskStateQueued, "run queued"},
		{"empty task ID and other enum member", "", myappattr.TaskStateOther, "myapp.task.run"},
		{"empty task ID and zero enum", "", myappattr.TaskState{}, "myapp.task.run"},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			tracer, sr := newTracer(t)

			_, span := myappspan.StartTaskRunByTaskID(context.Background(), tracer, myappattr.NewTaskID(test.taskID), test.state)
			span.End()

			if got := sr.Ended()[0].Name(); got != test.want {
				t.Fatalf("Name() = %q, want %q", got, test.want)
			}
		})
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

func TestStartTaskDispatchByWorkerFallsBackToTheLiteralNameTemplate(t *testing.T) {
	tests := []struct {
		name     string
		hostName string
		want     string
	}{
		{"host name set", "worker-1", "dispatch worker-1:8080"},
		{"empty host name", "", "dispatch"},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			tracer, sr := newTracer(t)

			_, span := myappspan.StartTaskDispatchByWorker(context.Background(), tracer, workerattr.NewHostName(test.hostName), workerattr.NewHostPort(8080))
			span.End()

			if got := sr.Ended()[0].Name(); got != test.want {
				t.Fatalf("Name() = %q, want %q", got, test.want)
			}
		})
	}
}

func TestStartTaskDispatchFallbackNamesTheSpanFromTheLiteralTemplate(t *testing.T) {
	tracer, sr := newTracer(t)

	_, span := myappspan.StartTaskDispatchFallback(context.Background(), tracer, workerattr.NewHostName("worker-1"), workerattr.NewHostPort(8080))
	span.End()

	got := sr.Ended()[0]
	if got.Name() != "dispatch" {
		t.Fatalf("Name() = %q, want %q", got.Name(), "dispatch")
	}
	want := attribute.NewSet(workerattr.NewHostName("worker-1").KeyValue(), workerattr.NewHostPort(8080).KeyValue())
	gotSet := attribute.NewSet(got.Attributes()...)
	if !gotSet.Equals(&want) {
		t.Fatalf("Attributes() = %v, want %v", gotSet, want)
	}
}
