import importlib

import pytest

from semconv import INSTRUMENTATION_NAME, INSTRUMENTATION_VERSION, SCHEMA_URL


def test_only_the_scope_constants_are_generated() -> None:
    assert SCHEMA_URL == "https://example.com/schemas/empty/1.0.0"
    assert INSTRUMENTATION_NAME == "generated-semconv"
    assert INSTRUMENTATION_VERSION == ""


def test_no_metric_or_attribute_module_is_generated() -> None:
    for module in ("attributes", "metrics"):
        with pytest.raises(ModuleNotFoundError):
            importlib.import_module(f"semconv.{module}")
