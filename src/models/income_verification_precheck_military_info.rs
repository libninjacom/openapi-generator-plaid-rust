/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationPrecheckMilitaryInfo : Data about military info in the income verification precheck.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckMilitaryInfo {
    /// Is the user currently active duty in the US military
    #[serde(rename = "is_active_duty", skip_serializing_if = "Option::is_none")]
    pub is_active_duty: Option<bool>,
    /// If the user is currently serving in the US military, the branch of the military they are serving in
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<Branch>,
}

impl IncomeVerificationPrecheckMilitaryInfo {
    /// Data about military info in the income verification precheck.
    pub fn new() -> IncomeVerificationPrecheckMilitaryInfo {
        IncomeVerificationPrecheckMilitaryInfo {
            is_active_duty: None,
            branch: None,
        }
    }
}

/// If the user is currently serving in the US military, the branch of the military they are serving in
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Branch {
    #[serde(rename = "AIR FORCE")]
    AIRFORCE,
    #[serde(rename = "ARMY")]
    ARMY,
    #[serde(rename = "COAST GUARD")]
    COASTGUARD,
    #[serde(rename = "MARINES")]
    MARINES,
    #[serde(rename = "NAVY")]
    NAVY,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}

impl Default for Branch {
    fn default() -> Branch {
        Self::AIRFORCE
    }
}

