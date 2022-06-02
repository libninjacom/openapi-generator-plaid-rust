/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferEventSyncResponse : Defines the response schema for `/transfer/event/sync`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferEventSyncResponse {
    #[serde(rename = "transfer_events")]
    pub transfer_events: Vec<crate::models::TransferEvent>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferEventSyncResponse {
    /// Defines the response schema for `/transfer/event/sync`
    pub fn new(transfer_events: Vec<crate::models::TransferEvent>, request_id: String) -> TransferEventSyncResponse {
        TransferEventSyncResponse {
            transfer_events,
            request_id,
        }
    }
}


