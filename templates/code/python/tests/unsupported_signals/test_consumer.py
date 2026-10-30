import inspect
from pathlib import Path

from opentelemetry.sdk.metrics import MeterProvider
from opentelemetry.sdk.metrics.export import (
    HistogramDataPoint,
    InMemoryMetricReader,
    Metric,
    NumberDataPoint,
)
from opentelemetry.util.types import AttributeValue

import semconv
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


def test_only_the_three_modules_are_generated() -> None:
    package = Path(semconv.__file__).parent
    assert sorted(path.name for path in package.glob("*.py")) == [
        "__init__.py",
        "attributes.py",
        "metrics.py",
    ]


def test_only_metric_classes_are_defined() -> None:
    defined = {
        name
        for name, member in inspect.getmembers(metrics, inspect.isclass)
        if member.__module__ == metrics.__name__
    }
    assert defined == {
        "MyappTaskStarted",
        "MyappTaskStartedObservable",
    }


def test_no_span_event_or_entity_symbol_exists() -> None:
    for module in (semconv, attributes, metrics):
        for name in dir(module):
            lowered = name.lower()
            assert "span" not in lowered, name
            assert "event" not in lowered, name
            assert "entity" not in lowered, name
            assert "tracer" not in lowered, name


def test_attributes_used_only_by_other_signals_still_get_constants() -> None:
    assert attributes.MYAPP_QUEUE_NAME_KEY == "myapp.queue.name"
    assert attributes.MYAPP_HOST_NAME_KEY == "myapp.host.name"
    assert attributes.MYAPP_HOST_TYPE_KEY == "myapp.host.type"
    assert attributes.MyappTaskState.QUEUED.value == "queued"


def test_the_metric_still_records() -> None:
    reader, provider = new_provider()
    metrics.MyappTaskStarted(metrics.get_meter(provider)).add(3, task_id="task-1")
    (point,) = number_points(collect(reader)["myapp.task.started"])
    assert point.value == 3
    assert attributes_of(point) == {"myapp.task.id": "task-1"}
