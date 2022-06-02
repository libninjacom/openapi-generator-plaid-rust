/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaystubOverrideEmployer : The employer on the paystub.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaystubOverrideEmployer {
    /// The name of the employer.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PaystubOverrideEmployer {
    /// The employer on the paystub.
    pub fn new() -> PaystubOverrideEmployer {
        PaystubOverrideEmployer {
            name: None,
        }
    }
}


