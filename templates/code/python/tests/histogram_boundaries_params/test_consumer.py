from opentelemetry.sdk.metrics import MeterProvider
from opentelemetry.sdk.metrics.export import (
    HistogramDataPoint,
    InMemoryMetricReader,
    Metric,
    NumberDataPoint,
)
from opentelemetry.util.types import AttributeValue

from semconv import INSTRUMENTATION_NAME, INSTRUMENTATION_VERSION, SCHEMA_URL, metrics


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


SDK_DEFAULT_BOUNDARIES = (
    0.0,
    5.0,
    10.0,
    25.0,
    50.0,
    75.0,
    100.0,
    250.0,
    500.0,
    750.0,
    1000.0,
    2500.0,
    5000.0,
    7500.0,
    10000.0,
)
DURATION_BOUNDARIES: tuple[float, ...] | None = (0.5, 1.5)
QUEUE_WAIT_BOUNDARIES: tuple[float, ...] | None = (1.0, 5.0, 25.0, 100.0)
PAYLOAD_BOUNDARIES: tuple[float, ...] | None = None
STEP_BOUNDARIES: tuple[float, ...] | None = (0.0, 0.5, 1.5, 3.0)


def advertised(boundaries: tuple[float, ...] | None) -> tuple[float, ...]:
    return SDK_DEFAULT_BOUNDARIES if boundaries is None else boundaries


def test_the_constants_follow_the_unit_and_the_annotation() -> None:
    assert metrics.MyappTaskDuration.BUCKET_BOUNDARIES == DURATION_BOUNDARIES
    assert metrics.MyappTaskQueueWait.BUCKET_BOUNDARIES == QUEUE_WAIT_BOUNDARIES
    assert metrics.MyappTaskPayload.BUCKET_BOUNDARIES == PAYLOAD_BOUNDARIES
    assert metrics.MyappTaskStep.BUCKET_BOUNDARIES == STEP_BOUNDARIES


def test_a_counter_has_no_bucket_boundaries() -> None:
    assert not hasattr(metrics.MyappTaskRetries, "BUCKET_BOUNDARIES")
    assert not hasattr(metrics.MyappTaskRetriesObservable, "BUCKET_BOUNDARIES")


def test_the_sdk_reports_the_advertised_boundaries() -> None:
    reader, provider = new_provider()
    meter = metrics.get_meter(provider)
    metrics.MyappTaskDuration(meter).record(0.4)
    metrics.MyappTaskQueueWait(meter).record(3.0)
    metrics.MyappTaskPayload(meter).record(512)
    metrics.MyappTaskStep(meter).record(1.0)
    collected = collect(reader)
    expected = {
        "myapp.task.duration": DURATION_BOUNDARIES,
        "myapp.task.queue_wait": QUEUE_WAIT_BOUNDARIES,
        "myapp.task.payload": PAYLOAD_BOUNDARIES,
        "myapp.task.step": STEP_BOUNDARIES,
    }
    for name, boundaries in expected.items():
        (point,) = histogram_points(collected[name])
        assert tuple(point.explicit_bounds) == advertised(boundaries), name
        assert sum(point.bucket_counts) == 1
        assert len(point.bucket_counts) == len(point.explicit_bounds) + 1


def test_a_value_lands_in_the_bucket_of_the_advertised_boundaries() -> None:
    reader, provider = new_provider()
    metrics.MyappTaskStep(metrics.get_meter(provider)).record(1.0)
    (point,) = histogram_points(collect(reader)["myapp.task.step"])
    assert tuple(point.explicit_bounds) == (0.0, 0.5, 1.5, 3.0)
    assert tuple(point.bucket_counts) == (0, 0, 1, 0, 0)


def test_the_annotation_overrides_the_unit_default() -> None:
    assert metrics.MyappTaskStep.UNIT == "s"
    assert metrics.MyappTaskStep.BUCKET_BOUNDARIES == (0.0, 0.5, 1.5, 3.0)
