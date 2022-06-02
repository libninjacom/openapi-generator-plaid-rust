/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorizationProposedTransfer : Details regarding the proposed transfer.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferAuthorizationProposedTransfer {
    #[serde(rename = "ach_class")]
    pub ach_class: crate::models::AchClass,
    /// The Plaid `account_id` for the account that will be debited or credited.
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "type")]
    pub _type: crate::models::TransferType,
    #[serde(rename = "user")]
    pub user: crate::models::TransferUserInResponse,
    /// The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "amount")]
    pub amount: String,
    /// The network or rails used for the transfer.
    #[serde(rename = "network")]
    pub network: String,
    /// Plaid's unique identifier for the origination account that was used for this transfer.
    #[serde(rename = "origination_account_id")]
    pub origination_account_id: String,
    /// The currency of the transfer amount. The default value is \"USD\".
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
}

impl TransferAuthorizationProposedTransfer {
    /// Details regarding the proposed transfer.
    pub fn new(ach_class: crate::models::AchClass, account_id: String, _type: crate::models::TransferType, user: crate::models::TransferUserInResponse, amount: String, network: String, origination_account_id: String, iso_currency_code: String) -> TransferAuthorizationProposedTransfer {
        TransferAuthorizationProposedTransfer {
            ach_class,
            account_id,
            _type,
            user,
            amount,
            network,
            origination_account_id,
            iso_currency_code,
        }
    }
}

