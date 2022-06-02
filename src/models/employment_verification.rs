/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EmploymentVerification : An object containing proof of employment data for an individual



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EmploymentVerification {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::EmploymentVerificationStatus>,
    /// Start of employment in ISO 8601 format (YYYY-MM-DD).
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// End of employment, if applicable. Provided in ISO 8601 format (YYY-MM-DD).
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "employer", skip_serializing_if = "Option::is_none")]
    pub employer: Option<crate::models::EmployerVerification>,
    /// Current title of employee.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "platform_ids", skip_serializing_if = "Option::is_none")]
    pub platform_ids: Option<crate::models::PlatformIds>,
}

impl EmploymentVerification {
    /// An object containing proof of employment data for an individual
    pub fn new() -> EmploymentVerification {
        EmploymentVerification {
            status: None,
            start_date: None,
            end_date: None,
            employer: None,
            title: None,
            platform_ids: None,
        }
    }
}


