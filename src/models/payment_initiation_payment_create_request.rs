/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationPaymentCreateRequest : PaymentInitiationPaymentCreateRequest defines the request schema for `/payment_initiation/payment/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID of the recipient the payment is for.
    #[serde(rename = "recipient_id")]
    pub recipient_id: String,
    /// A reference for the payment. This must be an alphanumeric string with at most 18 characters and must not contain any special characters (since not all institutions support them).
    #[serde(rename = "reference")]
    pub reference: String,
    #[serde(rename = "amount")]
    pub amount: crate::models::PaymentAmount,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::models::ExternalPaymentScheduleRequest>>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<crate::models::ExternalPaymentOptions>,
}

impl PaymentInitiationPaymentCreateRequest {
    /// PaymentInitiationPaymentCreateRequest defines the request schema for `/payment_initiation/payment/create`
    pub fn new(recipient_id: String, reference: String, amount: crate::models::PaymentAmount) -> PaymentInitiationPaymentCreateRequest {
        PaymentInitiationPaymentCreateRequest {
            client_id: None,
            secret: None,
            recipient_id,
            reference,
            amount,
            schedule: None,
            options: None,
        }
    }
}


