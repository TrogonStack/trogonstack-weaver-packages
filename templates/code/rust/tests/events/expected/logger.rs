use super::options::InstrumentationOptions;

#[derive(Clone, Debug)]
pub struct Logger<L: opentelemetry::logs::Logger = <opentelemetry::logs::NoopLoggerProvider as opentelemetry::logs::LoggerProvider>::Logger>(L);
impl<L: opentelemetry::logs::Logger> Logger<L> {
    pub fn new<P: opentelemetry::logs::LoggerProvider<Logger = L>>(provider: &P, options: InstrumentationOptions) -> Self {
        let logger = provider.logger_with_scope(options.scope.into());
        Self(logger)
    }
    pub fn inner(&self) -> &L { &self.0 }
}
impl Default for Logger {
    fn default() -> Self { Self::new(&opentelemetry::logs::NoopLoggerProvider::new(), InstrumentationOptions::default()) }
}
