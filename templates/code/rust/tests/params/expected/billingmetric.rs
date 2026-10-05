#![allow(deprecated)]








#[derive(Clone, Debug)]
pub enum InvoiceAmountHistogramAttr {


    Currency(super::billingattr::CurrencyAttr),


    RequestUrl(super::myappattr::RequestUrlAttr),

}
impl InvoiceAmountHistogramAttr {
    fn key_value(self) -> ::opentelemetry::KeyValue {
        match self {

            Self::Currency(value) => value.key_value(),

            Self::RequestUrl(value) => value.key_value(),

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
        let mut attributes = vec![r#billing_invoice_id.key_value()];

        attributes.extend(options.into_iter().map(InvoiceAmountHistogramAttr::key_value));

        self.instrument.record(value, &attributes);
    }
}
impl Default for InvoiceAmountHistogram {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
