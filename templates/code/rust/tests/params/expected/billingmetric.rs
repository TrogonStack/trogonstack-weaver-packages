#![allow(deprecated)]
#[derive(Clone, Debug)]
pub enum InvoiceAmountHistogramAttr {
    Currency(super::billingattr::CurrencyAttr),
    RequestUrl(super::myappattr::RequestUrlAttr),
}
impl From<InvoiceAmountHistogramAttr> for ::opentelemetry::KeyValue {
    fn from(value: InvoiceAmountHistogramAttr) -> Self {
        match value {
            InvoiceAmountHistogramAttr::Currency(value) => ::opentelemetry::KeyValue::from(value),
            InvoiceAmountHistogramAttr::RequestUrl(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
/// Amount of each issued invoice.
#[derive(Clone)]
pub struct InvoiceAmountHistogram { instrument: ::opentelemetry::metrics::Histogram<f64> }
impl InvoiceAmountHistogram {
    pub fn new(meter: &super::meter::Meter) -> Self {
        Self { instrument: meter.inner().f64_histogram("acme.billing.invoice.amount")
            .with_description("Amount of each issued invoice.")
            .with_unit("{currency_unit}")
            .build() }
    }
    pub fn record(&self, value: f64, r#billing_invoice_id: super::billingattr::InvoiceIdAttr, options: impl IntoIterator<Item = InvoiceAmountHistogramAttr>) {
        let mut attributes = vec![::opentelemetry::KeyValue::from(r#billing_invoice_id)];
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.instrument.record(value, &attributes);
    }
}
impl Default for InvoiceAmountHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
