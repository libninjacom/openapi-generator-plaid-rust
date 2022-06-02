/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferUserAddressInRequest : The address associated with the account holder.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferUserAddressInRequest {
    /// The street number and name (i.e., \"100 Market St.\").
    #[serde(rename = "street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// Ex. \"San Francisco\"
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The state or province (e.g., \"California\").
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The postal code (e.g., \"94103\").
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// A two-letter country code (e.g., \"US\").
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl TransferUserAddressInRequest {
    /// The address associated with the account holder.
    pub fn new() -> TransferUserAddressInRequest {
        TransferUserAddressInRequest {
            street: None,
            city: None,
            region: None,
            postal_code: None,
            country: None,
        }
    }
}


