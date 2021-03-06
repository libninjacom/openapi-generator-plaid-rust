/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationRecipientGetResponse : PaymentInitiationRecipientGetResponse defines the response schema for `/payment_initiation/recipient/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientGetResponse {
    /// The ID of the recipient.
    #[serde(rename = "recipient_id")]
    pub recipient_id: String,
    /// The name of the recipient.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::models::PaymentInitiationAddress>,
    /// The International Bank Account Number (IBAN) for the recipient.
    #[serde(rename = "iban", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "bacs", skip_serializing_if = "Option::is_none")]
    pub bacs: Option<Box<crate::models::RecipientBacsNullable>>,
    /// The EMI (E-Money Institution) recipient that this recipient is associated with, if any. This EMI recipient is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    #[serde(rename = "emi_recipient_id", skip_serializing_if = "Option::is_none")]
    pub emi_recipient_id: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl PaymentInitiationRecipientGetResponse {
    /// PaymentInitiationRecipientGetResponse defines the response schema for `/payment_initiation/recipient/get`
    pub fn new(recipient_id: String, name: String, request_id: String) -> PaymentInitiationRecipientGetResponse {
        PaymentInitiationRecipientGetResponse {
            recipient_id,
            name,
            address: None,
            iban: None,
            bacs: None,
            emi_recipient_id: None,
            request_id,
        }
    }
}


