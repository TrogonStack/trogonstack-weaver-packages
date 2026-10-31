package myappmetric_test

import (
	"context"
	"testing"

	"go.opentelemetry.io/otel/attribute"
	sdkmetric "go.opentelemetry.io/otel/sdk/metric"
	"go.opentelemetry.io/otel/sdk/metric/metricdata"

	"example.com/semconv/authattr"
	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
	"example.com/semconv/semconvmeter"
)

func TestTaskDurationRecordSetRecordsTheSameAttributesAsRecord(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	mp := sdkmetric.NewMeterProvider(sdkmetric.WithReader(reader))
	meter := semconvmeter.New(mp, "test")

	taskDuration, err := myappmetric.NewTaskDuration(meter)
	if err != nil {
		t.Fatalf("NewTaskDuration() error = %v", err)
	}

	taskDuration.Record(
		context.Background(),
		0.25,
		myappattr.NewTaskID("t-1"),
		myappmetric.WithTaskDurationMethod(authattr.MethodToken),
		myappmetric.WithTaskDurationTaskState(myappattr.TaskState{}),
	)
	set := myappmetric.NewTaskDurationSet(
		myappattr.NewTaskID("t-1"),
		myappmetric.WithTaskDurationMethod(authattr.MethodToken),
		myappmetric.WithTaskDurationTaskState(myappattr.TaskState{}),
	)
	taskDuration.RecordSet(context.Background(), 0.75, set)

	m, ok := findMetric(collect(t, reader), "myapp.task.duration")
	if !ok {
		t.Fatal("myapp.task.duration was not recorded")
	}
	histogram, ok := m.Data.(metricdata.Histogram[float64])
	if !ok {
		t.Fatalf("Data = %T, want metricdata.Histogram[float64]", m.Data)
	}
	if len(histogram.DataPoints) != 1 {
		t.Fatalf("len(DataPoints) = %d, want 1, both paths must land in one series", len(histogram.DataPoints))
	}
	dp := histogram.DataPoints[0]
	if dp.Count != 2 {
		t.Fatalf("Count = %d, want 2", dp.Count)
	}
	want := attribute.NewSet(myappattr.NewTaskID("t-1").KeyValue(), authattr.MethodToken.KeyValue())
	if !dp.Attributes.Equals(&want) {
		t.Fatalf("Attributes = %v, want %v", dp.Attributes, want)
	}
}

func TestTaskActiveAddSetRecordsTheSameAttributesAsAdd(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	mp := sdkmetric.NewMeterProvider(sdkmetric.WithReader(reader))
	meter := semconvmeter.New(mp, "test")

	taskActive, err := myappmetric.NewTaskActive(meter)
	if err != nil {
		t.Fatalf("NewTaskActive() error = %v", err)
	}

	taskActive.Add(context.Background(), 1, myappattr.TaskStateDone)
	taskActive.AddSet(context.Background(), 2, myappmetric.NewTaskActiveSet(myappattr.TaskStateDone))

	m, ok := findMetric(collect(t, reader), "myapp.task.active")
	if !ok {
		t.Fatal("myapp.task.active was not recorded")
	}
	sum := m.Data.(metricdata.Sum[int64])
	if len(sum.DataPoints) != 1 {
		t.Fatalf("len(DataPoints) = %d, want 1, both paths must land in one series", len(sum.DataPoints))
	}
	dp := sum.DataPoints[0]
	if dp.Value != 3 {
		t.Fatalf("Value = %d, want 3", dp.Value)
	}
	want := attribute.NewSet(myappattr.TaskStateDone.KeyValue())
	if !dp.Attributes.Equals(&want) {
		t.Fatalf("Attributes = %v, want %v", dp.Attributes, want)
	}
}

func TestZeroSetRecordsNoAttributes(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	mp := sdkmetric.NewMeterProvider(sdkmetric.WithReader(reader))
	meter := semconvmeter.New(mp, "test")

	taskActive, err := myappmetric.NewTaskActive(meter)
	if err != nil {
		t.Fatalf("NewTaskActive() error = %v", err)
	}

	var set myappmetric.TaskActiveSet
	taskActive.AddSet(context.Background(), 1, set)

	m, ok := findMetric(collect(t, reader), "myapp.task.active")
	if !ok {
		t.Fatal("myapp.task.active was not recorded")
	}
	sum := m.Data.(metricdata.Sum[int64])
	if n := sum.DataPoints[0].Attributes.Len(); n != 0 {
		t.Fatalf("Attributes.Len() = %d, want 0", n)
	}
}

func TestObserveSetRecordsTheSameAttributesAsObserve(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	mp := sdkmetric.NewMeterProvider(sdkmetric.WithReader(reader))
	meter := semconvmeter.New(mp, "test")

	set := myappmetric.NewTaskActiveSet(myappattr.TaskStateQueued)
	_, err := myappmetric.NewTaskActiveObservable(meter,
		func(ctx context.Context, o myappmetric.TaskActiveObserver) error {
			o.Observe(4, myappattr.TaskStateDone)
			o.ObserveSet(7, set)
			return nil
		})
	if err != nil {
		t.Fatalf("NewTaskActiveObservable() error = %v", err)
	}

	m, ok := findMetric(collect(t, reader), "myapp.task.active")
	if !ok {
		t.Fatal("myapp.task.active was not observed")
	}
	sum := m.Data.(metricdata.Sum[int64])
	if len(sum.DataPoints) != 2 {
		t.Fatalf("len(DataPoints) = %d, want 2", len(sum.DataPoints))
	}
	got := map[string]int64{}
	for _, dp := range sum.DataPoints {
		state, _ := dp.Attributes.Value(attribute.Key("myapp.task.state"))
		got[state.AsString()] = dp.Value
	}
	if got["done"] != 4 || got["queued"] != 7 {
		t.Fatalf("observed %v, want done=4 queued=7", got)
	}
}
