package myappmetric_test

import (
	"context"
	"testing"

	sdkmetric "go.opentelemetry.io/otel/sdk/metric"

	"example.com/semconv/authattr"
	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
	"example.com/semconv/semconvmeter"
)

func benchmarkMeter(b *testing.B, options ...sdkmetric.Option) semconvmeter.Meter {
	b.Helper()
	options = append(options, sdkmetric.WithReader(sdkmetric.NewManualReader()))
	return semconvmeter.New(sdkmetric.NewMeterProvider(options...), "bench")
}

func BenchmarkTaskDurationRecord(b *testing.B) {
	taskDuration, err := myappmetric.NewTaskDuration(benchmarkMeter(b))
	if err != nil {
		b.Fatal(err)
	}
	ctx := context.Background()
	taskID := myappattr.NewTaskID("t-1")
	b.ReportAllocs()
	b.ResetTimer()
	for range b.N {
		taskDuration.Record(ctx, 0.25, taskID, myappmetric.WithTaskDurationMethod(authattr.MethodToken))
	}
}

func BenchmarkTaskDurationRecordDisabled(b *testing.B) {
	taskDuration, err := myappmetric.NewTaskDuration(benchmarkMeter(b,
		sdkmetric.WithView(dropView(myappmetric.TaskDuration{}.Name()))))
	if err != nil {
		b.Fatal(err)
	}
	ctx := context.Background()
	taskID := myappattr.NewTaskID("t-1")
	b.ReportAllocs()
	b.ResetTimer()
	for range b.N {
		taskDuration.Record(ctx, 0.25, taskID, myappmetric.WithTaskDurationMethod(authattr.MethodToken))
	}
}

func BenchmarkTaskActiveAdd(b *testing.B) {
	taskActive, err := myappmetric.NewTaskActive(benchmarkMeter(b))
	if err != nil {
		b.Fatal(err)
	}
	ctx := context.Background()
	b.ReportAllocs()
	b.ResetTimer()
	for range b.N {
		taskActive.Add(ctx, 1, myappattr.TaskStateDone)
	}
}

func BenchmarkTaskActiveAddDisabled(b *testing.B) {
	taskActive, err := myappmetric.NewTaskActive(benchmarkMeter(b,
		sdkmetric.WithView(dropView(myappmetric.TaskActive{}.Name()))))
	if err != nil {
		b.Fatal(err)
	}
	ctx := context.Background()
	b.ReportAllocs()
	b.ResetTimer()
	for range b.N {
		taskActive.Add(ctx, 1, myappattr.TaskStateDone)
	}
}
