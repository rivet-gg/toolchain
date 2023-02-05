/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudCaptchaLevel : How hard a captcha should be.

/// How hard a captcha should be.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CloudCaptchaLevel {
    #[serde(rename = "easy")]
    Easy,
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "difficult")]
    Difficult,
    #[serde(rename = "always_on")]
    AlwaysOn,

}

impl ToString for CloudCaptchaLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Easy => String::from("easy"),
            Self::Moderate => String::from("moderate"),
            Self::Difficult => String::from("difficult"),
            Self::AlwaysOn => String::from("always_on"),
        }
    }
}

impl Default for CloudCaptchaLevel {
    fn default() -> CloudCaptchaLevel {
        Self::Easy
    }
}




