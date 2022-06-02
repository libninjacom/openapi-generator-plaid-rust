/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferSweepStatus : The status of the sweep for the transfer. `unswept`: The transfer hasn't been swept yet. `swept`: The transfer was swept to the sweep account. `reverse_swept`: The transfer was reversed, funds were pulled back or pushed back to the sweep account. `null`: The transfer will never be swept (e.g. if the transfer is cancelled or reversed before being swept)

/// The status of the sweep for the transfer. `unswept`: The transfer hasn't been swept yet. `swept`: The transfer was swept to the sweep account. `reverse_swept`: The transfer was reversed, funds were pulled back or pushed back to the sweep account. `null`: The transfer will never be swept (e.g. if the transfer is cancelled or reversed before being swept)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferSweepStatus {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "unswept")]
    Unswept,
    #[serde(rename = "swept")]
    Swept,
    #[serde(rename = "reverse_swept")]
    ReverseSwept,

}

impl ToString for TransferSweepStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Null => String::from("null"),
            Self::Unswept => String::from("unswept"),
            Self::Swept => String::from("swept"),
            Self::ReverseSwept => String::from("reverse_swept"),
        }
    }
}

impl Default for TransferSweepStatus {
    fn default() -> TransferSweepStatus {
        Self::Null
    }
}




