/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentTransaction : A transaction within an investment account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvestmentTransaction {
    /// The ID of the Investment transaction, unique across all Plaid transactions. Like all Plaid identifiers, the `investment_transaction_id` is case sensitive.
    #[serde(rename = "investment_transaction_id")]
    pub investment_transaction_id: String,
    /// A legacy field formerly used internally by Plaid to identify certain canceled transactions.
    #[serde(rename = "cancel_transaction_id", skip_serializing_if = "Option::is_none")]
    pub cancel_transaction_id: Option<String>,
    /// The `account_id` of the account against which this transaction posted.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The `security_id` to which this transaction is related.
    #[serde(rename = "security_id")]
    pub security_id: Option<String>,
    /// The [ISO 8601](https://wikipedia.org/wiki/ISO_8601) posting date for the transaction.
    #[serde(rename = "date")]
    pub date: String,
    /// The institution’s description of the transaction.
    #[serde(rename = "name")]
    pub name: String,
    /// The number of units of the security involved in this transaction.
    #[serde(rename = "quantity")]
    pub quantity: f32,
    /// The complete value of the transaction. Positive values when cash is debited, e.g. purchases of stock; negative values when cash is credited, e.g. sales of stock. Treatment remains the same for cash-only movements unassociated with securities.
    #[serde(rename = "amount")]
    pub amount: f32,
    /// The price of the security at which this transaction occurred.
    #[serde(rename = "price")]
    pub price: f32,
    /// The combined value of all fees applied to this transaction
    #[serde(rename = "fees")]
    pub fees: Option<f32>,
    /// Value is one of the following: `buy`: Buying an investment `sell`: Selling an investment `cancel`: A cancellation of a pending transaction `cash`: Activity that modifies a cash position `fee`: A fee on the account `transfer`: Activity which modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer  For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).
    #[serde(rename = "type")]
    pub _type: Type,
    /// For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).
    #[serde(rename = "subtype")]
    pub subtype: Subtype,
    /// The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-`null`.
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
    #[serde(rename = "unofficial_currency_code")]
    pub unofficial_currency_code: Option<String>,
}

impl InvestmentTransaction {
    /// A transaction within an investment account.
    pub fn new(investment_transaction_id: String, account_id: String, security_id: Option<String>, date: String, name: String, quantity: f32, amount: f32, price: f32, fees: Option<f32>, _type: Type, subtype: Subtype, iso_currency_code: Option<String>, unofficial_currency_code: Option<String>) -> InvestmentTransaction {
        InvestmentTransaction {
            investment_transaction_id,
            cancel_transaction_id: None,
            account_id,
            security_id,
            date,
            name,
            quantity,
            amount,
            price,
            fees,
            _type,
            subtype,
            iso_currency_code,
            unofficial_currency_code,
        }
    }
}

/// Value is one of the following: `buy`: Buying an investment `sell`: Selling an investment `cancel`: A cancellation of a pending transaction `cash`: Activity that modifies a cash position `fee`: A fee on the account `transfer`: Activity which modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer  For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "fee")]
    Fee,
    #[serde(rename = "transfer")]
    Transfer,
}

impl Default for Type {
    fn default() -> Type {
        Self::Buy
    }
}
/// For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Subtype {
    #[serde(rename = "account fee")]
    AccountFee,
    #[serde(rename = "adjustment")]
    Adjustment,
    #[serde(rename = "assignment")]
    Assignment,
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "buy to cover")]
    BuyToCover,
    #[serde(rename = "contribution")]
    Contribution,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "distribution")]
    Distribution,
    #[serde(rename = "dividend")]
    Dividend,
    #[serde(rename = "dividend reinvestment")]
    DividendReinvestment,
    #[serde(rename = "exercise")]
    Exercise,
    #[serde(rename = "expire")]
    Expire,
    #[serde(rename = "fund fee")]
    FundFee,
    #[serde(rename = "interest")]
    Interest,
    #[serde(rename = "interest receivable")]
    InterestReceivable,
    #[serde(rename = "interest reinvestment")]
    InterestReinvestment,
    #[serde(rename = "legal fee")]
    LegalFee,
    #[serde(rename = "loan payment")]
    LoanPayment,
    #[serde(rename = "long-term capital gain")]
    LongTermCapitalGain,
    #[serde(rename = "long-term capital gain reinvestment")]
    LongTermCapitalGainReinvestment,
    #[serde(rename = "management fee")]
    ManagementFee,
    #[serde(rename = "margin expense")]
    MarginExpense,
    #[serde(rename = "merger")]
    Merger,
    #[serde(rename = "miscellaneous fee")]
    MiscellaneousFee,
    #[serde(rename = "non-qualified dividend")]
    NonQualifiedDividend,
    #[serde(rename = "non-resident tax")]
    NonResidentTax,
    #[serde(rename = "pending credit")]
    PendingCredit,
    #[serde(rename = "pending debit")]
    PendingDebit,
    #[serde(rename = "qualified dividend")]
    QualifiedDividend,
    #[serde(rename = "rebalance")]
    Rebalance,
    #[serde(rename = "return of principal")]
    ReturnOfPrincipal,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "sell short")]
    SellShort,
    #[serde(rename = "short-term capital gain")]
    ShortTermCapitalGain,
    #[serde(rename = "short-term capital gain reinvestment")]
    ShortTermCapitalGainReinvestment,
    #[serde(rename = "spin off")]
    SpinOff,
    #[serde(rename = "split")]
    Split,
    #[serde(rename = "stock distribution")]
    StockDistribution,
    #[serde(rename = "tax")]
    Tax,
    #[serde(rename = "tax withheld")]
    TaxWithheld,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "transfer fee")]
    TransferFee,
    #[serde(rename = "trust fee")]
    TrustFee,
    #[serde(rename = "unqualified gain")]
    UnqualifiedGain,
    #[serde(rename = "withdrawal")]
    Withdrawal,
}

impl Default for Subtype {
    fn default() -> Subtype {
        Self::AccountFee
    }
}

