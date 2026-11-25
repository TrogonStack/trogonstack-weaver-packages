package myappspan_test

import (
	"context"
	"math"
	"strings"
	"testing"

	"go.opentelemetry.io/otel/trace/noop"

	"example.com/semconv/myappattr"
	"example.com/semconv/myappspan"
	"example.com/semconv/semconvtracer"
	"example.com/semconv/workerattr"
)

func startAllocations(start func(context.Context, semconvtracer.Tracer)) float64 {
	ctx := context.Background()
	tracer := semconvtracer.New(noop.NewTracerProvider(), "test")
	return testing.AllocsPerRun(100, func() { start(ctx, tracer) })
}

func TestStarterAllocationsWithANonRecordingTracer(t *testing.T) {
	longTaskID := myappattr.NewTaskID(strings.Repeat("task", 1000))
	for _, test := range []struct {
		name  string
		start func(context.Context, semconvtracer.Tracer)
		want  float64
	}{
		{
			name: "3 for the tracer, 2 for the attributes, 1 for a name from a string attribute",
			start: func(ctx context.Context, tracer semconvtracer.Tracer) {
				myappspan.StartTaskRunByTaskID(ctx, tracer, myappattr.NewTaskID("t-1"), myappattr.TaskStateDone)
			},
			want: 6,
		},
		{
			name: "3 for the tracer, 2 for the attributes, 1 for a 4004 byte name from a string attribute",
			start: func(ctx context.Context, tracer semconvtracer.Tracer) {
				myappspan.StartTaskRunByTaskID(ctx, tracer, longTaskID, myappattr.TaskStateDone)
			},
			want: 6,
		},
		{
			name: "3 for the tracer, 2 for the attributes, 1 for a name from an int attribute",
			start: func(ctx context.Context, tracer semconvtracer.Tracer) {
				myappspan.StartNameAttempt(ctx, tracer, myappattr.NewTaskAttempt(math.MinInt64))
			},
			want: 6,
		},
		{
			name: "3 for the tracer, 2 for the attributes, 1 for a name from a float attribute",
			start: func(ctx context.Context, tracer semconvtracer.Tracer) {
				myappspan.StartNameFloat(ctx, tracer, myappattr.NewNumberValue(-math.MaxFloat64))
			},
			want: 6,
		},
		{
			name: "3 for the tracer, 3 for the attributes and the array, 2 for copies of the array, 1 for a name from an array attribute",
			start: func(ctx context.Context, tracer semconvtracer.Tracer) {
				myappspan.StartNameIndices(ctx, tracer, myappattr.NewArrayIndices(math.MinInt64, 2))
			},
			want: 9,
		},
		{
			name: "3 for the tracer, 2 for the attributes, 0 for a literal fallback name",
			start: func(ctx context.Context, tracer semconvtracer.Tracer) {
				myappspan.StartTaskDispatchByWorker(ctx, tracer, workerattr.NewHostName(""), workerattr.NewHostPort(8080))
			},
			want: 5,
		},
		{
			name: "3 for the tracer, 0 for the caller name of an untemplated span",
			start: func(ctx context.Context, tracer semconvtracer.Tracer) {
				myappspan.StartQueueDrain(ctx, tracer, myappspan.NewQueueDrainName("drain reports"))
			},
			want: 3,
		},
	} {
		t.Run(test.name, func(t *testing.T) {
			if got := startAllocations(test.start); got != test.want {
				t.Fatalf("allocations per start = %v, want %v", got, test.want)
			}
		})
	}
}
