import enum

from semconv import (
    INSTRUMENTATION_NAME,
    INSTRUMENTATION_VERSION,
    SCHEMA_URL,
    attributes,
)


def test_module_constants() -> None:
    assert SCHEMA_URL == "https://example.com/schemas/attributes/1.0.0"
    assert INSTRUMENTATION_NAME == "generated-semconv"
    assert INSTRUMENTATION_VERSION == ""


def test_attribute_keys_equal_the_registry() -> None:
    keys = {
        name: getattr(attributes, name)
        for name in dir(attributes)
        if name.endswith("_KEY")
    }
    assert keys == {
        "AUTH_LEVEL_KEY": "auth.level",
        "AUTH_METHOD_KEY": "auth.method",
        "AUTH_USER_ID_KEY": "auth.user.id",
        "MYAPP_TASK_ASSIGNED_BY_KEY": "myapp.task.assigned_by",
        "MYAPP_TASK_CANCELLED_KEY": "myapp.task.cancelled",
        "MYAPP_TASK_ESCAPED_KEY": "myapp.task.escaped",
        "MYAPP_TASK_FLAGS_KEY": "myapp.task.flags",
        "MYAPP_TASK_ID_KEY": "myapp.task.id",
        "MYAPP_TASK_LANE_KEY": "myapp.task.lane",
        "MYAPP_TASK_OWNER_KEY": "myapp.task.owner",
        "MYAPP_TASK_PREEMPTIBLE_KEY": "myapp.task.preemptible",
        "MYAPP_TASK_PROGRESS_KEY": "myapp.task.progress",
        "MYAPP_TASK_RETRIES_KEY": "myapp.task.retries",
        "MYAPP_TASK_SAMPLE_RATE_KEY": "myapp.task.sample_rate",
        "MYAPP_TASK_SHARD_IDS_KEY": "myapp.task.shard_ids",
        "MYAPP_TASK_STATE_KEY": "myapp.task.state",
        "MYAPP_TASK_TAGS_KEY": "myapp.task.tags",
        "MYAPP_TASK_WEIGHTS_KEY": "myapp.task.weights",
    }


def test_string_enum_members_are_strings() -> None:
    assert issubclass(attributes.MyappTaskState, enum.StrEnum)
    assert [member.value for member in attributes.MyappTaskState] == [
        "queued",
        "running",
        "paused",
    ]
    assert str(attributes.MyappTaskState.RUNNING) == "running"
    assert attributes.AuthMethod("token") is attributes.AuthMethod.TOKEN


def test_integer_enum_members_are_integers() -> None:
    assert issubclass(attributes.AuthLevel, enum.IntEnum)
    assert [member.value for member in attributes.AuthLevel] == [1, 2]
    assert attributes.AuthLevel.HIGH + 1 == 3


def test_boolean_enum_members_are_booleans() -> None:
    assert [member.value for member in attributes.MyappTaskPreemptible] == [True, False]
    assert attributes.MyappTaskPreemptible.ALLOWED.value is True
    assert attributes.MyappTaskPreemptible.FORBIDDEN.value is False


def test_double_enum_members_are_floats() -> None:
    assert issubclass(attributes.MyappTaskSampleRate, float)
    assert [member.value for member in attributes.MyappTaskSampleRate] == [0.1, 1.0]
    assert isinstance(attributes.MyappTaskSampleRate.ALL.value, float)


def test_deprecated_enum_members_are_still_generated() -> None:
    assert attributes.MyappTaskState.PAUSED.value == "paused"
    assert attributes.MyappTaskLane.SLOW.value == "slow"


def test_member_values_survive_escaping() -> None:
    assert (
        attributes.MyappTaskEscaped.QUOTED.value
        == 'quote" slash\\ newline\n tab\t backspace\b formfeed\f null\x00 snowman☃'
    )
