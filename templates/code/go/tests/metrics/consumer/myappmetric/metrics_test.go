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

func collect(t *testing.T, reader *sdkmetric.ManualReader) metricdata.ResourceMetrics {
	t.Helper()
	var rm metricdata.ResourceMetrics
	if err := reader.Collect(context.Background(), &rm); err != nil {
		t.Fatalf("Collect() error = %v", err)
	}
	return rm
}

func findMetric(rm metricdata.ResourceMetrics, name string) (metricdata.Metrics, bool) {
	for _, sm := range rm.ScopeMetrics {
		for _, m := range sm.Metrics {
			if m.Name == name {
				return m, true
			}
		}
	}
	return metricdata.Metrics{}, false
}

func TestTaskActiveRecordsRequiredAttribute(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	mp := sdkmetric.NewMeterProvider(sdkmetric.WithReader(reader))
	meter := semconvmeter.New(mp, "test")

	taskActive, err := myappmetric.NewTaskActive(meter)
	if err != nil {
		t.Fatalf("NewTaskActive() error = %v", err)
	}
	if got, want := taskActive.Name(), "myapp.task.active"; got != want {
		t.Fatalf("Name() = %q, want %q", got, want)
	}

	taskActive.Add(context.Background(), 1, myappattr.TaskStateDone)

	m, ok := findMetric(collect(t, reader), "myapp.task.active")
	if !ok {
		t.Fatal("myapp.task.active was not recorded")
	}
	if got, want := m.Unit, "{task}"; got != want {
		t.Fatalf("Unit = %q, want %q", got, want)
	}
	if got, want := m.Description, "Number of tasks currently running."; got != want {
		t.Fatalf("Description = %q, want %q", got, want)
	}
	sum, ok := m.Data.(metricdata.Sum[int64])
	if !ok {
		t.Fatalf("Data = %T, want metricdata.Sum[int64]", m.Data)
	}
	if len(sum.DataPoints) != 1 {
		t.Fatalf("len(DataPoints) = %d, want 1", len(sum.DataPoints))
	}
	dp := sum.DataPoints[0]
	if dp.Value != 1 {
		t.Fatalf("Value = %d, want 1", dp.Value)
	}
	want := attribute.NewSet(myappattr.TaskStateDone.KeyValue())
	if !dp.Attributes.Equals(&want) {
		t.Fatalf("Attributes = %v, want %v", dp.Attributes, want)
	}
}

func TestZeroValueEnumAttributeIsDroppedNotPanicked(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	mp := sdkmetric.NewMeterProvider(sdkmetric.WithReader(reader))
	meter := semconvmeter.New(mp, "test")

	taskActive, err := myappmetric.NewTaskActive(meter)
	if err != nil {
		t.Fatalf("NewTaskActive() error = %v", err)
	}

	taskActive.Add(context.Background(), 1, myappattr.TaskState{})

	m, ok := findMetric(collect(t, reader), "myapp.task.active")
	if !ok {
		t.Fatal("myapp.task.active was not recorded")
	}
	sum := m.Data.(metricdata.Sum[int64])
	if len(sum.DataPoints) != 1 {
		t.Fatalf("len(DataPoints) = %d, want 1", len(sum.DataPoints))
	}
	if n := sum.DataPoints[0].Attributes.Len(); n != 0 {
		t.Fatalf("Attributes.Len() = %d, want 0, the zero TaskState must not be recorded", n)
	}
}

func TestTaskDurationRecordsRequiredAndOptionalAttributes(t *testing.T) {
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
	)

	m, ok := findMetric(collect(t, reader), "myapp.task.duration")
	if !ok {
		t.Fatal("myapp.task.duration was not recorded")
	}
	histogram, ok := m.Data.(metricdata.Histogram[float64])
	if !ok {
		t.Fatalf("Data = %T, want metricdata.Histogram[float64]", m.Data)
	}
	if len(histogram.DataPoints) != 1 {
		t.Fatalf("len(DataPoints) = %d, want 1", len(histogram.DataPoints))
	}
	dp := histogram.DataPoints[0]
	want := attribute.NewSet(myappattr.NewTaskID("t-1").KeyValue(), authattr.MethodToken.KeyValue())
	if !dp.Attributes.Equals(&want) {
		t.Fatalf("Attributes = %v, want %v", dp.Attributes, want)
	}
}

func TestTaskDurationDropsZeroValueOptionalAttribute(t *testing.T) {
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
		myappmetric.WithTaskDurationMethod(authattr.Method{}),
	)

	m, ok := findMetric(collect(t, reader), "myapp.task.duration")
	if !ok {
		t.Fatal("myapp.task.duration was not recorded")
	}
	histogram := m.Data.(metricdata.Histogram[float64])
	dp := histogram.DataPoints[0]
	want := attribute.NewSet(myappattr.NewTaskID("t-1").KeyValue())
	if !dp.Attributes.Equals(&want) {
		t.Fatalf("Attributes = %v, want %v, the zero Method must not be recorded", dp.Attributes, want)
	}
}

func TestZeroTaskActiveRecordsNothing(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	mp := sdkmetric.NewMeterProvider(sdkmetric.WithReader(reader))
	if _, err := myappmetric.NewTaskActive(semconvmeter.New(mp, "test")); err != nil {
		t.Fatalf("NewTaskActive() error = %v", err)
	}

	var taskActive myappmetric.TaskActive
	taskActive.Add(context.Background(), 1, myappattr.TaskStateDone)

	if m, ok := findMetric(collect(t, reader), "myapp.task.active"); ok {
		t.Fatalf("the zero TaskActive recorded %v, want nothing", m.Data)
	}
}
