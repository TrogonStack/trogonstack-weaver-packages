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


PROFILE = ["alpha", "beta"]


def test_wrapped_required_assignments_record_every_attribute() -> None:
    reader, provider = new_provider()
    metrics.MyappTaskLong(metrics.get_meter(provider)).add(
        1,
        task_config_env_kind=attributes.MyappTaskConfigEnvKind.STAGING,
        task_config_profile_identifier=PROFILE,
        task_environment=attributes.MyappTaskEnvironment.PRODUCTION,
        task_tags=["a"],
        task_config_profile_revision=["r1"],
    )
    (point,) = number_points(collect(reader)["myapp.task.long"])
    assert attributes_of(point) == {
        "myapp.task.config_env_kind": "staging",
        "myapp.task.config_profile_identifier": ("alpha", "beta"),
        "myapp.task.environment": "production",
        "myapp.task.tags": ("a",),
        "myapp.task.config_profile_revision": ("r1",),
    }


def test_wrapped_optional_assignments_are_omitted_when_none() -> None:
    reader, provider = new_provider()
    optional = metrics.MyappTaskLongOptional(metrics.get_meter(provider))
    optional.add(1)
    optional.add(
        2,
        task_config_env_kind=attributes.MyappTaskConfigEnvKind.PRODUCTION,
        task_config_profile_identifier=PROFILE,
        task_environment=attributes.MyappTaskEnvironment.PRODUCTION,
        task_tags=[],
    )
    points = number_points(collect(reader)["myapp.task.long.optional"])
    by_value = {point.value: attributes_of(point) for point in points}
    assert by_value[1] == {}
    assert by_value[2] == {
        "myapp.task.config_env_kind": "production",
        "myapp.task.config_profile_identifier": ("alpha", "beta"),
        "myapp.task.environment": "production",
        "myapp.task.tags": (),
    }


def test_wrapped_observations_carry_the_same_attributes() -> None:
    observation = metrics.MyappTaskLongObservable.observation(
        1,
        task_config_env_kind=attributes.MyappTaskConfigEnvKind.PRODUCTION,
        task_config_profile_identifier=PROFILE,
        task_environment=attributes.MyappTaskEnvironment.PRODUCTION,
        task_tags=["a"],
    )
    assert observation.attributes == {
        "myapp.task.config_env_kind": "production",
        "myapp.task.config_profile_identifier": ["alpha", "beta"],
        "myapp.task.environment": "production",
        "myapp.task.tags": ["a"],
    }
