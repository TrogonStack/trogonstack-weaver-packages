#![allow(deprecated)]
use ::opentelemetry::logs::LogRecord;
use ::opentelemetry::trace::TraceContextExt;

/// A task reached a final state.
///
/// Emitted once per task, after its last attempt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MyappTaskFinished;

impl MyappTaskFinished {
    pub const NAME: &'static str = "myapp.task.finished";
}

#[derive(Clone, Debug)]
pub enum MyappTaskFinishedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
    BoolValues(super::attribute::MyappBoolValues),
    DoubleValues(super::attribute::MyappDoubleValues),
    IntValues(super::attribute::MyappIntValues),
    StringValues(super::attribute::MyappStringValues),
    /// Recommended: When the task ran more than once.
    TaskAttempt(super::attribute::MyappTaskAttempt),
    /// Opt-in: the convention records it only when a user asks for it.
    HostName(super::attribute::WorkerHostName),
}
/// A task reached a final state.
///
/// Emitted once per task, after its last attempt.
pub fn r#emit_myapp_task_finished<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#myapp_task_state: super::attribute::MyappTaskState, options: impl IntoIterator<Item = MyappTaskFinishedOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes = ::smallvec::SmallVec::<[MyappTaskFinishedOption; 6]>::new();
    for option in options {
        match option {
            MyappTaskFinishedOption::Severity(value) => severity = value,
            MyappTaskFinishedOption::Timestamp(value) => timestamp = Some(value),
            attribute => attributes.push(attribute),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some(MyappTaskFinished::NAME)) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name(MyappTaskFinished::NAME);
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    record.add_attribute(super::attribute::MyappTaskState::KEY, r#myapp_task_state);
    for option in attributes {
        match option {
            MyappTaskFinishedOption::BoolValues(value) => record.add_attribute(super::attribute::MyappBoolValues::KEY, value),
            MyappTaskFinishedOption::DoubleValues(value) => record.add_attribute(super::attribute::MyappDoubleValues::KEY, value),
            MyappTaskFinishedOption::IntValues(value) => record.add_attribute(super::attribute::MyappIntValues::KEY, value),
            MyappTaskFinishedOption::StringValues(value) => record.add_attribute(super::attribute::MyappStringValues::KEY, value),
            MyappTaskFinishedOption::TaskAttempt(value) => record.add_attribute(super::attribute::MyappTaskAttempt::KEY, value),
            MyappTaskFinishedOption::HostName(value) => record.add_attribute(super::attribute::WorkerHostName::KEY, value),
            MyappTaskFinishedOption::Severity(_) | MyappTaskFinishedOption::Timestamp(_) => continue,
        }
    }
    logger.inner().emit(record);
}
/// A task is about to run again.
/// Opt-in: record only when the user requests it.
#[deprecated(note = "Read the attempt from `myapp.task.finished` instead.")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MyappTaskRetried;

impl MyappTaskRetried {
    pub const NAME: &'static str = "myapp.task.retried";
}

#[derive(Clone, Debug)]
pub enum MyappTaskRetriedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
}
/// A task is about to run again.
/// Opt-in: record only when the user requests it.
#[deprecated(note = "Read the attempt from `myapp.task.finished` instead.")]
pub fn r#emit_myapp_task_retried<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#myapp_task_attempt: super::attribute::MyappTaskAttempt, options: impl IntoIterator<Item = MyappTaskRetriedOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    for option in options {
        match option {
            MyappTaskRetriedOption::Severity(value) => severity = value,
            MyappTaskRetriedOption::Timestamp(value) => timestamp = Some(value),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some(MyappTaskRetried::NAME)) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name(MyappTaskRetried::NAME);
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    record.add_attribute(super::attribute::MyappTaskAttempt::KEY, r#myapp_task_attempt);
    logger.inner().emit(record);
}
/// A worker host started taking tasks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WorkerHostStarted;

impl WorkerHostStarted {
    pub const NAME: &'static str = "worker.host.started";
}

#[derive(Clone, Debug)]
pub enum WorkerHostStartedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
}
/// A worker host started taking tasks.
pub fn r#emit_worker_host_started<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, options: impl IntoIterator<Item = WorkerHostStartedOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    for option in options {
        match option {
            WorkerHostStartedOption::Severity(value) => severity = value,
            WorkerHostStartedOption::Timestamp(value) => timestamp = Some(value),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some(WorkerHostStarted::NAME)) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name(WorkerHostStarted::NAME);
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    logger.inner().emit(record);
}
/// A task reached a final state on a worker.
///
/// Emitted once per task, after its last attempt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WorkerTaskFinished;

impl WorkerTaskFinished {
    pub const NAME: &'static str = "worker.task.finished";
}

#[derive(Clone, Debug)]
pub enum WorkerTaskFinishedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
    BoolValues(super::attribute::MyappBoolValues),
    DoubleValues(super::attribute::MyappDoubleValues),
    IntValues(super::attribute::MyappIntValues),
    StringValues(super::attribute::MyappStringValues),
    /// Recommended: When the task ran more than once.
    TaskAttempt(super::attribute::MyappTaskAttempt),
}
/// A task reached a final state on a worker.
///
/// Emitted once per task, after its last attempt.
pub fn r#emit_worker_task_finished<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#myapp_task_state: super::attribute::MyappTaskState, r#worker_host_name: super::attribute::WorkerHostName, options: impl IntoIterator<Item = WorkerTaskFinishedOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    let mut attributes = ::smallvec::SmallVec::<[WorkerTaskFinishedOption; 5]>::new();
    for option in options {
        match option {
            WorkerTaskFinishedOption::Severity(value) => severity = value,
            WorkerTaskFinishedOption::Timestamp(value) => timestamp = Some(value),
            attribute => attributes.push(attribute),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some(WorkerTaskFinished::NAME)) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name(WorkerTaskFinished::NAME);
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    record.add_attribute(super::attribute::MyappTaskState::KEY, r#myapp_task_state);
    record.add_attribute(super::attribute::WorkerHostName::KEY, r#worker_host_name);
    for option in attributes {
        match option {
            WorkerTaskFinishedOption::BoolValues(value) => record.add_attribute(super::attribute::MyappBoolValues::KEY, value),
            WorkerTaskFinishedOption::DoubleValues(value) => record.add_attribute(super::attribute::MyappDoubleValues::KEY, value),
            WorkerTaskFinishedOption::IntValues(value) => record.add_attribute(super::attribute::MyappIntValues::KEY, value),
            WorkerTaskFinishedOption::StringValues(value) => record.add_attribute(super::attribute::MyappStringValues::KEY, value),
            WorkerTaskFinishedOption::TaskAttempt(value) => record.add_attribute(super::attribute::MyappTaskAttempt::KEY, value),
            WorkerTaskFinishedOption::Severity(_) | WorkerTaskFinishedOption::Timestamp(_) => continue,
        }
    }
    logger.inner().emit(record);
}
