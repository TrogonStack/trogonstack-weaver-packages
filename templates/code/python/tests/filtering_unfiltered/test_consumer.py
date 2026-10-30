import inspect

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


PRESENT = [
    "Duration",
    "LegacyDuration",
    "PreviewDuration",
    "Queued",
    "Retired",
    "RetryDuration",
    "Started",
]
ABSENT: list[str] = []
OBSERVABLES = ["Queued", "Retired", "Started"]
UNEXPECTED_OBSERVABLES: list[str] = []
CONSTANTS = ["ASSIGNEE", "ID", "OWNER", "PRIORITY", "STATE"]
ABSENT_CONSTANTS: list[str] = []
MEMBERS = ["QUEUED", "RUNNING", "PAUSED"]
ABSENT_MEMBERS: list[str] = []
STARTED_PARAMETERS = [
    "task_id",
    "task_assignee",
    "task_owner",
    "task_priority",
    "task_state",
]


def test_only_the_expected_metric_classes_exist() -> None:
    for suffix in PRESENT:
        assert hasattr(metrics, f"MyappTask{suffix}")
    for suffix in ABSENT:
        assert not hasattr(metrics, f"MyappTask{suffix}")
    for suffix in OBSERVABLES:
        assert hasattr(metrics, f"MyappTask{suffix}Observable")
    for suffix in UNEXPECTED_OBSERVABLES:
        assert not hasattr(metrics, f"MyappTask{suffix}")


def test_only_the_expected_attribute_constants_exist() -> None:
    for suffix in CONSTANTS:
        assert getattr(attributes, f"MYAPP_TASK_{suffix}_KEY") == (
            "myapp.task." + suffix.lower()
        )
    for suffix in ABSENT_CONSTANTS:
        assert not hasattr(attributes, f"MYAPP_TASK_{suffix}_KEY")


def test_only_the_expected_enum_members_exist() -> None:
    assert [member.name for member in attributes.MyappTaskState] == MEMBERS
    for member in ABSENT_MEMBERS:
        assert member not in attributes.MyappTaskState.__members__


def test_metric_signatures_follow_the_filters() -> None:
    parameters = inspect.signature(metrics.MyappTaskStarted.add).parameters
    assert [key for key in parameters if key not in ("self", "amount")] == (
        STARTED_PARAMETERS
    )
    assert parameters["task_id"].default is inspect.Parameter.empty
    for key in STARTED_PARAMETERS[1:]:
        assert parameters[key].default is None


def test_surviving_metrics_record() -> None:
    reader, provider = new_provider()
    meter = metrics.get_meter(provider)
    metrics.MyappTaskStarted(meter).add(2, task_id="task-1")
    metrics.MyappTaskDuration(meter).record(0.5, task_id="task-1")
    metrics.MyappTaskRetryDuration(meter).record(1.5, task_id="task-2")
    collected = collect(reader)
    (started,) = number_points(collected["myapp.task.started"])
    assert started.value == 2
    assert attributes_of(started) == {"myapp.task.id": "task-1"}
    points = histogram_points(collected["myapp.task.duration"])
    assert sorted(point.sum for point in points) == [0.5, 1.5]
