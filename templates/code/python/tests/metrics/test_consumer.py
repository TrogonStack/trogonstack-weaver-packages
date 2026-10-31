from collections.abc import Iterable

import pytest
from opentelemetry.metrics import CallbackOptions, Observation
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


def test_scope_carries_the_schema_url() -> None:
    reader, provider = new_provider()
    meter = metrics.get_meter(provider)
    assert meter.schema_url == SCHEMA_URL
    metrics.MyappTaskStarted(meter).add(1, task_id="a")
    collect(reader)


def test_constants_match_the_registry() -> None:
    assert SCHEMA_URL == "https://example.com/schemas/metrics/1.0.0"
    assert metrics.MyappTaskDuration.NAME == "myapp.task.duration"
    assert metrics.MyappTaskStarted.NAME == "myapp.task.started"
    assert attributes.MYAPP_TASK_ID_KEY == "myapp.task.id"
    assert attributes.MYAPP_TASK_STATE_KEY == "myapp.task.state"
    assert attributes.AUTH_SUCCESS_KEY == "auth.success"
    assert attributes.MyappTaskState.QUEUED.value == "queued"
    assert attributes.AuthMethod.TOKEN.value == "token"


def test_histogram_records_required_and_optional_attributes() -> None:
    reader, provider = new_provider()
    duration = metrics.MyappTaskDuration(metrics.get_meter(provider))
    duration.record(0.25, task_id="task-1")
    duration.record(
        2.5,
        task_id="task-2",
        task_state=attributes.MyappTaskState.DONE,
        auth_method=attributes.AuthMethod.TOKEN,
    )
    metric = collect(reader)["myapp.task.duration"]
    assert metric.unit == "s"
    assert metric.description == "Time a task took from start to finish."
    points = sorted(histogram_points(metric), key=lambda point: point.sum)
    assert [attributes_of(point) for point in points] == [
        {"myapp.task.id": "task-1"},
        {
            "myapp.task.id": "task-2",
            "myapp.task.state": "done",
            "auth.method": "token",
        },
    ]
    assert [point.sum for point in points] == [0.25, 2.5]


def test_histogram_reports_the_registry_boundaries() -> None:
    reader, provider = new_provider()
    metrics.MyappTaskDuration(metrics.get_meter(provider)).record(0.5, task_id="a")
    (point,) = histogram_points(collect(reader)["myapp.task.duration"])
    assert point.explicit_bounds == (0.0, 0.005, 0.01, 0.1, 1.0, 10.0)
    assert point.bucket_counts == (0, 0, 0, 0, 1, 0, 0)


def test_integer_histogram_reports_its_own_boundaries() -> None:
    reader, provider = new_provider()
    metrics.MyappTaskPayload(metrics.get_meter(provider)).record(200)
    metric = collect(reader)["myapp.task.payload"]
    assert metric.unit == "By"
    (point,) = histogram_points(metric)
    assert point.explicit_bounds == (128.0, 256.0, 1024.0)


def test_counter_adds_attributes() -> None:
    reader, provider = new_provider()
    started = metrics.MyappTaskStarted(metrics.get_meter(provider))
    started.add(1)
    started.add(2, task_id="task-1")
    started.add(3, task_id="task-1")
    points = number_points(collect(reader)["myapp.task.started"])
    totals = {
        tuple(sorted(attributes_of(point).items())): point.value for point in points
    }
    assert totals == {(): 1, (("myapp.task.id", "task-1"),): 5}


def test_up_down_counter_takes_negative_amounts() -> None:
    reader, provider = new_provider()
    active = metrics.MyappTaskActive(metrics.get_meter(provider))
    active.add(3, task_state=attributes.MyappTaskState.QUEUED)
    active.add(-1, task_state=attributes.MyappTaskState.QUEUED)
    (point,) = number_points(collect(reader)["myapp.task.active"])
    assert point.value == 2
    assert attributes_of(point) == {"myapp.task.state": "queued"}


def test_gauge_keeps_the_last_value() -> None:
    reader, provider = new_provider()
    depth = metrics.MyappTaskFloatDepth(metrics.get_meter(provider))
    depth.set(1.5)
    depth.set(2.5)
    (point,) = number_points(collect(reader)["myapp.task.float.depth"])
    assert point.value == 2.5


