/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationPaystubsGetResponse : IncomeVerificationPaystubsGetResponse defines the response schema for `/income/verification/paystubs/get`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncomeVerificationPaystubsGetResponse {
    /// Metadata for an income document.
    #[serde(rename = "document_metadata", skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<Vec<crate::models::DocumentMetadata>>,
    #[serde(rename = "paystubs")]
    pub paystubs: Vec<crate::models::Paystub>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::PlaidError>>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl IncomeVerificationPaystubsGetResponse {
    /// IncomeVerificationPaystubsGetResponse defines the response schema for `/income/verification/paystubs/get`.
    pub fn new(paystubs: Vec<crate::models::Paystub>, request_id: String) -> IncomeVerificationPaystubsGetResponse {
        IncomeVerificationPaystubsGetResponse {
            document_metadata: None,
            paystubs,
            error: None,
            request_id,
        }
    }
}


