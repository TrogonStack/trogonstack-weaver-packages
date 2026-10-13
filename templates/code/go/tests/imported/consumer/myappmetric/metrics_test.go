package myappmetric_test

import (
	"context"
	"testing"

	"go.opentelemetry.io/otel/attribute"
	sdkmetric "go.opentelemetry.io/otel/sdk/metric"
	"go.opentelemetry.io/otel/sdk/metric/metricdata"

	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
	"example.com/semconv/semconvmeter"
)

func TestTaskFailedRecordsRawAndTypedAttributes(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	mp := sdkmetric.NewMeterProvider(sdkmetric.WithReader(reader))
	meter := semconvmeter.New(mp, "test")

	taskFailed, err := myappmetric.NewTaskFailed(meter)
	if err != nil {
		t.Fatalf("NewTaskFailed() error = %v", err)
	}

	taskFailed.Add(
		context.Background(),
		1,
		"timeout",
		myappattr.NewTaskID("t-1"),
		myappmetric.WithTaskFailedHTTPRequestMethodList([]string{"GET", "POST"}),
	)

	var rm metricdata.ResourceMetrics
	if err := reader.Collect(context.Background(), &rm); err != nil {
		t.Fatalf("Collect() error = %v", err)
	}
	var m metricdata.Metrics
	var found bool
	for _, sm := range rm.ScopeMetrics {
		for _, candidate := range sm.Metrics {
			if candidate.Name == "myapp.task.failed" {
				m, found = candidate, true
			}
		}
	}
	if !found {
		t.Fatal("myapp.task.failed was not recorded")
	}
	sum := m.Data.(metricdata.Sum[int64])
	if len(sum.DataPoints) != 1 {
		t.Fatalf("len(DataPoints) = %d, want 1", len(sum.DataPoints))
	}
	want := attribute.NewSet(
		attribute.String("error.type", "timeout"),
		myappattr.NewTaskID("t-1").KeyValue(),
		attribute.StringSlice("http.request.method_list", []string{"GET", "POST"}),
	)
	got := sum.DataPoints[0].Attributes
	if !got.Equals(&want) {
		t.Fatalf("Attributes = %v, want %v", got, want)
	}
}
