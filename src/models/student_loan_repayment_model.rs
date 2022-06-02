/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StudentLoanRepaymentModel : Student loan repayment information used to configure Sandbox test data for the Liabilities product



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StudentLoanRepaymentModel {
    /// The only currently supported value for this field is `standard`.
    #[serde(rename = "type")]
    pub _type: String,
    /// Configures the number of months before repayment starts.
    #[serde(rename = "non_repayment_months")]
    pub non_repayment_months: f32,
    /// Configures the number of months of repayments before the loan is paid off.
    #[serde(rename = "repayment_months")]
    pub repayment_months: f32,
}

impl StudentLoanRepaymentModel {
    /// Student loan repayment information used to configure Sandbox test data for the Liabilities product
    pub fn new(_type: String, non_repayment_months: f32, repayment_months: f32) -> StudentLoanRepaymentModel {
        StudentLoanRepaymentModel {
            _type,
            non_repayment_months,
            repayment_months,
        }
    }
}


