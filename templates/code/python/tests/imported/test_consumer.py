from opentelemetry.sdk.metrics import MeterProvider
from opentelemetry.sdk.metrics.export import (
    HistogramDataPoint,
    InMemoryMetricReader,
    Metric,
    NumberDataPoint,
)
from opentelemetry.util.types import AttributeValue

from semconv import (
    INSTRUMENTATION_NAME,
    INSTRUMENTATION_VERSION,
    SCHEMA_URL,
    attributes,
    metrics,
)


def collect(reader: InMemoryMetricReader) -> dict[str, Metric]:
    data = reader.get_metrics_data()
    assert data is not None
    scopes = data.resource_metrics[0].scope_metrics
    assert len(scopes) == 1
    scope = scopes[0].scope
    assert scope.name == INSTRUMENTATION_NAME
    assert scope.version == INSTRUMENTATION_VERSION
    assert scope.schema_url == SCHEMA_URL
    return {metric.name: metric for metric in scopes[0].metrics}


def new_provider() -> tuple[InMemoryMetricReader, MeterProvider]:
    reader = InMemoryMetricReader()
    return reader, MeterProvider(metric_readers=[reader])


def attributes_of(
    point: NumberDataPoint | HistogramDataPoint,
) -> dict[str, AttributeValue]:
    return dict(point.attributes or {})


def number_points(metric: Metric) -> list[NumberDataPoint]:
    points = list(metric.data.data_points)
    assert all(isinstance(point, NumberDataPoint) for point in points)
    return [point for point in points if isinstance(point, NumberDataPoint)]


def histogram_points(metric: Metric) -> list[HistogramDataPoint]:
    points = list(metric.data.data_points)
    assert all(isinstance(point, HistogramDataPoint) for point in points)
    return [point for point in points if isinstance(point, HistogramDataPoint)]


def test_a_local_attribute_uses_its_constant() -> None:
    assert attributes.MYAPP_TASK_ID_KEY == "myapp.task.id"
    assert not hasattr(attributes, "ERROR_TYPE_KEY")
    assert not hasattr(attributes, "SERVER_ADDRESS_KEY")


def test_imported_attributes_use_literal_keys_and_registry_types() -> None:
    reader, provider = new_provider()
    metrics.MyappTaskFailed(metrics.get_meter(provider)).add(
        1,
        error_type="timeout",
        task_id="task-1",
        http_request_method_list=["GET", "POST"],
    )
    (point,) = number_points(collect(reader)["myapp.task.failed"])
    assert attributes_of(point) == {
        "error.type": "timeout",
        "myapp.task.id": "task-1",
        "http.request.method_list": ("GET", "POST"),
    }


def test_imported_optional_attributes_are_omitted_when_none() -> None:
    reader, provider = new_provider()
    duration = metrics.EdgeUpstreamDuration(metrics.get_meter(provider))
    duration.record(0.25, server_address="upstream.example.com")
    duration.record(0.5, server_address="upstream.example.com", server_port=443)
    points = histogram_points(collect(reader)["edge.upstream.duration"])
    assert len(points) == 2
    recorded = sorted(
        sorted(attributes_of(point).items(), key=lambda item: item[0])
        for point in points
    )
    assert recorded == [
        [("server.address", "upstream.example.com")],
        [("server.address", "upstream.example.com"), ("server.port", 443)],
    ]


def test_imported_attributes_work_in_observations() -> None:
    observation = metrics.MyappTaskFailedObservable.observation(
        2, error_type="boom", task_id="task-1"
    )
    assert observation.value == 2
    assert observation.attributes == {"error.type": "boom", "myapp.task.id": "task-1"}
