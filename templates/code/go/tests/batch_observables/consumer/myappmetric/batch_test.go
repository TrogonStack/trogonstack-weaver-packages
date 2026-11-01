package myappmetric_test

import (
	"context"
	"errors"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
	sdkmetric "go.opentelemetry.io/otel/sdk/metric"
	"go.opentelemetry.io/otel/sdk/metric/metricdata"

	"example.com/semconv"
	"example.com/semconv/myappattr"
	"example.com/semconv/myappmetric"
	"example.com/semconv/semconvmeter"
	"example.com/semconv/workermetric"
)

func collect(t *testing.T, reader *sdkmetric.ManualReader) metricdata.ResourceMetrics {
	t.Helper()
	var result metricdata.ResourceMetrics
	if err := reader.Collect(context.Background(), &result); err != nil {
		t.Fatal(err)
	}
	return result
}

func findMetric(t *testing.T, result metricdata.ResourceMetrics, name string) metricdata.Metrics {
	t.Helper()
	for _, scope := range result.ScopeMetrics {
		for _, measurement := range scope.Metrics {
			if measurement.Name == name {
				return measurement
			}
		}
	}
	t.Fatalf("metric %q was not collected", name)
	return metricdata.Metrics{}
}

func newRecorder(t *testing.T, options ...sdkmetric.Option) (semconvmeter.Meter, *sdkmetric.ManualReader) {
	t.Helper()
	reader := sdkmetric.NewManualReader()
	provider := sdkmetric.NewMeterProvider(append(options, sdkmetric.WithReader(reader))...)
	t.Cleanup(func() { _ = provider.Shutdown(context.Background()) })
	return semconvmeter.New(provider, "batch-test"), reader
}

func newQueueDepth(t *testing.T, meter semconvmeter.Meter) myappmetric.QueueDepthBatchObservable {
	t.Helper()
	depth, err := myappmetric.NewQueueDepthBatchObservable(meter)
	if err != nil {
		t.Fatal(err)
	}
	return depth
}

func TestBatchCollectsOneSharedSnapshot(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	provider := sdkmetric.NewMeterProvider(sdkmetric.WithReader(reader))
	t.Cleanup(func() { _ = provider.Shutdown(context.Background()) })
	meter := semconvmeter.New(provider, "batch-test", metric.WithSchemaURL("https://example.com/override/9.0.0"))
	depth, err := myappmetric.NewQueueDepthBatchObservable(meter)
	if err != nil {
		t.Fatal(err)
	}
	ratio, err := workermetric.NewQueueRatioBatchObservable(meter)
	if err != nil {
		t.Fatal(err)
	}
	queue := myappattr.NewQueueName("main")
	const exact int64 = 9_007_199_254_740_993
	var calls atomic.Int64
	_, err = meter.RegisterCallback(func(_ context.Context, observer semconvmeter.Observer) error {
		snapshot := calls.Add(1)
		depth.Observe(observer, exact+snapshot, queue,
			myappmetric.WithQueueDepthTaskState(myappattr.TaskStateQueued))
		ratio.Observe(observer, float64(snapshot)/4, queue)
		return nil
	}, depth.Observable(), ratio.Observable())
	if err != nil {
		t.Fatal(err)
	}
	for snapshot := int64(1); snapshot <= 2; snapshot++ {
		result := collect(t, reader)
		if got := calls.Load(); got != snapshot {
			t.Fatalf("callback calls = %d, want %d", got, snapshot)
		}
		if len(result.ScopeMetrics) != 1 || result.ScopeMetrics[0].Scope.SchemaURL != semconv.SchemaURL {
			t.Fatalf("scope = %+v, want registry schema", result.ScopeMetrics)
		}
		depthMetric := findMetric(t, result, depth.Name())
		if depthMetric.Unit != "{task}" || depthMetric.Description != "Number of tasks waiting in the queue." {
			t.Fatalf("depth identity = %+v", depthMetric)
		}
		depthPoints := depthMetric.Data.(metricdata.Gauge[int64]).DataPoints
		if len(depthPoints) != 1 || depthPoints[0].Value != exact+snapshot {
			t.Fatalf("depth = %+v, want exact int64 %d", depthPoints, exact+snapshot)
		}
		wantAttributes := attribute.NewSet(queue.KeyValue(), myappattr.TaskStateQueued.KeyValue())
		if !depthPoints[0].Attributes.Equals(&wantAttributes) {
			t.Fatalf("attributes = %v, want %v", depthPoints[0].Attributes, wantAttributes)
		}
		ratioMetric := findMetric(t, result, ratio.Name())
		if ratioMetric.Unit != "1" || ratioMetric.Description != "Fraction of the queue in use." {
			t.Fatalf("ratio identity = %+v", ratioMetric)
		}
		ratioPoints := ratioMetric.Data.(metricdata.Gauge[float64]).DataPoints
		if len(ratioPoints) != 1 || ratioPoints[0].Value != float64(snapshot)/4 {
			t.Fatalf("ratio = %+v, want %v", ratioPoints, float64(snapshot)/4)
		}
	}
}

