#![allow(deprecated)]
#[derive(Clone, Debug)]
pub enum BillingInvoiceAmountAttr {
    Currency(super::attribute::BillingCurrency),
    RequestUrl(super::attribute::MyappRequestUrl),
}

impl From<BillingInvoiceAmountAttr> for ::opentelemetry::KeyValue {
    fn from(value: BillingInvoiceAmountAttr) -> Self {
        match value {
            BillingInvoiceAmountAttr::Currency(value) => ::opentelemetry::KeyValue::from(value),
            BillingInvoiceAmountAttr::RequestUrl(value) => ::opentelemetry::KeyValue::from(value),
        }
    }
}
/// Amount of each issued invoice.
#[derive(Clone)]
pub struct BillingInvoiceAmount {
    instrument: ::opentelemetry::metrics::Histogram<f64>,
}

impl BillingInvoiceAmount {
    pub const NAME: &'static str = "acme.billing.invoice.amount";
    pub const UNIT: &'static str = "{currency_unit}";
    pub const DESCRIPTION: &'static str = "Amount of each issued invoice.";

    pub fn new(meter: &super::meter::Meter) -> Self {
        let instrument = meter.inner().f64_histogram(Self::NAME)
            .with_description(Self::DESCRIPTION)
            .with_unit(Self::UNIT)
            .build();
        Self { instrument }
    }
    pub fn record(&self, value: f64, r#billing_invoice_id: super::attribute::BillingInvoiceId, options: impl IntoIterator<Item = BillingInvoiceAmountAttr>) {
        let mut attributes = ::smallvec::SmallVec::<[::opentelemetry::KeyValue; 3]>::new();
        attributes.extend([::opentelemetry::KeyValue::from(r#billing_invoice_id)]);
        attributes.extend(options.into_iter().map(::opentelemetry::KeyValue::from));
        self.instrument.record(value, &attributes);
    }
}

impl Default for BillingInvoiceAmount {
    fn default() -> Self { Self::new(&super::meter::Meter::default()) }
}
