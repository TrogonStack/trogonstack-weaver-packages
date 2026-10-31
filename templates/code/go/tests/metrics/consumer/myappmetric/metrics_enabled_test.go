package myappmetric_test

import (
	"context"
	"testing"

	sdkmetric "go.opentelemetry.io/otel/sdk/metric"

	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
	"example.com/semconv/semconvmeter"
)

func dropView(name string) sdkmetric.View {
	return sdkmetric.NewView(
		sdkmetric.Instrument{Name: name},
		sdkmetric.Stream{Aggregation: sdkmetric.AggregationDrop{}},
	)
}

func TestDisabledInstrumentRecordsNothing(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	mp := sdkmetric.NewMeterProvider(
		sdkmetric.WithReader(reader),
		sdkmetric.WithView(dropView(myappmetric.TaskDuration{}.Name())),
	)
	meter := semconvmeter.New(mp, "test")

	taskDuration, err := myappmetric.NewTaskDuration(meter)
	if err != nil {
		t.Fatalf("NewTaskDuration() error = %v", err)
	}

	taskDuration.Record(context.Background(), 0.25, myappattr.NewTaskID("t-1"))

	if m, ok := findMetric(collect(t, reader), "myapp.task.duration"); ok {
		t.Fatalf("the dropped TaskDuration recorded %v, want nothing", m.Data)
	}
}

func TestDisabledInstrumentSkipsBuildingAttributes(t *testing.T) {
	ctx := context.Background()
	add := func(taskActive myappmetric.TaskActive) float64 {
		return testing.AllocsPerRun(100, func() {
			taskActive.Add(ctx, 1, myappattr.TaskStateDone)
		})
	}

	enabledReader := sdkmetric.NewManualReader()
	enabled, err := myappmetric.NewTaskActive(semconvmeter.New(
		sdkmetric.NewMeterProvider(sdkmetric.WithReader(enabledReader)), "test"))
	if err != nil {
		t.Fatalf("NewTaskActive() error = %v", err)
	}
	if allocs := add(enabled); allocs == 0 {
		t.Fatal("Add on an enabled TaskActive allocated nothing, so this test cannot tell the two paths apart")
	}

	disabledReader := sdkmetric.NewManualReader()
	disabled, err := myappmetric.NewTaskActive(semconvmeter.New(
		sdkmetric.NewMeterProvider(
			sdkmetric.WithReader(disabledReader),
			sdkmetric.WithView(dropView(myappmetric.TaskActive{}.Name())),
		), "test"))
	if err != nil {
		t.Fatalf("NewTaskActive() error = %v", err)
	}
	if allocs := add(disabled); allocs != 0 {
		t.Fatalf("Add on a dropped TaskActive allocated %v times per call, want 0", allocs)
	}
}
