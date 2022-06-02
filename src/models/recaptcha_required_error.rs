/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RecaptchaRequiredError : The request was flagged by Plaid's fraud system, and requires additional verification to ensure they are not a bot.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RecaptchaRequiredError {
    /// RECAPTCHA_ERROR
    #[serde(rename = "error_type")]
    pub error_type: String,
    /// RECAPTCHA_REQUIRED
    #[serde(rename = "error_code")]
    pub error_code: String,
    #[serde(rename = "display_message")]
    pub display_message: String,
    /// 400
    #[serde(rename = "http_code")]
    pub http_code: String,
    /// Your user will be prompted to solve a Google reCAPTCHA challenge in the Link Recaptcha pane. If they solve the challenge successfully, the user's request is resubmitted and they are directed to the next Item creation step.
    #[serde(rename = "link_user_experience")]
    pub link_user_experience: String,
    /// Plaid's fraud system detects abusive traffic and considers a variety of parameters throughout Item creation requests. When a request is considered risky or possibly fraudulent, Link presents a reCAPTCHA for the user to solve.
    #[serde(rename = "common_causes")]
    pub common_causes: String,
    /// Link will automatically guide your user through reCAPTCHA verification. As a general rule, we recommend instrumenting basic fraud monitoring to detect and protect your website from spam and abuse.  If your user cannot verify their session, please submit a Support ticket with the following identifiers: `link_session_id` or `request_id`
    #[serde(rename = "troubleshooting_steps")]
    pub troubleshooting_steps: String,
}

impl RecaptchaRequiredError {
    /// The request was flagged by Plaid's fraud system, and requires additional verification to ensure they are not a bot.
    pub fn new(error_type: String, error_code: String, display_message: String, http_code: String, link_user_experience: String, common_causes: String, troubleshooting_steps: String) -> RecaptchaRequiredError {
        RecaptchaRequiredError {
            error_type,
            error_code,
            display_message,
            http_code,
            link_user_experience,
            common_causes,
            troubleshooting_steps,
        }
    }
}


