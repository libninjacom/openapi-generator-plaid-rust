/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EmploymentVerificationStatus : Current employment status.

/// Current employment status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmploymentVerificationStatus {
    #[serde(rename = "EMPLOYMENT_STATUS_ACTIVE")]
    EMPLOYMENTSTATUSACTIVE,
    #[serde(rename = "EMPLOYMENT_STATUS_INACTIVE")]
    EMPLOYMENTSTATUSINACTIVE,
    #[serde(rename = "null")]
    Null,

}

impl ToString for EmploymentVerificationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::EMPLOYMENTSTATUSACTIVE => String::from("EMPLOYMENT_STATUS_ACTIVE"),
            Self::EMPLOYMENTSTATUSINACTIVE => String::from("EMPLOYMENT_STATUS_INACTIVE"),
            Self::Null => String::from("null"),
        }
    }
}

impl Default for EmploymentVerificationStatus {
    fn default() -> EmploymentVerificationStatus {
        Self::EMPLOYMENTSTATUSACTIVE
    }
}




