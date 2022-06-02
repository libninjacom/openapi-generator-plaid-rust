/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DepositSwitchCreateRequestOptions : Options to configure the `/deposit_switch/create` request. If provided, cannot be `null`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DepositSwitchCreateRequestOptions {
    /// The URL registered to receive webhooks when the status of a deposit switch request has changed. 
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
    /// An array of access tokens corresponding to transaction items to use when attempting to match the user to their Payroll Provider. These tokens must be created by the same client id as the one creating the switch, and have access to the transactions product.
    #[serde(rename = "transaction_item_access_tokens", skip_serializing_if = "Option::is_none")]
    pub transaction_item_access_tokens: Option<Vec<String>>,
}

impl DepositSwitchCreateRequestOptions {
    /// Options to configure the `/deposit_switch/create` request. If provided, cannot be `null`.
    pub fn new() -> DepositSwitchCreateRequestOptions {
        DepositSwitchCreateRequestOptions {
            webhook: None,
            transaction_item_access_tokens: None,
        }
    }
}