func TestBatchObserveSetAndCounterKinds(t *testing.T) {
	meter, reader := newRecorder(t)
	depth := newQueueDepth(t, meter)
	ratio, err := workermetric.NewQueueRatioBatchObservable(meter)
	if err != nil {
		t.Fatal(err)
	}
	completed, err := myappmetric.NewTasksCompletedBatchObservable(meter)
	if err != nil {
		t.Fatal(err)
	}
	active, err := myappmetric.NewQueueActiveBatchObservable(meter)
	if err != nil {
		t.Fatal(err)
	}
	queue := myappattr.NewQueueName("main")
	depthAttributes := myappmetric.NewQueueDepthSet(queue,
		myappmetric.WithQueueDepthTaskState(myappattr.TaskStateRunning))
	ratioAttributes := workermetric.NewQueueRatioSet(queue)
	_, err = meter.RegisterCallback(func(_ context.Context, observer semconvmeter.Observer) error {
		depth.ObserveSet(observer, 7, depthAttributes)
		ratio.ObserveSet(observer, 0.75, ratioAttributes)
		completed.Observe(observer, 9, queue)
		active.Observe(observer, -2, queue)
		return nil
	}, depth.Observable(), ratio.Observable(), completed.Observable(), active.Observable())
	if err != nil {
		t.Fatal(err)
	}
	result := collect(t, reader)
	depthPoints := findMetric(t, result, depth.Name()).Data.(metricdata.Gauge[int64]).DataPoints
	wantAttributes := attribute.NewSet(queue.KeyValue(), myappattr.TaskStateRunning.KeyValue())
	if len(depthPoints) != 1 || depthPoints[0].Value != 7 || !depthPoints[0].Attributes.Equals(&wantAttributes) {
		t.Fatalf("depth points = %+v, want value 7 and %v", depthPoints, wantAttributes)
	}
	ratioPoints := findMetric(t, result, ratio.Name()).Data.(metricdata.Gauge[float64]).DataPoints
	queueAttributes := attribute.NewSet(queue.KeyValue())
	if len(ratioPoints) != 1 || ratioPoints[0].Value != 0.75 || !ratioPoints[0].Attributes.Equals(&queueAttributes) {
		t.Fatalf("ratio points = %+v, want value 0.75 and %v", ratioPoints, queueAttributes)
	}
	completedSum := findMetric(t, result, completed.Name()).Data.(metricdata.Sum[int64])
	if !completedSum.IsMonotonic || len(completedSum.DataPoints) != 1 || completedSum.DataPoints[0].Value != 9 {
		t.Fatalf("completed sum = %+v, want monotonic value 9", completedSum)
	}
	activeSum := findMetric(t, result, active.Name()).Data.(metricdata.Sum[int64])
	if activeSum.IsMonotonic || len(activeSum.DataPoints) != 1 || activeSum.DataPoints[0].Value != -2 {
		t.Fatalf("active sum = %+v, want non-monotonic value -2", activeSum)
	}
}

