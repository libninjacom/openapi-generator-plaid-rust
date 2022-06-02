/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentAmount : The amount and currency of a payment



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentAmount {
    /// The ISO-4217 currency code of the payment. For standing orders, `\"GBP\"` must be used.
    #[serde(rename = "currency")]
    pub currency: Currency,
    /// The amount of the payment. Must contain at most two digits of precision e.g. `1.23`. Minimum accepted value is `1`.
    #[serde(rename = "value")]
    pub value: f32,
}

impl PaymentAmount {
    /// The amount and currency of a payment
    pub fn new(currency: Currency, value: f32) -> PaymentAmount {
        PaymentAmount {
            currency,
            value,
        }
    }
}

/// The ISO-4217 currency code of the payment. For standing orders, `\"GBP\"` must be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "GBP")]
    GBP,
    #[serde(rename = "EUR")]
    EUR,
}

impl Default for Currency {
    fn default() -> Currency {
        Self::GBP
    }
}
