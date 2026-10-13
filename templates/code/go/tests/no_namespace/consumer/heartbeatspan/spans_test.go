package heartbeatspan_test

import (
	"context"
	"testing"

	"go.opentelemetry.io/otel/attribute"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	"go.opentelemetry.io/otel/sdk/trace/tracetest"

	"example.com/semconv/heartbeatattr"
	"example.com/semconv/heartbeatspan"
	"example.com/semconv/semconvtracer"
)

func TestReadmeHeartbeatExampleCompilesAndRecords(t *testing.T) {
	sr := tracetest.NewSpanRecorder()
	tp := sdktrace.NewTracerProvider(sdktrace.WithSpanProcessor(sr))
	tracer := semconvtracer.New(tp, "test")

	ctx, span := heartbeatspan.Start(context.Background(), tracer, heartbeatspan.Name{}, heartbeatattr.NewSequence(1))
	_ = ctx
	span.End()

	ended := sr.Ended()
	if len(ended) != 1 {
		t.Fatalf("len(Ended()) = %d, want 1", len(ended))
	}
	got := ended[0]
	if got.Name() != "heartbeat" {
		t.Fatalf("Name() = %q, want %q", got.Name(), "heartbeat")
	}
	want := attribute.NewSet(heartbeatattr.NewSequence(1).KeyValue())
	gotAttrs := attribute.NewSet(got.Attributes()...)
	if !gotAttrs.Equals(&want) {
		t.Fatalf("Attributes() = %v, want %v", gotAttrs, want)
	}
}
