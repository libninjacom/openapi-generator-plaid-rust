/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsRecurringGetResponse : TransactionsRecurringGetResponse defines the response schema for `/transactions/recurring/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsRecurringGetResponse {
    /// An array of depository transaction streams.
    #[serde(rename = "inflow_streams")]
    pub inflow_streams: Vec<crate::models::TransactionStream>,
    /// An array of expense transaction streams.
    #[serde(rename = "outflow_streams")]
    pub outflow_streams: Vec<crate::models::TransactionStream>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransactionsRecurringGetResponse {
    /// TransactionsRecurringGetResponse defines the response schema for `/transactions/recurring/get`
    pub fn new(inflow_streams: Vec<crate::models::TransactionStream>, outflow_streams: Vec<crate::models::TransactionStream>, request_id: String) -> TransactionsRecurringGetResponse {
        TransactionsRecurringGetResponse {
            inflow_streams,
            outflow_streams,
            request_id,
        }
    }
}


