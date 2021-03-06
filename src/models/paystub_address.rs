/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaystubAddress : Address on the paystub



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaystubAddress {
    /// The full city name.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The ISO 3166-1 alpha-2 country code.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The postal code of the address.
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The region or state Example: `\"NC\"`
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The full street address.
    #[serde(rename = "street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// Street address line 1.
    #[serde(rename = "line1", skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Street address line 2.
    #[serde(rename = "line2", skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// The region or state Example: `\"NC\"`
    #[serde(rename = "state_code", skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
}

impl PaystubAddress {
    /// Address on the paystub
    pub fn new() -> PaystubAddress {
        PaystubAddress {
            city: None,
            country: None,
            postal_code: None,
            region: None,
            street: None,
            line1: None,
            line2: None,
            state_code: None,
        }
    }
}


