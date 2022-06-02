/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InstitutionsGetRequestOptions : An optional object to filter `/institutions/get` results.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstitutionsGetRequestOptions {
    /// Filter the Institutions based on which products they support. 
    #[serde(rename = "products", skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<crate::models::Products>>,
    /// Specify an array of routing numbers to filter institutions. The response will only return institutions that match all of the routing numbers in the array. Routing number records used for this matching are not comprehensive; failure to match a given routing number to an institution does not mean that the institution is unsupported by Plaid.
    #[serde(rename = "routing_numbers", skip_serializing_if = "Option::is_none")]
    pub routing_numbers: Option<Vec<String>>,
    /// Limit results to institutions with or without OAuth login flows.
    #[serde(rename = "oauth", skip_serializing_if = "Option::is_none")]
    pub oauth: Option<bool>,
    /// When `true`, return the institution's homepage URL, logo and primary brand color.  Note that Plaid does not own any of the logos shared by the API, and that by accessing or using these logos, you agree that you are doing so at your own risk and will, if necessary, obtain all required permissions from the appropriate rights holders and adhere to any applicable usage guidelines. Plaid disclaims all express or implied warranties with respect to the logos.
    #[serde(rename = "include_optional_metadata", skip_serializing_if = "Option::is_none")]
    pub include_optional_metadata: Option<bool>,
    /// When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    #[serde(rename = "include_auth_metadata", skip_serializing_if = "Option::is_none")]
    pub include_auth_metadata: Option<bool>,
    /// When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    #[serde(rename = "include_payment_initiation_metadata", skip_serializing_if = "Option::is_none")]
    pub include_payment_initiation_metadata: Option<bool>,
}

impl InstitutionsGetRequestOptions {
    /// An optional object to filter `/institutions/get` results.
    pub fn new() -> InstitutionsGetRequestOptions {
        InstitutionsGetRequestOptions {
            products: None,
            routing_numbers: None,
            oauth: None,
            include_optional_metadata: None,
            include_auth_metadata: None,
            include_payment_initiation_metadata: None,
        }
    }
}


