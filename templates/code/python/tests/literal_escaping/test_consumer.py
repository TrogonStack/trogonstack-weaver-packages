import ast
import inspect
from types import ModuleType

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


def constant_docstring(module: ModuleType, constant: str) -> str | None:
    tree = ast.parse(inspect.getsource(module))
    statements = tree.body
    for index, statement in enumerate(statements):
        if (
            isinstance(statement, ast.AnnAssign)
            and isinstance(statement.target, ast.Name)
            and statement.target.id == constant
        ):
            following = statements[index + 1]
            assert isinstance(following, ast.Expr)
            docstring = ast.literal_eval(following.value)
            assert isinstance(docstring, str)
            return docstring
    return None


def test_enum_values_round_trip() -> None:
    assert attributes.DemoMode.QUOTED.value == 'it\'s "quoted"'
    assert attributes.DemoMode.PATH.value == "C:\\temp\\new"
    assert attributes.DemoMode.UNICODE.value == "caf\u00e9 \u2603"


def test_name_unit_and_description_round_trip() -> None:
    reader, provider = new_provider()
    metrics.DemoWork(metrics.get_meter(provider)).add(1)
    metric = collect(reader)["demo.work"]
    assert metric.unit == '{"unit"}'
    assert metric.description == 'Counts "quoted" work with a \\\\ backslash.'


def test_enum_values_reach_the_sdk_unchanged() -> None:
    reader, provider = new_provider()
    work = metrics.DemoWork(metrics.get_meter(provider))
    for mode in attributes.DemoMode:
        work.add(1, mode=mode)
    points = number_points(collect(reader)["demo.work"])
    assert sorted(str(attributes_of(point)["demo.mode"]) for point in points) == sorted(
        mode.value for mode in attributes.DemoMode
    )


def test_attribute_docstrings_are_escaped_and_normalized() -> None:
    assert constant_docstring(attributes, "DEMO_TEXT_KEY") == (
        'Says "hello" and \\\\ it\'s a backslash.\n\n'
        'Contains a triple quote """ and a backslash at the end \\\n'
        "on two lines, with trailing spaces\n"
        "and a tab    here."
    )


def test_class_and_method_docstrings_are_escaped() -> None:
    assert inspect.getdoc(metrics.DemoWork) == (
        'Counts "quoted" work with a \\\\ backslash.\n\nEnds with a backslash \\\\'
    )
    assert inspect.getdoc(metrics.DemoWork.add) == (
        "Add to the counter.\n\n"
        "Args:\n"
        '    text: Conditionally required: When it\'s set, as in "demo.mode" \\\\'
    )
