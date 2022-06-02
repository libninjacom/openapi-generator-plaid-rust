/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferEventListResponse : Defines the response schema for `/bank_transfer/event/list`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BankTransferEventListResponse {
    #[serde(rename = "bank_transfer_events")]
    pub bank_transfer_events: Vec<crate::models::BankTransferEvent>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BankTransferEventListResponse {
    /// Defines the response schema for `/bank_transfer/event/list`
    pub fn new(bank_transfer_events: Vec<crate::models::BankTransferEvent>, request_id: String) -> BankTransferEventListResponse {
        BankTransferEventListResponse {
            bank_transfer_events,
            request_id,
        }
    }
}


