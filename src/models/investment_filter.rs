/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentFilter : A filter to apply to `investment`-type accounts (or `brokerage`-type acconunts for API versions 2018-05-22 and earlier).



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvestmentFilter {
    /// An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    #[serde(rename = "account_subtypes")]
    pub account_subtypes: Vec<crate::models::InvestmentAccountSubtype>,
}

impl InvestmentFilter {
    /// A filter to apply to `investment`-type accounts (or `brokerage`-type acconunts for API versions 2018-05-22 and earlier).
    pub fn new(account_subtypes: Vec<crate::models::InvestmentAccountSubtype>) -> InvestmentFilter {
        InvestmentFilter {
            account_subtypes,
        }
    }
}


