/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RecipientBacs : An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RecipientBacs {
    /// The account number of the account. Maximum of 10 characters.
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The 6-character sort code of the account.
    #[serde(rename = "sort_code", skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}

impl RecipientBacs {
    /// An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub fn new() -> RecipientBacs {
        RecipientBacs {
            account: None,
            sort_code: None,
        }
    }
}


