package myappevent_test

import (
	"context"
	"sync"
	"testing"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/log"
	sdklog "go.opentelemetry.io/otel/sdk/log"

	"example.com/semconv/myappattr"
	"example.com/semconv/myappevent"
	"example.com/semconv/semconvlogger"
	"example.com/semconv/workerattr"
)

type recordingExporter struct {
	mu      sync.Mutex
	records []sdklog.Record
}

func (e *recordingExporter) Export(ctx context.Context, records []sdklog.Record) error {
	e.mu.Lock()
	defer e.mu.Unlock()
	for _, r := range records {
		e.records = append(e.records, r.Clone())
	}
	return nil
}

func (e *recordingExporter) Shutdown(context.Context) error { return nil }

func (e *recordingExporter) ForceFlush(context.Context) error { return nil }

func (e *recordingExporter) snapshot() []sdklog.Record {
	e.mu.Lock()
	defer e.mu.Unlock()
	out := make([]sdklog.Record, len(e.records))
	copy(out, e.records)
	return out
}

func newLogger(t *testing.T) (semconvlogger.Logger, *recordingExporter) {
	t.Helper()
	exporter := &recordingExporter{}
	lp := sdklog.NewLoggerProvider(sdklog.WithProcessor(sdklog.NewSimpleProcessor(exporter)))
	t.Cleanup(func() { _ = lp.Shutdown(context.Background()) })
	return semconvlogger.New(lp, "test"), exporter
}

func attributesOf(r sdklog.Record) []attribute.KeyValue {
	var out []attribute.KeyValue
	r.WalkAttributes(func(kv attribute.KeyValue) bool {
		out = append(out, kv)
		return true
	})
	return out
}

func TestEmitTaskFinishedRecordsNameSeverityAndAttributes(t *testing.T) {
	logger, exporter := newLogger(t)

	myappevent.EmitTaskFinished(
		context.Background(),
		logger,
		myappattr.TaskStateDone,
		myappevent.WithTaskFinishedTaskAttempt(myappattr.NewTaskAttempt(2)),
		myappevent.WithTaskFinishedHostName(workerattr.NewHostName("worker-1")),
	)

	records := exporter.snapshot()
	if len(records) != 1 {
		t.Fatalf("len(records) = %d, want 1", len(records))
	}
	r := records[0]
	if got, want := r.EventName(), "myapp.task.finished"; got != want {
		t.Fatalf("EventName() = %q, want %q", got, want)
	}
	if got, want := r.Severity(), log.SeverityInfo; got != want {
		t.Fatalf("Severity() = %v, want %v", got, want)
	}
	want := attribute.NewSet(
		myappattr.TaskStateDone.KeyValue(),
		myappattr.NewTaskAttempt(2).KeyValue(),
		workerattr.NewHostName("worker-1").KeyValue(),
	)
	got := attribute.NewSet(attributesOf(r)...)
	if !got.Equals(&want) {
		t.Fatalf("attributes = %v, want %v", got, want)
	}
}

func TestEmitTaskFinishedDropsZeroValueRequiredAttribute(t *testing.T) {
	logger, exporter := newLogger(t)

	myappevent.EmitTaskFinished(context.Background(), logger, myappattr.TaskState{})

	records := exporter.snapshot()
	if len(records) != 1 {
		t.Fatalf("len(records) = %d, want 1", len(records))
	}
	if attrs := attributesOf(records[0]); len(attrs) != 0 {
		t.Fatalf("attributes = %v, want none, the zero TaskState must not be recorded", attrs)
	}
}

func TestWithTaskFinishedSeverityOverridesDefault(t *testing.T) {
	logger, exporter := newLogger(t)

	myappevent.EmitTaskFinished(
		context.Background(),
		logger,
		myappattr.TaskStateDone,
		myappevent.WithTaskFinishedSeverity(log.SeverityError),
	)

	records := exporter.snapshot()
	if got, want := records[0].Severity(), log.SeverityError; got != want {
		t.Fatalf("Severity() = %v, want %v", got, want)
	}
}

func TestZeroLoggerEmitsNothing(t *testing.T) {
	var logger semconvlogger.Logger
	myappevent.EmitTaskFinished(context.Background(), logger, myappattr.TaskStateDone)
}
