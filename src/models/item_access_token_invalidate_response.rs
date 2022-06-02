/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemAccessTokenInvalidateResponse : ItemAccessTokenInvalidateResponse defines the response schema for `/item/access_token/invalidate`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemAccessTokenInvalidateResponse {
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "new_access_token")]
    pub new_access_token: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl ItemAccessTokenInvalidateResponse {
    /// ItemAccessTokenInvalidateResponse defines the response schema for `/item/access_token/invalidate`
    pub fn new(new_access_token: String, request_id: String) -> ItemAccessTokenInvalidateResponse {
        ItemAccessTokenInvalidateResponse {
            new_access_token,
            request_id,
        }
    }
}

