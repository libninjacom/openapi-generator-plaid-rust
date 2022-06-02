/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentAccountSubtype : Valid account subtypes for investment accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-investment).

/// Valid account subtypes for investment accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-investment).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvestmentAccountSubtype {
    #[serde(rename = "529")]
    _529,
    #[serde(rename = "401a")]
    _401a,
    #[serde(rename = "401k")]
    _401k,
    #[serde(rename = "403B")]
    _403B,
    #[serde(rename = "457b")]
    _457b,
    #[serde(rename = "brokerage")]
    Brokerage,
    #[serde(rename = "cash isa")]
    CashIsa,
    #[serde(rename = "education savings account")]
    EducationSavingsAccount,
    #[serde(rename = "fixed annuity")]
    FixedAnnuity,
    #[serde(rename = "gic")]
    Gic,
    #[serde(rename = "health reimbursement arrangement")]
    HealthReimbursementArrangement,
    #[serde(rename = "hsa")]
    Hsa,
    #[serde(rename = "ira")]
    Ira,
    #[serde(rename = "isa")]
    Isa,
    #[serde(rename = "keogh")]
    Keogh,
    #[serde(rename = "lif")]
    Lif,
    #[serde(rename = "life insurance")]
    LifeInsurance,
    #[serde(rename = "lira")]
    Lira,
    #[serde(rename = "lrif")]
    Lrif,
    #[serde(rename = "lrsp")]
    Lrsp,
    #[serde(rename = "mutual fund")]
    MutualFund,
    #[serde(rename = "non-taxable brokerage account")]
    NonTaxableBrokerageAccount,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "other annuity")]
    OtherAnnuity,
    #[serde(rename = "other insurance")]
    OtherInsurance,
    #[serde(rename = "person")]
    Person,
    #[serde(rename = "prif")]
    Prif,
    #[serde(rename = "profit sharing plan")]
    ProfitSharingPlan,
    #[serde(rename = "qshr")]
    Qshr,
    #[serde(rename = "rdsp")]
    Rdsp,
    #[serde(rename = "resp")]
    Resp,
    #[serde(rename = "retirement")]
    Retirement,
    #[serde(rename = "rlif")]
    Rlif,
    #[serde(rename = "roth")]
    Roth,
    #[serde(rename = "roth 401k")]
    Roth401k,
    #[serde(rename = "rrif")]
    Rrif,
    #[serde(rename = "rrsp")]
    Rrsp,
    #[serde(rename = "sarsep")]
    Sarsep,
    #[serde(rename = "sep ira")]
    SepIra,
    #[serde(rename = "simple ira")]
    SimpleIra,
    #[serde(rename = "sipp")]
    Sipp,
    #[serde(rename = "stock plan")]
    StockPlan,
    #[serde(rename = "tfsa")]
    Tfsa,
    #[serde(rename = "trust")]
    Trust,
    #[serde(rename = "ugma")]
    Ugma,
    #[serde(rename = "utma")]
    Utma,
    #[serde(rename = "variable annuity")]
    VariableAnnuity,
    #[serde(rename = "all")]
    All,

}

impl ToString for InvestmentAccountSubtype {
    fn to_string(&self) -> String {
        match self {
            Self::_529 => String::from("529"),
            Self::_401a => String::from("401a"),
            Self::_401k => String::from("401k"),
            Self::_403B => String::from("403B"),
            Self::_457b => String::from("457b"),
            Self::Brokerage => String::from("brokerage"),
            Self::CashIsa => String::from("cash isa"),
            Self::EducationSavingsAccount => String::from("education savings account"),
            Self::FixedAnnuity => String::from("fixed annuity"),
            Self::Gic => String::from("gic"),
            Self::HealthReimbursementArrangement => String::from("health reimbursement arrangement"),
            Self::Hsa => String::from("hsa"),
            Self::Ira => String::from("ira"),
            Self::Isa => String::from("isa"),
            Self::Keogh => String::from("keogh"),
            Self::Lif => String::from("lif"),
            Self::LifeInsurance => String::from("life insurance"),
            Self::Lira => String::from("lira"),
            Self::Lrif => String::from("lrif"),
            Self::Lrsp => String::from("lrsp"),
            Self::MutualFund => String::from("mutual fund"),
            Self::NonTaxableBrokerageAccount => String::from("non-taxable brokerage account"),
            Self::Other => String::from("other"),
            Self::OtherAnnuity => String::from("other annuity"),
            Self::OtherInsurance => String::from("other insurance"),
            Self::Person => String::from("person"),
            Self::Prif => String::from("prif"),
            Self::ProfitSharingPlan => String::from("profit sharing plan"),
            Self::Qshr => String::from("qshr"),
            Self::Rdsp => String::from("rdsp"),
            Self::Resp => String::from("resp"),
            Self::Retirement => String::from("retirement"),
            Self::Rlif => String::from("rlif"),
            Self::Roth => String::from("roth"),
            Self::Roth401k => String::from("roth 401k"),
            Self::Rrif => String::from("rrif"),
            Self::Rrsp => String::from("rrsp"),
            Self::Sarsep => String::from("sarsep"),
            Self::SepIra => String::from("sep ira"),
            Self::SimpleIra => String::from("simple ira"),
            Self::Sipp => String::from("sipp"),
            Self::StockPlan => String::from("stock plan"),
            Self::Tfsa => String::from("tfsa"),
            Self::Trust => String::from("trust"),
            Self::Ugma => String::from("ugma"),
            Self::Utma => String::from("utma"),
            Self::VariableAnnuity => String::from("variable annuity"),
            Self::All => String::from("all"),
        }
    }
}

impl Default for InvestmentAccountSubtype {
    fn default() -> InvestmentAccountSubtype {
        Self::_529
    }
}




