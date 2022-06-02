/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportCreateResponse : AssetReportCreateResponse defines the response schema for `/asset_report/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetReportCreateResponse {
    /// A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    #[serde(rename = "asset_report_token")]
    pub asset_report_token: String,
    /// A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    #[serde(rename = "asset_report_id")]
    pub asset_report_id: String,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl AssetReportCreateResponse {
    /// AssetReportCreateResponse defines the response schema for `/asset_report/create`
    pub fn new(asset_report_token: String, asset_report_id: String, request_id: String) -> AssetReportCreateResponse {
        AssetReportCreateResponse {
            asset_report_token,
            asset_report_id,
            request_id,
        }
    }
}


