/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SandboxBankTransferFireWebhookResponse : Defines the response schema for `/sandbox/bank_transfer/fire_webhook`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SandboxBankTransferFireWebhookResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl SandboxBankTransferFireWebhookResponse {
    /// Defines the response schema for `/sandbox/bank_transfer/fire_webhook`
    pub fn new(request_id: String) -> SandboxBankTransferFireWebhookResponse {
        SandboxBankTransferFireWebhookResponse {
            request_id,
        }
    }
}