def test_float_instruments_take_fractions() -> None:
    reader, provider = new_provider()
    meter = metrics.get_meter(provider)
    metrics.MyappTaskFloatStarted(meter).add(0.5)
    metrics.MyappTaskFloatActive(meter).add(-0.25)
    collected = collect(reader)
    assert number_points(collected["myapp.task.float.started"])[0].value == 0.5
    assert number_points(collected["myapp.task.float.active"])[0].value == -0.25


def test_boolean_and_enum_attributes() -> None:
    reader, provider = new_provider()
    attempts = metrics.AuthAttempts(metrics.get_meter(provider))
    attempts.add(1, method=attributes.AuthMethod.PASSWORD, success=True)
    attempts.add(1, method=attributes.AuthMethod.TOKEN, success=False)
    points = number_points(collect(reader)["auth.attempts"])
    assert sorted(
        (
            str(attributes_of(point)["auth.method"]),
            bool(attributes_of(point)["auth.success"]),
        )
        for point in points
    ) == [("password", True), ("token", False)]


def test_refinements_record_into_the_base_instrument() -> None:
    reader, provider = new_provider()
    meter = metrics.get_meter(provider)
    metrics.MyappTaskDuration(meter).record(1.0, task_id="base")
    metrics.MyappTaskRetryDuration(meter).record(
        2.0, task_id="retry", task_state=attributes.MyappTaskState.DONE
    )
    metrics.WorkerTaskDuration(meter).record(3.0, myapp_task_id="worker")
    assert metrics.MyappTaskRetryDuration.NAME == "myapp.task.duration"
    assert metrics.WorkerTaskDuration.NAME == "myapp.task.duration"
    metric = collect(reader)["myapp.task.duration"]
    ids = sorted(
        str(attributes_of(point)["myapp.task.id"]) for point in histogram_points(metric)
    )
    assert ids == ["base", "retry", "worker"]


def test_observable_instruments_report_through_callbacks() -> None:
    def started(_: CallbackOptions) -> Iterable[Observation]:
        yield metrics.MyappTaskStartedObservable.observation(7, task_id="task-1")

    def depth(_: CallbackOptions) -> Iterable[Observation]:
        yield metrics.MyappQueueDepthObservable.observation(4)

    def active(_: CallbackOptions) -> Iterable[Observation]:
        yield metrics.MyappTaskActiveObservable.observation(
            2, task_state=attributes.MyappTaskState.QUEUED
        )

    def float_depth(_: CallbackOptions) -> Iterable[Observation]:
        yield metrics.MyappTaskFloatDepthObservable.observation(1.5)

    reader, provider = new_provider()
    meter = metrics.get_meter(provider)
    metrics.MyappTaskStartedObservable([started], meter)
    metrics.MyappQueueDepthObservable([depth], meter)
    metrics.MyappTaskActiveObservable([active], meter)
    metrics.MyappTaskFloatDepthObservable([float_depth], meter)
    collected = collect(reader)
    (counter_point,) = number_points(collected["myapp.task.started"])
    assert counter_point.value == 7
    assert attributes_of(counter_point) == {"myapp.task.id": "task-1"}
    assert number_points(collected["myapp.queue.depth"])[0].value == 4
    assert number_points(collected["myapp.task.active"])[0].value == 2
    assert number_points(collected["myapp.task.float.depth"])[0].value == 1.5
    assert metrics.MyappTaskStartedObservable.NAME == metrics.MyappTaskStarted.NAME
    assert collected["myapp.task.started"].unit == "{task}"


def test_required_attributes_cannot_be_omitted() -> None:
    started = metrics.MyappTaskActive(metrics.get_meter(MeterProvider()))
    with pytest.raises(TypeError):
        started.add(1)  # type: ignore[call-arg]
    with pytest.raises(TypeError):
        started.add(1, "queued")  # type: ignore[arg-type, call-arg]
    with pytest.raises(AttributeError):
        started.add(1, task_state="queued")  # type: ignore[arg-type]
