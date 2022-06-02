/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConnectedApplication : Describes the connected application for a particular end user.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConnectedApplication {
    /// This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    #[serde(rename = "application_id")]
    pub application_id: String,
    /// The name of the application
    #[serde(rename = "name")]
    pub name: String,
    /// A URL that links to the application logo image (will be deprecated in the future, please use logo_url).
    #[serde(rename = "logo")]
    pub logo: Option<String>,
    /// A URL that links to the application logo image.
    #[serde(rename = "logo_url")]
    pub logo_url: Option<String>,
    /// The URL for the application's website
    #[serde(rename = "application_url")]
    pub application_url: Option<String>,
    /// A string provided by the connected app stating why they use their respective enabled products.
    #[serde(rename = "reason_for_access")]
    pub reason_for_access: Option<String>,
    /// The date this application was linked in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date this application was granted production access at Plaid in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    #[serde(rename = "join_date")]
    pub join_date: String,
    /// (Deprecated) A list of enums representing the data collected and products enabled for this connected application.
    #[serde(rename = "product_data_types")]
    pub product_data_types: Vec<ProductDataTypes>,
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Box<crate::models::ScopesNullable>>,
    #[serde(rename = "requested_scopes", skip_serializing_if = "Option::is_none")]
    pub requested_scopes: Option<Box<crate::models::RequestedScopes>>,
}

impl ConnectedApplication {
    /// Describes the connected application for a particular end user.
    pub fn new(application_id: String, name: String, logo: Option<String>, logo_url: Option<String>, application_url: Option<String>, reason_for_access: Option<String>, created_at: String, join_date: String, product_data_types: ProductDataTypes) -> ConnectedApplication {
        ConnectedApplication {
            application_id,
            name,
            logo,
            logo_url,
            application_url,
            reason_for_access,
            created_at,
            join_date,
            product_data_types,
            scopes: None,
            requested_scopes: None,
        }
    }
}

/// (Deprecated) A list of enums representing the data collected and products enabled for this connected application.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProductDataTypes {
    #[serde(rename = "ACCOUNT_BALANCE")]
    BALANCE,
    #[serde(rename = "ACCOUNT_USER_INFO")]
    USERINFO,
    #[serde(rename = "ACCOUNT_TRANSACTIONS")]
    TRANSACTIONS,
}

impl Default for ProductDataTypes {
    fn default() -> ProductDataTypes {
        Self::BALANCE
    }
}
