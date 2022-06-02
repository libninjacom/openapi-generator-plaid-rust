/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PlatformIds : An object containing a set of ids related to an employee



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlatformIds {
    /// The ID of an employee as given by their employer
    #[serde(rename = "employee_id", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// The ID of an employee as given by their payroll
    #[serde(rename = "payroll_id", skip_serializing_if = "Option::is_none")]
    pub payroll_id: Option<String>,
    /// The ID of the position of the employee
    #[serde(rename = "position_id", skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
}

impl PlatformIds {
    /// An object containing a set of ids related to an employee
    pub fn new() -> PlatformIds {
        PlatformIds {
            employee_id: None,
            payroll_id: None,
            position_id: None,
        }
    }
}


