#![allow(deprecated)]
use ::opentelemetry::logs::LogRecord;
use ::opentelemetry::trace::TraceContextExt;

/// An invoice was issued.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BillingInvoiceIssued;

impl BillingInvoiceIssued {
    pub const NAME: &'static str = "acme.billing.invoice.issued";
}

#[derive(Clone, Debug)]
pub enum BillingInvoiceIssuedOption {
    Severity(::opentelemetry::logs::Severity),
    Timestamp(::std::time::SystemTime),
}
/// An invoice was issued.
pub fn r#emit_billing_invoice_issued<L: ::opentelemetry::logs::Logger>(context: &::opentelemetry::Context, logger: &super::logger::Logger<L>, r#billing_invoice_id: super::attribute::BillingInvoiceId, options: impl IntoIterator<Item = BillingInvoiceIssuedOption>) {
    let mut severity = ::opentelemetry::logs::Severity::Info;
    let mut timestamp = None;
    for option in options {
        match option {
            BillingInvoiceIssuedOption::Severity(value) => severity = value,
            BillingInvoiceIssuedOption::Timestamp(value) => timestamp = Some(value),
        }
    }
    if !logger.inner().event_enabled(severity, "", Some(BillingInvoiceIssued::NAME)) { return; }
    let _context_guard = context.clone().attach();
    let mut record = logger.inner().create_log_record();
    record.set_event_name(BillingInvoiceIssued::NAME);
    record.set_severity_number(severity);
    record.set_observed_timestamp(::std::time::SystemTime::now());
    if let Some(timestamp) = timestamp { record.set_timestamp(timestamp); }
    let span = context.span();
    let span_context = span.span_context();
    if span_context.is_valid() { record.set_trace_context(span_context.trace_id(), span_context.span_id(), Some(span_context.trace_flags())); }
    record.add_attribute(super::attribute::BillingInvoiceId::KEY, r#billing_invoice_id);
    logger.inner().emit(record);
}
