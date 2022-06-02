/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationWebhookStatus : Status of the income verification webhook



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncomeVerificationWebhookStatus {
    #[serde(rename = "id")]
    pub id: String,
}

impl IncomeVerificationWebhookStatus {
    /// Status of the income verification webhook
    pub fn new(id: String) -> IncomeVerificationWebhookStatus {
        IncomeVerificationWebhookStatus {
            id,
        }
    }
}


