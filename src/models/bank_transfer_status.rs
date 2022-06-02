/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferStatus : The status of the transfer.

/// The status of the transfer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankTransferStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "reversed")]
    Reversed,

}

impl ToString for BankTransferStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Pending => String::from("pending"),
            Self::Posted => String::from("posted"),
            Self::Cancelled => String::from("cancelled"),
            Self::Failed => String::from("failed"),
            Self::Reversed => String::from("reversed"),
        }
    }
}

impl Default for BankTransferStatus {
    fn default() -> BankTransferStatus {
        Self::Pending
    }
}