func TestIndividualCallbacksRemainCompatible(t *testing.T) {
	meter, reader := newRecorder(t)
	if _, err := myappmetric.NewQueueDepthObservable(meter, nil); err == nil {
		t.Fatal("a nil individual callback was accepted")
	}
	queue := myappattr.NewQueueName("main")
	_, err := workermetric.NewQueueRatioObservable(meter,
		func(_ context.Context, observer workermetric.QueueRatioObserver) error {
			observer.Observe(0.25, queue)
			return nil
		})
	if err != nil {
		t.Fatal(err)
	}
	depth := newQueueDepth(t, meter)
	_, err = meter.RegisterCallback(func(_ context.Context, observer semconvmeter.Observer) error {
		depth.Observe(observer, 3, queue)
		return nil
	}, depth.Observable())
	if err != nil {
		t.Fatal(err)
	}
	result := collect(t, reader)
	ratio := findMetric(t, result, "worker.queue.ratio").Data.(metricdata.Gauge[float64]).DataPoints
	if len(ratio) != 1 || ratio[0].Value != 0.25 {
		t.Fatalf("individual points = %+v, want value 0.25", ratio)
	}
	depthPoints := findMetric(t, result, depth.Name()).Data.(metricdata.Gauge[int64]).DataPoints
	if len(depthPoints) != 1 || depthPoints[0].Value != 3 {
		t.Fatalf("batch points = %+v, want value 3", depthPoints)
	}
}

func TestBatchRegistrationValidation(t *testing.T) {
	meter, reader := newRecorder(t)
	depth := newQueueDepth(t, meter)
	foreignMeter, _ := newRecorder(t)
	foreign := newQueueDepth(t, foreignMeter)
	var calls atomic.Int64
	callback := func(context.Context, semconvmeter.Observer) error { calls.Add(1); return nil }
	for _, test := range []struct {
		name        string
		callback    semconvmeter.Callback
		instruments []semconvmeter.Observable
		wantError   string
	}{
		{name: "nil callback", instruments: []semconvmeter.Observable{depth.Observable()}, wantError: "callback is nil"},
		{name: "zero handle", callback: callback, instruments: []semconvmeter.Observable{{}}, wantError: "observable 0 is zero"},
		{name: "foreign handle", callback: callback, instruments: []semconvmeter.Observable{foreign.Observable()}, wantError: "belongs to another Meter"},
		{name: "valid then foreign", callback: callback, instruments: []semconvmeter.Observable{depth.Observable(), foreign.Observable()}, wantError: "belongs to another Meter"},
	} {
		t.Run(test.name, func(t *testing.T) {
			registration, err := meter.RegisterCallback(test.callback, test.instruments...)
			if err == nil || !strings.Contains(err.Error(), test.wantError) {
				t.Fatalf("RegisterCallback error = %v, want %q", err, test.wantError)
			}
			if err := registration.Unregister(); err != nil {
				t.Fatalf("failed registration Unregister = %v", err)
			}
		})
	}
	empty, err := meter.RegisterCallback(callback)
	if err != nil {
		t.Fatal(err)
	}
	if err := empty.Unregister(); err != nil {
		t.Fatal(err)
	}
	collect(t, reader)
	if calls.Load() != 0 {
		t.Fatal("an invalid or empty registration installed a callback")
	}
}

func TestBatchOwnershipUsesTheSameMeterValue(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	provider := sdkmetric.NewMeterProvider(sdkmetric.WithReader(reader))
	t.Cleanup(func() { _ = provider.Shutdown(context.Background()) })
	meter := semconvmeter.New(provider, "same-scope")
	other := semconvmeter.New(provider, "same-scope")
	depth := newQueueDepth(t, meter)
	callback := func(_ context.Context, observer semconvmeter.Observer) error {
		depth.Observe(observer, 3, myappattr.NewQueueName("main"))
		return nil
	}
	if _, err := other.RegisterCallback(callback, depth.Observable()); err == nil {
		t.Fatal("a separately created Meter accepted the handle")
	}
	copied := meter
	if _, err := copied.RegisterCallback(callback, depth.Observable(), depth.Observable()); err != nil {
		t.Fatal(err)
	}
	points := findMetric(t, collect(t, reader), depth.Name()).Data.(metricdata.Gauge[int64]).DataPoints
	if len(points) != 1 || points[0].Value != 3 {
		t.Fatalf("points = %+v, want one point with value 3", points)
	}
}

