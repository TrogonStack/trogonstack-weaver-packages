package myappspan_test

import (
	"context"
	"math"
	"testing"

	"go.opentelemetry.io/otel/attribute"

	"example.com/semconv/myappattr"
	"example.com/semconv/myappspan"
	"example.com/semconv/semconvtracer"
)

func startedName(t *testing.T, start func(context.Context, semconvtracer.Tracer)) string {
	t.Helper()
	tracer, recorder := newTracer(t)
	start(context.Background(), tracer)
	started := recorder.Started()
	if len(started) != 1 {
		t.Fatalf("len(Started()) = %d, want 1", len(started))
	}
	return started[0].Name()
}

func TestNamesWriteValuesAsEmitWritesThem(t *testing.T) {
	floats := []float64{
		0, math.Copysign(0, -1), 1, -1.5, 0.1, 1e-6, 1e-7, 1.2345678901234567e-6, -1.2345678901234567e-6,
		1e20, 1e21, 9.999999999999999e20, 123456789012345680000, 1e-100, math.MaxFloat64, -math.MaxFloat64,
		math.SmallestNonzeroFloat64, math.Inf(1), math.Inf(-1), math.NaN(),
	}
	for _, value := range floats {
		want := attribute.Float64Value(value).Emit()
		if got := startedName(t, func(ctx context.Context, tracer semconvtracer.Tracer) {
			myappspan.StartNameFloat(ctx, tracer, myappattr.NewNumberValue(value))
		}); got != want {
			t.Errorf("name from %v = %q, want %q", value, got, want)
		}
	}

	for _, value := range []int{0, 7, -7, math.MaxInt64, math.MinInt64} {
		want := "attempt " + attribute.IntValue(value).Emit()
		if got := startedName(t, func(ctx context.Context, tracer semconvtracer.Tracer) {
			myappspan.StartNameAttempt(ctx, tracer, myappattr.NewTaskAttempt(value))
		}); got != want {
			t.Errorf("name from %v = %q, want %q", value, got, want)
		}
	}

	for _, test := range []struct {
		tags    []string
		indices []int
		samples []float64
		flags   []bool
	}{
		{[]string{"a"}, []int{1}, []float64{1.5}, []bool{true}},
		{
			[]string{`"quoted" \ <b>&`, "\x00\b\f\n\r\t\x1f\x7f", "\xff\xfe invalid", "  ", "é 日本"},
			[]int{math.MinInt64, 0, math.MaxInt64},
			floats[:17],
			[]bool{true, false, true},
		},
		{[]string{"a", "b"}, []int{-1, 2}, []float64{1, math.NaN(), math.Inf(-1), 1e-7}, []bool{false}},
	} {
		want := attribute.StringSliceValue(test.tags).Emit() + ":" +
			attribute.IntSliceValue(test.indices).Emit() + ":" +
			attribute.Float64SliceValue(test.samples).Emit() + ":" +
			attribute.BoolSliceValue(test.flags).Emit()
		if got := startedName(t, func(ctx context.Context, tracer semconvtracer.Tracer) {
			myappspan.StartNameArrays(ctx, tracer,
				myappattr.NewArrayFlags(test.flags...),
				myappattr.NewArrayIndices(test.indices...),
				myappattr.NewArraySamples(test.samples...),
				myappattr.NewArrayTags(test.tags...),
			)
		}); got != want {
			t.Errorf("name from arrays = %q, want %q", got, want)
		}
	}
}

func TestNamesSkipATemplateWithAnEmptyArray(t *testing.T) {
	if got := startedName(t, func(ctx context.Context, tracer semconvtracer.Tracer) {
		myappspan.StartNameIndices(ctx, tracer, myappattr.NewArrayIndices())
	}); got != "myapp.name.indices" {
		t.Fatalf("name = %q, want %q", got, "myapp.name.indices")
	}
}
