package myappattr_test

import (
	"testing"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"

	"example.com/semconv/myappattr"
)

func TestKeyValue(t *testing.T) {
	kv := myappattr.NewTaskID("t-1").KeyValue()
	want := attribute.String("myapp.task.id", "t-1")
	if kv != want {
		t.Fatalf("KeyValue() = %v, want %v", kv, want)
	}
}

func TestEnumMemberKeyValue(t *testing.T) {
	kv := myappattr.TaskStateRunning.KeyValue()
	want := attribute.String("myapp.task.state", "running")
	if kv != want {
		t.Fatalf("KeyValue() = %v, want %v", kv, want)
	}
}

func TestZeroValueEnumDropsWithoutReporting(t *testing.T) {
	var reported error
	otel.SetErrorHandler(otel.ErrorHandlerFunc(func(err error) { reported = err }))
	t.Cleanup(func() { otel.SetErrorHandler(otel.ErrorHandlerFunc(func(error) {})) })

	kv := myappattr.TaskState{}.KeyValue()

	if kv.Valid() {
		t.Fatalf("KeyValue() = %v, want an invalid attribute.KeyValue", kv)
	}
	if reported != nil {
		t.Fatalf("the zero TaskState reported %v, want nothing reported", reported)
	}
}

func TestZeroValueBooleanEnumIsNotItsFalseMember(t *testing.T) {
	var reported error
	otel.SetErrorHandler(otel.ErrorHandlerFunc(func(err error) { reported = err }))
	t.Cleanup(func() { otel.SetErrorHandler(otel.ErrorHandlerFunc(func(error) {})) })

	if kv := myappattr.TaskPreemptibleForbidden.KeyValue(); !kv.Valid() || kv.Value.AsBool() {
		t.Fatalf("TaskPreemptibleForbidden.KeyValue() = %v, want a valid false attribute", kv)
	}

	zero := myappattr.TaskPreemptible{}.KeyValue()
	if zero.Valid() {
		t.Fatalf("the zero TaskPreemptible.KeyValue() = %v, want an invalid attribute", zero)
	}
	if reported != nil {
		t.Fatalf("the zero TaskPreemptible reported %v, want nothing reported", reported)
	}
}
