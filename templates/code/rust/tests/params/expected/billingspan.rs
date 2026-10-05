#![allow(deprecated)]
use ::opentelemetry::trace::{TraceContextExt, Tracer};
use ::std::fmt::Write;

/// An invoice is being issued.
pub struct InvoiceIssueSpan<S: ::opentelemetry::trace::Span = ::opentelemetry::trace::noop::NoopSpan> { span: S }
impl<S: ::opentelemetry::trace::Span> InvoiceIssueSpan<S> {
    pub fn into_context(self, parent: &::opentelemetry::Context) -> ::opentelemetry::Context
    where S: Send + Sync + 'static,
    {
        parent.with_span(self.span)
    }
    pub fn span(&mut self) -> &mut S { &mut self.span }
    pub fn end(&mut self) { ::opentelemetry::trace::Span::end(&mut self.span); }
    pub fn end_with_timestamp(&mut self, timestamp: ::std::time::SystemTime) { ::opentelemetry::trace::Span::end_with_timestamp(&mut self.span, timestamp); }
    pub fn record_error(&mut self, error: &dyn ::std::error::Error) { ::opentelemetry::trace::Span::record_error(&mut self.span, error); }
    pub fn set_status(&mut self, status: ::opentelemetry::trace::Status) { ::opentelemetry::trace::Span::set_status(&mut self.span, status); }
}
impl Default for InvoiceIssueSpan {
    fn default() -> Self {
        Self { span: super::tracer::Tracer::default().inner().start("acme.billing.invoice.issue") }
    }
}

pub fn r#start_invoice_issue<T: ::opentelemetry::trace::Tracer>(context: &::opentelemetry::Context, tracer: &super::tracer::Tracer<T>, r#billing_invoice_id: super::billingattr::InvoiceIdAttr) -> InvoiceIssueSpan<T::Span> {
    let mut name = String::with_capacity("invoice ".len() + ::std::convert::AsRef::<str>::as_ref(&r#billing_invoice_id).len());
    name.push_str("invoice ");
    write!(&mut name, "{}", r#billing_invoice_id).expect("writing a span name to String cannot fail");
    let attributes: [::opentelemetry::KeyValue; 1] = [::opentelemetry::KeyValue::from(r#billing_invoice_id)];
    let span = tracer.inner().span_builder(name)
        .with_kind(::opentelemetry::trace::SpanKind::Internal)
        .with_attributes(attributes)
        .start_with_context(tracer.inner(), context);
    InvoiceIssueSpan { span }
}
