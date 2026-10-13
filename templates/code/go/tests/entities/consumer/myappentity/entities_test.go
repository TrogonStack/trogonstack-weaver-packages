package myappentity_test

import (
	"testing"

	"go.opentelemetry.io/otel/attribute"

	"example.com/semconv/myappattr"
	"example.com/semconv/myappentity"
)

func TestNewHostRecordsRequiredAndOptionalAttributes(t *testing.T) {
	host := myappentity.NewHost(
		myappattr.NewHostName("h-1"),
		myappentity.WithHostHostType(myappattr.HostTypePhysical),
		myappentity.WithHostHostCPUCount(myappattr.NewHostCPUCount(4)),
	)

	want := attribute.NewSet(
		myappattr.NewHostName("h-1").KeyValue(),
		myappattr.HostTypePhysical.KeyValue(),
		myappattr.NewHostCPUCount(4).KeyValue(),
	)
	got := attribute.NewSet(host.Attributes()...)
	if !got.Equals(&want) {
		t.Fatalf("Attributes() = %v, want %v", got, want)
	}

	resourceAttrs := attribute.NewSet(host.Resource().Attributes()...)
	if !resourceAttrs.Equals(&want) {
		t.Fatalf("Resource().Attributes() = %v, want %v", resourceAttrs, want)
	}
}

func TestNewHostDropsZeroValueOptionalAttribute(t *testing.T) {
	host := myappentity.NewHost(
		myappattr.NewHostName("h-2"),
		myappentity.WithHostHostType(myappattr.HostType{}),
	)

	want := attribute.NewSet(myappattr.NewHostName("h-2").KeyValue())
	got := attribute.NewSet(host.Attributes()...)
	if !got.Equals(&want) {
		t.Fatalf("Attributes() = %v, want %v, the zero HostType must not be recorded", got, want)
	}
}

func TestZeroHostHasNoAttributes(t *testing.T) {
	var host myappentity.Host
	if attrs := host.Attributes(); len(attrs) != 0 {
		t.Fatalf("Attributes() = %v, want none", attrs)
	}
}
