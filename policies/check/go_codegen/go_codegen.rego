package after_resolution

import rego.v1

renderable_types := {
	"string",
	"string[]",
	"int",
	"int[]",
	"double",
	"double[]",
	"boolean",
	"boolean[]",
}

renderable_value_types := {
	"int64",
	"float64",
}

# Enums arrive as an object with members, which the template renders; every
# other type arrives as a string.
deny contains finding if {
	some attr in input.registry.attributes
	is_string(attr.type)
	not attr.type in renderable_types

	finding := {
		"id": "go_codegen_unsupported_attribute_type",
		"context": {
			"attribute_key": attr.key,
			"attribute_type": attr.type,
		},
		"message": sprintf(
			"Attribute '%s' has type '%s', which the Go template does not render. Use one of: %s, or an enum.",
			[attr.key, attr.type, concat(", ", sort(renderable_types))],
		),
		"level": "violation",
	}
}

deny contains finding if {
	some attr in input.registry.attributes
	count(split(attr.key, ".")) < 2

	finding := {
		"id": "go_codegen_attribute_without_namespace",
		"context": {"attribute_key": attr.key},
		"message": sprintf(
			"Attribute '%s' has no namespace. The Go template puts each attribute in a package named for its namespace and names it after the rest of the key, so the key needs both.",
			[attr.key],
		),
		"level": "violation",
	}
}

deny contains finding if {
	some metric in input.registry.metrics
	count(split(metric.name, ".")) < 2

	finding := {
		"id": "go_codegen_metric_without_namespace",
		"context": {},
		"message": sprintf(
			"Metric '%s' has no namespace. The Go template puts each metric in a package named for its namespace and names it after the rest of the name, so the name needs both.",
			[metric.name],
		),
		"level": "violation",
		"signal_type": "metric",
		"signal_name": metric.name,
	}
}

# The registry schema has no field for an instrument's value type, so it
# travels as an annotation; a missing and a misspelled one fail the same way.
deny contains finding if {
	some metric in input.registry.metrics
	value_type := object.get(metric, ["annotations", "go", "value_type"], "")
	not value_type in renderable_value_types

	finding := {
		"id": "go_codegen_invalid_metric_value_type",
		"context": {"value_type": value_type},
		"message": sprintf(
			"Metric '%s' must set annotations.go.value_type to one of: %s. The Go template reads the instrument's value type from it.",
			[metric.name, concat(", ", sort(renderable_value_types))],
		),
		"level": "violation",
		"signal_type": "metric",
		"signal_name": metric.name,
	}
}
