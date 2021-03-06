/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MortgageInterestRate : Object containing metadata about the interest rate for the mortgage.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MortgageInterestRate {
    /// Percentage value (interest rate of current mortgage, not APR) of interest payable on a loan.
    #[serde(rename = "percentage")]
    pub percentage: Option<f32>,
    /// The type of interest charged (fixed or variable).
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

impl MortgageInterestRate {
    /// Object containing metadata about the interest rate for the mortgage.
    pub fn new(percentage: Option<f32>, _type: Option<String>) -> MortgageInterestRate {
        MortgageInterestRate {
            percentage,
            _type,
        }
    }
}


