/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationPrecheckEmployer : Information about the end user's employer



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckEmployer {
    /// The employer's name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<crate::models::IncomeVerificationPrecheckEmployerAddress>>,
    /// The employer's tax id
    #[serde(rename = "tax_id", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// The URL for the employer's public website
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl IncomeVerificationPrecheckEmployer {
    /// Information about the end user's employer
    pub fn new() -> IncomeVerificationPrecheckEmployer {
        IncomeVerificationPrecheckEmployer {
            name: None,
            address: None,
            tax_id: None,
            url: None,
        }
    }
}


