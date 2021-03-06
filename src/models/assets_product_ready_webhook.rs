/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetsProductReadyWebhook : Fired when the Asset Report has been generated and `/asset_report/get` is ready to be called.  If you attempt to retrieve an Asset Report before this webhook has fired, you’ll receive a response with the HTTP status code 400 and a Plaid error code of `PRODUCT_NOT_READY`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetsProductReadyWebhook {
    /// `ASSETS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `PRODUCT_READY`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `asset_report_id` that can be provided to `/asset_report/get` to retrieve the Asset Report.
    #[serde(rename = "asset_report_id")]
    pub asset_report_id: String,
}

impl AssetsProductReadyWebhook {
    /// Fired when the Asset Report has been generated and `/asset_report/get` is ready to be called.  If you attempt to retrieve an Asset Report before this webhook has fired, you’ll receive a response with the HTTP status code 400 and a Plaid error code of `PRODUCT_NOT_READY`.
    pub fn new(webhook_type: String, webhook_code: String, asset_report_id: String) -> AssetsProductReadyWebhook {
        AssetsProductReadyWebhook {
            webhook_type,
            webhook_code,
            asset_report_id,
        }
    }
}


