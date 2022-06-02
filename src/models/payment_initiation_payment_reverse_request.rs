/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationPaymentReverseRequest : PaymentInitiationPaymentReverseRequest defines the request schema for `/payment_initiation/payment/reverse`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The ID of the payment to reverse
    #[serde(rename = "payment_id")]
    pub payment_id: String,
}

impl PaymentInitiationPaymentReverseRequest {
    /// PaymentInitiationPaymentReverseRequest defines the request schema for `/payment_initiation/payment/reverse`
    pub fn new(payment_id: String) -> PaymentInitiationPaymentReverseRequest {
        PaymentInitiationPaymentReverseRequest {
            client_id: None,
            secret: None,
            payment_id,
        }
    }
}


