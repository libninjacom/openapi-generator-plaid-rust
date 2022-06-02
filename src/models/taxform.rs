/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Taxform : Data about an official document used to report the user's income to the IRS.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Taxform {
    /// An identifier of the document referenced by the document metadata.
    #[serde(rename = "doc_id", skip_serializing_if = "Option::is_none")]
    pub doc_id: Option<String>,
    /// The type of tax document. Currently, the only supported value is `w2`.
    #[serde(rename = "document_type")]
    pub document_type: String,
    #[serde(rename = "w2", skip_serializing_if = "Option::is_none")]
    pub w2: Option<crate::models::W2>,
}

impl Taxform {
    /// Data about an official document used to report the user's income to the IRS.
    pub fn new(document_type: String) -> Taxform {
        Taxform {
            doc_id: None,
            document_type,
            w2: None,
        }
    }
}

