import enum

from opentelemetry.sdk.metrics import MeterProvider
from opentelemetry.sdk.metrics.export import InMemoryMetricReader

from semconv import attributes, metrics


def test_integer_enum_values_are_exact() -> None:
    assert issubclass(attributes.DemoLimit, enum.IntEnum)
    assert [member.value for member in attributes.DemoLimit] == [
        -9223372036854775808,
        -9223372036854775807,
        -9007199254740993,
        9007199254740993,
        9223372036854775806,
        9223372036854775807,
    ]


def test_integer_enum_values_reach_the_sdk_unchanged() -> None:
    reader = InMemoryMetricReader()
    provider = MeterProvider(metric_readers=[reader])
    try:
        recorded = metrics.DemoRecorded(metrics.get_meter(provider))
        for member in attributes.DemoLimit:
            recorded.add(1, limit=member, upstream_limit=9223372036854775807)
        data = reader.get_metrics_data()
        assert data is not None
        metric = data.resource_metrics[0].scope_metrics[0].metrics[0]
        assert [dict(point.attributes or {}) for point in metric.data.data_points] == [
            {"demo.limit": member.value, "upstream.limit": 9223372036854775807}
            for member in attributes.DemoLimit
        ]
    finally:
        provider.shutdown()
