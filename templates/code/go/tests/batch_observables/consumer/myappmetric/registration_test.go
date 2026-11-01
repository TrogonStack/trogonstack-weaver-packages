package myappmetric_test

import (
	"context"
	"errors"
	"sync/atomic"
	"testing"

	"go.opentelemetry.io/otel/metric"
	sdkmetric "go.opentelemetry.io/otel/sdk/metric"
	"go.opentelemetry.io/otel/sdk/metric/metricdata"

	"example.com/semconv/myappattr"
	"example.com/semconv/semconvmeter"
)

type decoratedProvider struct {
	metric.MeterProvider
	decorate func(metric.Meter) metric.Meter
}

func (provider decoratedProvider) Meter(name string, options ...metric.MeterOption) metric.Meter {
	return provider.decorate(provider.MeterProvider.Meter(name, options...))
}

type nonComparableMeter struct {
	metric.Meter
	values []int
}

func TestBatchDoesNotRequireComparableNativeMeters(t *testing.T) {
	reader := sdkmetric.NewManualReader()
	provider := sdkmetric.NewMeterProvider(sdkmetric.WithReader(reader))
	t.Cleanup(func() { _ = provider.Shutdown(context.Background()) })
	meter := semconvmeter.New(decoratedProvider{
		MeterProvider: provider,
		decorate: func(meter metric.Meter) metric.Meter {
			return nonComparableMeter{Meter: meter, values: []int{1}}
		},
	}, "decorated")
	depth := newQueueDepth(t, meter)
	_, err := meter.RegisterCallback(func(_ context.Context, observer semconvmeter.Observer) error {
		depth.Observe(observer, 2, myappattr.NewQueueName("main"))
		return nil
	}, depth.Observable())
	if err != nil {
		t.Fatal(err)
	}
	points := findMetric(t, collect(t, reader), depth.Name()).Data.(metricdata.Gauge[int64]).DataPoints
	if len(points) != 1 || points[0].Value != 2 {
		t.Fatalf("points = %+v, want value 2", points)
	}
}

type failingRegistration struct {
	metric.Registration
	unregistered atomic.Int64
	failure      error
}

func (registration *failingRegistration) Unregister() error {
	registration.unregistered.Add(1)
	return registration.failure
}

type registrationFailureMeter struct {
	metric.Meter
	registration metric.Registration
	failure      error
}

func (meter registrationFailureMeter) RegisterCallback(metric.Callback, ...metric.Observable) (metric.Registration, error) {
	return meter.registration, meter.failure
}

func TestBatchCleansUpFailedNativeRegistration(t *testing.T) {
	provider := sdkmetric.NewMeterProvider()
	t.Cleanup(func() { _ = provider.Shutdown(context.Background()) })
	registrationError := errors.New("registration rejected")
	cleanupError := errors.New("registration cleanup failed")
	nativeRegistration := &failingRegistration{failure: cleanupError}
	meter := semconvmeter.New(decoratedProvider{
		MeterProvider: provider,
		decorate: func(meter metric.Meter) metric.Meter {
			return registrationFailureMeter{
				Meter: meter, registration: nativeRegistration, failure: registrationError,
			}
		},
	}, "failed-registration")
	depth := newQueueDepth(t, meter)
	registration, err := meter.RegisterCallback(func(context.Context, semconvmeter.Observer) error {
		return nil
	}, depth.Observable())
	if !errors.Is(err, registrationError) || !errors.Is(err, cleanupError) {
		t.Fatalf("RegisterCallback error = %v, want registration and cleanup errors", err)
	}
	if nativeRegistration.unregistered.Load() != 1 {
		t.Fatalf("native cleanup calls = %d, want 1", nativeRegistration.unregistered.Load())
	}
	if err := registration.Unregister(); err != nil {
		t.Fatal(err)
	}
	if nativeRegistration.unregistered.Load() != 1 {
		t.Fatal("the failed registration remained exposed")
	}
}