func TestBatchRestrictsObservationsToRegisteredHandles(t *testing.T) {
	meter, reader := newRecorder(t)
	depth := newQueueDepth(t, meter)
	ratio, err := workermetric.NewQueueRatioBatchObservable(meter)
	if err != nil {
		t.Fatal(err)
	}
	foreignMeter, foreignReader := newRecorder(t)
	foreign := newQueueDepth(t, foreignMeter)
	queue := myappattr.NewQueueName("main")
	_, err = meter.RegisterCallback(func(_ context.Context, observer semconvmeter.Observer) error {
		depth.Observe(observer, 5, queue)
		ratio.Observe(observer, 0.5, queue)
		ratio.ObserveSet(observer, 0.75, workermetric.NewQueueRatioSet(queue))
		foreign.Observe(observer, 100, queue)
		return nil
	}, depth.Observable())
	if err != nil {
		t.Fatal(err)
	}
	result := collect(t, reader)
	points := findMetric(t, result, depth.Name()).Data.(metricdata.Gauge[int64]).DataPoints
	if len(points) != 1 || points[0].Value != 5 {
		t.Fatalf("registered depth points = %+v, want value 5", points)
	}
	for _, scope := range result.ScopeMetrics {
		for _, measurement := range scope.Metrics {
			if measurement.Name != depth.Name() {
				t.Fatalf("unregistered metric %q was collected", measurement.Name)
			}
		}
	}
	if len(collect(t, foreignReader).ScopeMetrics) != 0 {
		t.Fatal("the foreign instrument recorded a measurement")
	}
}

func TestBatchCopiesTheRegistrationSet(t *testing.T) {
	meter, reader := newRecorder(t)
	depth := newQueueDepth(t, meter)
	instruments := []semconvmeter.Observable{depth.Observable()}
	_, err := meter.RegisterCallback(func(_ context.Context, observer semconvmeter.Observer) error {
		depth.Observe(observer, 4, myappattr.NewQueueName("main"))
		return nil
	}, instruments...)
	if err != nil {
		t.Fatal(err)
	}
	instruments[0] = semconvmeter.Observable{}
	points := findMetric(t, collect(t, reader), depth.Name()).Data.(metricdata.Gauge[int64]).DataPoints
	if len(points) != 1 || points[0].Value != 4 {
		t.Fatalf("points = %+v, want value 4", points)
	}
}

func TestBatchForwardsContextAndCallbackError(t *testing.T) {
	meter, reader := newRecorder(t)
	depth := newQueueDepth(t, meter)
	type contextKey struct{}
	callbackError := errors.New("sample unavailable")
	var receivedContext bool
	_, err := meter.RegisterCallback(func(ctx context.Context, observer semconvmeter.Observer) error {
		receivedContext = ctx.Value(contextKey{}) == "collection"
		depth.Observe(observer, 6, myappattr.NewQueueName("main"))
		return callbackError
	}, depth.Observable())
	if err != nil {
		t.Fatal(err)
	}
	var result metricdata.ResourceMetrics
	err = reader.Collect(context.WithValue(context.Background(), contextKey{}, "collection"), &result)
	if !errors.Is(err, callbackError) || !receivedContext {
		t.Fatalf("Collect error = %v, context forwarded = %v", err, receivedContext)
	}
	points := findMetric(t, result, depth.Name()).Data.(metricdata.Gauge[int64]).DataPoints
	if len(points) != 1 || points[0].Value != 6 {
		t.Fatalf("observations before callback error = %+v, want value 6", points)
	}
}

func TestBatchUnregisterStopsCollection(t *testing.T) {
	meter, reader := newRecorder(t)
	depth := newQueueDepth(t, meter)
	var calls atomic.Int64
	registration, err := meter.RegisterCallback(func(_ context.Context, observer semconvmeter.Observer) error {
		depth.Observe(observer, calls.Add(1), myappattr.NewQueueName("main"))
		return nil
	}, depth.Observable())
	if err != nil {
		t.Fatal(err)
	}
	collect(t, reader)
	if err := registration.Unregister(); err != nil {
		t.Fatal(err)
	}
	if err := registration.Unregister(); err != nil {
		t.Fatal(err)
	}
	collect(t, reader)
	if calls.Load() != 1 {
		t.Fatalf("callback calls = %d, want 1", calls.Load())
	}
}

