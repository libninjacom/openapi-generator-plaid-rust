/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Pay : An object representing a monetary amount.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Pay {
    /// A numerical amount of a specific currency.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Currency code, e.g. USD
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl Pay {
    /// An object representing a monetary amount.
    pub fn new() -> Pay {
        Pay {
            amount: None,
            currency: None,
        }
    }
}


