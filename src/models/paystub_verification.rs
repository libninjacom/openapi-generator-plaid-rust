/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaystubVerification : An object containing details on the paystub's verification status. This object will only be populated if the [`income_verification.access_tokens`](/docs/api/tokens/#link-token-create-request-income-verification-access-tokens) parameter was provided during the `/link/token/create` call or if a problem was detected with the information supplied by the user; otherwise it will be `null`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaystubVerification {
    #[serde(rename = "verification_status")]
    pub verification_status: Option<crate::models::PaystubVerificationStatus>,
    #[serde(rename = "verification_attributes")]
    pub verification_attributes: Vec<crate::models::VerificationAttribute>,
}

impl PaystubVerification {
    /// An object containing details on the paystub's verification status. This object will only be populated if the [`income_verification.access_tokens`](/docs/api/tokens/#link-token-create-request-income-verification-access-tokens) parameter was provided during the `/link/token/create` call or if a problem was detected with the information supplied by the user; otherwise it will be `null`.
    pub fn new(verification_status: Option<crate::models::PaystubVerificationStatus>, verification_attributes: Vec<crate::models::VerificationAttribute>) -> PaystubVerification {
        PaystubVerification {
            verification_status,
            verification_attributes,
        }
    }
}


