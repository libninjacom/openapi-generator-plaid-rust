/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferSweepListResponse : Defines the response schema for `/transfer/sweep/list`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferSweepListResponse {
    #[serde(rename = "sweeps")]
    pub sweeps: Vec<crate::models::TransferSweep>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferSweepListResponse {
    /// Defines the response schema for `/transfer/sweep/list`
    pub fn new(sweeps: Vec<crate::models::TransferSweep>, request_id: String) -> TransferSweepListResponse {
        TransferSweepListResponse {
            sweeps,
            request_id,
        }
    }
}


