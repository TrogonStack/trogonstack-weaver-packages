import pytest
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


def test_every_scalar_type_is_recorded_as_given() -> None:
    reader, provider = new_provider()
    metrics.MyappTaskScalarsRequired(metrics.get_meter(provider)).add(
        1,
        task_cancelled=False,
        task_id="task-1",
        task_progress=0.5,
        task_retries=3,
    )
    (point,) = number_points(collect(reader)["myapp.task.scalars.required"])
    recorded = attributes_of(point)
    assert recorded == {
        "myapp.task.cancelled": False,
        "myapp.task.id": "task-1",
        "myapp.task.progress": 0.5,
        "myapp.task.retries": 3,
    }
    assert isinstance(recorded["myapp.task.cancelled"], bool)
    assert isinstance(recorded["myapp.task.progress"], float)


def test_optional_scalars_are_omitted_when_none() -> None:
    reader, provider = new_provider()
    optional = metrics.MyappTaskScalarsOptional(metrics.get_meter(provider))
    optional.add(1)
    optional.add(1, task_cancelled=True, task_retries=0)
    optional.add(1, task_id="task-1")
    points = number_points(collect(reader)["myapp.task.scalars.optional"])
    assert sorted(sorted(attributes_of(point).items()) for point in points) == [
        [],
        [("myapp.task.cancelled", True), ("myapp.task.retries", 0)],
        [("myapp.task.id", "task-1")],
    ]


def test_falsy_optional_values_are_not_omitted() -> None:
    reader, provider = new_provider()
    metrics.MyappTaskScalarsOptional(metrics.get_meter(provider)).add(
        1,
        task_cancelled=False,
        task_id="",
        task_progress=0.0,
        task_retries=0,
    )
    (point,) = number_points(collect(reader)["myapp.task.scalars.optional"])
    assert attributes_of(point) == {
        "myapp.task.cancelled": False,
        "myapp.task.id": "",
        "myapp.task.progress": 0.0,
        "myapp.task.retries": 0,
    }


def test_array_attributes_are_copied_into_sequences() -> None:
    reader, provider = new_provider()
    tags = ["nightly", "batch"]
    metrics.MyappTaskArraysRequired(metrics.get_meter(provider)).add(
        2,
        task_flags=[True, False],
        task_shard_ids=(1, 2),
        task_tags=tags,
        task_weights=[0.25, 0.75],
    )
    tags.append("late")
    (point,) = number_points(collect(reader)["myapp.task.arrays.required"])
    assert attributes_of(point) == {
        "myapp.task.flags": (True, False),
        "myapp.task.shard_ids": (1, 2),
        "myapp.task.tags": ("nightly", "batch"),
        "myapp.task.weights": (0.25, 0.75),
    }


def test_optional_arrays_are_omitted_when_none() -> None:
    reader, provider = new_provider()
    gauge = metrics.MyappTaskArraysOptional(metrics.get_meter(provider))
    gauge.set(1.5)
    gauge.set(2.5, task_tags=[], task_shard_ids=[7])
    points = number_points(collect(reader)["myapp.task.arrays.optional"])
    assert sorted(
        (point.value, sorted(attributes_of(point).items())) for point in points
    ) == [
        (1.5, []),
        (2.5, [("myapp.task.shard_ids", (7,)), ("myapp.task.tags", ())]),
    ]


def test_every_enum_type_is_recorded_by_value() -> None:
    reader, provider = new_provider()
    metrics.MyappTaskEnumsRequired(metrics.get_meter(provider)).add(
        1,
        auth_level=attributes.AuthLevel.HIGH,
        auth_method=attributes.AuthMethod.TOKEN,
        task_preemptible=attributes.MyappTaskPreemptible.FORBIDDEN,
        task_sample_rate=attributes.MyappTaskSampleRate.TENTH,
        task_state=attributes.MyappTaskState.RUNNING,
    )
    (point,) = number_points(collect(reader)["myapp.task.enums.required"])
    recorded = attributes_of(point)
    assert recorded == {
        "auth.level": 2,
        "auth.method": "token",
        "myapp.task.preemptible": False,
        "myapp.task.sample_rate": 0.1,
        "myapp.task.state": "running",
    }
    assert type(recorded["auth.method"]) is str
    assert type(recorded["auth.level"]) is int
    assert type(recorded["myapp.task.preemptible"]) is bool
    assert type(recorded["myapp.task.sample_rate"]) is float


def test_optional_enums_are_omitted_when_none() -> None:
    reader, provider = new_provider()
    optional = metrics.MyappTaskEnumsOptional(metrics.get_meter(provider))
    optional.add(1)
    optional.add(1, task_state=attributes.MyappTaskState.QUEUED)
    points = number_points(collect(reader)["myapp.task.enums.optional"])
    assert sorted(sorted(attributes_of(point).items()) for point in points) == [
        [],
        [("myapp.task.state", "queued")],
    ]


def test_required_and_optional_attributes_mix() -> None:
    reader, provider = new_provider()
    metrics.MyappTaskMixed(metrics.get_meter(provider)).record(
        0.75,
        auth_method=attributes.AuthMethod.PASSWORD,
        task_id="task-1",
        task_tags=["a"],
    )
    (point,) = histogram_points(collect(reader)["myapp.task.mixed"])
    assert attributes_of(point) == {
        "auth.method": "password",
        "myapp.task.id": "task-1",
        "myapp.task.tags": ("a",),
    }
    assert point.sum == 0.75


def test_observations_carry_the_same_attributes() -> None:
    observation = metrics.MyappTaskEnumsRequiredObservable.observation(
        5,
        auth_level=attributes.AuthLevel.LOW,
        auth_method=attributes.AuthMethod.PASSWORD,
        task_preemptible=attributes.MyappTaskPreemptible.ALLOWED,
        task_sample_rate=attributes.MyappTaskSampleRate.ALL,
        task_state=attributes.MyappTaskState.QUEUED,
    )
    assert observation.value == 5
    assert observation.attributes == {
        "auth.level": 1,
        "auth.method": "password",
        "myapp.task.preemptible": True,
        "myapp.task.sample_rate": 1.0,
        "myapp.task.state": "queued",
    }
    sparse = metrics.MyappTaskScalarsOptionalObservable.observation(1, task_id="x")
    assert sparse.attributes == {"myapp.task.id": "x"}


def test_the_type_checker_rejects_misuse() -> None:
    counter = metrics.MyappTaskScalarsRequired(metrics.get_meter(MeterProvider()))
    with pytest.raises(TypeError):
        counter.add(1, task_id="x")  # type: ignore[call-arg]
    with pytest.raises(TypeError):
        counter.add(  # type: ignore[call-arg]
            1,
            task_cancelled=False,
            task_id="x",
            task_progress=0.0,
            task_retries=0,
            task_unknown=1,
        )
