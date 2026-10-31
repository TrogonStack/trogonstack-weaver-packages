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


def test_keys_and_class_names_follow_the_registry_names() -> None:
    assert attributes.HOST_KEY == "host"
    assert attributes.K8S_POD_UID_KEY == "k8s.pod.uid"
    assert attributes.HTTP_REQUEST_METHOD_ORIGINAL_KEY == "http.request.method_original"
    assert metrics.Requests.NAME == "requests"
    assert metrics.HttpServerRequestCount.NAME == "http.server.request_count"
    assert metrics.K8sPodCpuTime.NAME == "k8s.pod.cpu.time"


def test_enum_members_survive_awkward_values() -> None:
    members = {member.name: member.value for member in attributes.HttpRoute}
    assert members == {"ROOT": "/", "TWO_XX": "2xx", "CLASS": "class"}


def test_a_metric_without_a_namespace_keeps_every_segment() -> None:
    reader, provider = new_provider()
    metrics.Requests(metrics.get_meter(provider)).add(
        1, host="a", http_route=attributes.HttpRoute.ROOT
    )
    (point,) = number_points(collect(reader)["requests"])
    assert attributes_of(point) == {"host": "a", "http.route": "/"}


def test_a_leading_segment_is_dropped_from_parameter_names() -> None:
    reader, provider = new_provider()
    metrics.HttpServerRequestCount(metrics.get_meter(provider)).add(
        4,
        request_method_original="get",
        response_status_code=200,
        k8s_pod_uid="pod-1",
    )
    (point,) = number_points(collect(reader)["http.server.request_count"])
    assert point.value == 4
    assert attributes_of(point) == {
        "http.request.method_original": "get",
        "http.response.status_code": 200,
        "k8s.pod.uid": "pod-1",
    }


def test_digits_in_names_are_kept() -> None:
    reader, provider = new_provider()
    metrics.K8sPodCpuTime(metrics.get_meter(provider)).add(1.5)
    (point,) = number_points(collect(reader)["k8s.pod.cpu.time"])
    assert point.value == 1.5


def test_a_keyword_valued_member_is_recorded_by_value() -> None:
    reader, provider = new_provider()
    metrics.Requests(metrics.get_meter(provider)).add(
        1, host="a", http_route=attributes.HttpRoute.CLASS
    )
    (point,) = number_points(collect(reader)["requests"])
    assert attributes_of(point)["http.route"] == "class"