func TestZeroBatchValuesAreHarmless(t *testing.T) {
	var meter semconvmeter.Meter
	var observer semconvmeter.Observer
	var registration semconvmeter.Registration
	var zeroDepth myappmetric.QueueDepthBatchObservable
	var zeroRatio workermetric.QueueRatioBatchObservable
	queue := myappattr.NewQueueName("main")
	zeroDepth.Observe(observer, 1, queue)
	zeroDepth.ObserveSet(observer, 1, myappmetric.NewQueueDepthSet(queue))
	zeroRatio.Observe(observer, 0.5, queue)
	zeroRatio.ObserveSet(observer, 0.5, workermetric.NewQueueRatioSet(queue))
	if zeroDepth.Name() != "myapp.queue.depth" || zeroRatio.Name() != "worker.queue.ratio" {
		t.Fatal("zero batch instruments lost their names")
	}
	depth := newQueueDepth(t, meter)
	called := false
	registration, err := meter.RegisterCallback(func(context.Context, semconvmeter.Observer) error {
		called = true
		return nil
	}, depth.Observable())
	if err != nil || called {
		t.Fatalf("zero Meter registration error = %v, called = %v", err, called)
	}
	if err := registration.Unregister(); err != nil {
		t.Fatal(err)
	}
	if err := (semconvmeter.Registration{}).Unregister(); err != nil {
		t.Fatal(err)
	}
}

func TestBatchRetainsMeasurementsWhenAnotherInstrumentIsDropped(t *testing.T) {
	meter, reader := newRecorder(t, sdkmetric.WithView(sdkmetric.NewView(
		sdkmetric.Instrument{Name: "worker.queue.ratio"},
		sdkmetric.Stream{Aggregation: sdkmetric.AggregationDrop{}},
	)))
	depth := newQueueDepth(t, meter)
	ratio, err := workermetric.NewQueueRatioBatchObservable(meter)
	if err != nil {
		t.Fatal(err)
	}
	queue := myappattr.NewQueueName("main")
	_, err = meter.RegisterCallback(func(_ context.Context, observer semconvmeter.Observer) error {
		depth.Observe(observer, 8, queue)
		ratio.Observe(observer, 0.5, queue)
		return nil
	}, depth.Observable(), ratio.Observable())
	if err != nil {
		t.Fatal(err)
	}
	result := collect(t, reader)
	points := findMetric(t, result, depth.Name()).Data.(metricdata.Gauge[int64]).DataPoints
	if len(points) != 1 || points[0].Value != 8 {
		t.Fatalf("retained points = %+v, want value 8", points)
	}
	for _, scope := range result.ScopeMetrics {
		for _, measurement := range scope.Metrics {
			if measurement.Name == ratio.Name() {
				t.Fatal("dropped instrument was collected")
			}
		}
	}
}

func TestBatchSupportsConcurrentCollections(t *testing.T) {
	firstReader := sdkmetric.NewManualReader()
	secondReader := sdkmetric.NewManualReader()
	provider := sdkmetric.NewMeterProvider(sdkmetric.WithReader(firstReader), sdkmetric.WithReader(secondReader))
	t.Cleanup(func() { _ = provider.Shutdown(context.Background()) })
	meter := semconvmeter.New(provider, "concurrent")
	depth := newQueueDepth(t, meter)
	entered := make(chan struct{}, 2)
	release := make(chan struct{})
	unblock := sync.OnceFunc(func() { close(release) })
	defer unblock()
	var calls atomic.Int64
	_, err := meter.RegisterCallback(func(ctx context.Context, observer semconvmeter.Observer) error {
		value := calls.Add(1)
		entered <- struct{}{}
		select {
		case <-release:
			depth.Observe(observer, value, myappattr.NewQueueName("main"))
		case <-ctx.Done():
			return ctx.Err()
		}
		return nil
	}, depth.Observable())
	if err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	results := make(chan error, 2)
	for _, reader := range []*sdkmetric.ManualReader{firstReader, secondReader} {
		go func() {
			var result metricdata.ResourceMetrics
			results <- reader.Collect(ctx, &result)
		}()
	}
	for range 2 {
		select {
		case <-entered:
		case <-ctx.Done():
			t.Fatal("callbacks did not overlap across readers")
		}
	}
	unblock()
	for range 2 {
		select {
		case err := <-results:
			if err != nil {
				t.Fatal(err)
			}
		case <-ctx.Done():
			t.Fatal("concurrent collections did not complete")
		}
	}
	if calls.Load() != 2 {
		t.Fatalf("callback calls = %d, want 2", calls.Load())
	}
}
