import enum

from opentelemetry.sdk.metrics import MeterProvider
from opentelemetry.sdk.metrics.export import InMemoryMetricReader

from semconv import attributes, metrics


def test_integral_double_enum_values_remain_floats() -> None:
    assert issubclass(attributes.DemoSample, float)
    assert not issubclass(attributes.DemoSample, enum.IntEnum)
    assert [member.value for member in attributes.DemoSample] == [1.0, 1.0e3]
    assert all(isinstance(member.value, float) for member in attributes.DemoSample)


def test_integral_double_enum_values_reach_the_sdk() -> None:
    reader = InMemoryMetricReader()
    provider = MeterProvider(metric_readers=[reader])
    try:
        recorded = metrics.DemoRecorded(metrics.get_meter(provider))
        recorded.add(1, sample=attributes.DemoSample.SINGLE, upstream_sample=1.0)
        recorded.add(1, sample=attributes.DemoSample.LARGE, upstream_sample=1.0e3)
        data = reader.get_metrics_data()
        assert data is not None
        metric = data.resource_metrics[0].scope_metrics[0].metrics[0]
        assert [dict(point.attributes or {}) for point in metric.data.data_points] == [
            {"demo.sample": 1.0, "upstream.sample": 1.0},
            {"demo.sample": 1.0e3, "upstream.sample": 1.0e3},
        ]
    finally:
        provider.shutdown()
