/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InstitutionsSearchPaymentInitiationOptions : Additional options that will be used to filter institutions by various Payment Initiation configurations.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstitutionsSearchPaymentInitiationOptions {
    /// A unique ID identifying the payment
    #[serde(rename = "payment_id", skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
}

impl InstitutionsSearchPaymentInitiationOptions {
    /// Additional options that will be used to filter institutions by various Payment Initiation configurations.
    pub fn new() -> InstitutionsSearchPaymentInitiationOptions {
        InstitutionsSearchPaymentInitiationOptions {
            payment_id: None,
        }
    }
}


