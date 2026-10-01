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
	"int",
	"double",
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

# The value type follows the code_generation.metric_value_type annotation the
# OpenTelemetry semantic conventions use; a missing and a misspelled one fail
# the same way.
deny contains finding if {
	some metric in input.registry.metrics
	value_type := object.get(metric, ["annotations", "code_generation", "metric_value_type"], "")
	not value_type in renderable_value_types

	finding := {
		"id": "go_codegen_invalid_metric_value_type",
		"context": {"metric_value_type": value_type},
		"message": sprintf(
			"Metric '%s' must set annotations.code_generation.metric_value_type to one of: %s. The Go template reads the instrument's value type from it.",
			[metric.name, concat(", ", sort(renderable_value_types))],
		),
		"level": "violation",
		"signal_type": "metric",
		"signal_name": metric.name,
	}
}

# Experimental: annotations.aggregation mirrors the metric aggregation field
# proposed in https://github.com/open-telemetry/weaver/issues/844 until the
# registry schema has one. The Go template accepts only what the Go API can
# express, which is explicit bucket boundaries on a histogram.
deny contains finding if {
	some metric in input.registry.metrics
	aggregation := metric.annotations.aggregation
	problem := aggregation_problem(metric.instrument, aggregation)

	finding := {
		"id": "go_codegen_invalid_aggregation",
		"context": {"aggregation": aggregation},
		"message": sprintf(
			"Metric '%s' sets annotations.aggregation, but %s.",
			[metric.name, problem],
		),
		"level": "violation",
		"signal_type": "metric",
		"signal_name": metric.name,
	}
}

aggregation_problem(instrument, aggregation) := "the Go template supports it only on a histogram" if {
	instrument != "histogram"
} else := "it must be an object with a method and parameters" if {
	not is_object(aggregation)
} else := "the Go template supports only the explicithistogram method" if {
	object.get(aggregation, "method", "") != "explicithistogram"
} else := "the Go API can express only parameters.boundaries" if {
	not only_boundaries(aggregation)
} else := "parameters.boundaries must be a non-empty list of numbers" if {
	not number_list(aggregation.parameters.boundaries)
} else := "parameters.boundaries must be in strictly increasing order" if {
	bounds := aggregation.parameters.boundaries
	some i, bound in bounds
	i > 0
	bound <= bounds[i - 1]
}

only_boundaries(aggregation) if {
	every key, _ in aggregation {
		key in {"method", "parameters"}
	}
	is_object(aggregation.parameters)
	every key, _ in aggregation.parameters {
		key == "boundaries"
	}
}

number_list(bounds) if {
	is_array(bounds)
	count(bounds) > 0
	every bound in bounds {
		is_number(bound)
	}
}

# The OpenTelemetry schema URL ends in the schema version, and weaver resolves
# a registry without a manifest to a placeholder URL that names no schema.
deny contains finding if {
	schema_url := object.get(input, "schema_url", "")
	not versioned_schema_url(schema_url)

	finding := {
		"id": "go_codegen_invalid_schema_url",
		"context": {"schema_url": schema_url},
		"message": sprintf(
			"The registry schema_url '%s' must be declared in the registry manifest and end in the schema version, such as 'https://example.com/schemas/1.0.0'. The Go template ties every instrument to it.",
			[schema_url],
		),
		"level": "violation",
	}
}

versioned_schema_url(schema_url) if {
	schema_url != "https://unknown/unknown"
	regex.match(`^https?://[^/]+(/[^/]+)*/[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$`, schema_url)
}
