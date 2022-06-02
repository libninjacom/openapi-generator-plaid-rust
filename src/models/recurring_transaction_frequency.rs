/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RecurringTransactionFrequency : describes the frequency of the transaction stream.

/// describes the frequency of the transaction stream.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecurringTransactionFrequency {
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "WEEKLY")]
    WEEKLY,
    #[serde(rename = "BIWEEKLY")]
    BIWEEKLY,
    #[serde(rename = "SEMI_MONTHLY")]
    SEMIMONTHLY,
    #[serde(rename = "MONTHLY")]
    MONTHLY,

}

impl ToString for RecurringTransactionFrequency {
    fn to_string(&self) -> String {
        match self {
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::WEEKLY => String::from("WEEKLY"),
            Self::BIWEEKLY => String::from("BIWEEKLY"),
            Self::SEMIMONTHLY => String::from("SEMI_MONTHLY"),
            Self::MONTHLY => String::from("MONTHLY"),
        }
    }
}

impl Default for RecurringTransactionFrequency {
    fn default() -> RecurringTransactionFrequency {
        Self::UNKNOWN
    }
}




