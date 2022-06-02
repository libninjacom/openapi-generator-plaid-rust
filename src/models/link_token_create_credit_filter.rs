/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateCreditFilter : A filter to apply to `credit`-type accounts



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkTokenCreateCreditFilter {
    /// An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    #[serde(rename = "account_subtypes", skip_serializing_if = "Option::is_none")]
    pub account_subtypes: Option<Vec<crate::models::CreditAccountSubtype>>,
}

impl LinkTokenCreateCreditFilter {
    /// A filter to apply to `credit`-type accounts
    pub fn new() -> LinkTokenCreateCreditFilter {
        LinkTokenCreateCreditFilter {
            account_subtypes: None,
        }
    }
}


