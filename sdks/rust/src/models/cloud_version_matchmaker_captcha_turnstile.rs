/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerCaptchaTurnstile : Turnstile captcha configuration.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionMatchmakerCaptchaTurnstile {
	#[serde(rename = "secret_key")]
	pub secret_key: String,
	#[serde(rename = "site_key")]
	pub site_key: String,
}

impl CloudVersionMatchmakerCaptchaTurnstile {
	/// Turnstile captcha configuration.
	pub fn new(secret_key: String, site_key: String) -> CloudVersionMatchmakerCaptchaTurnstile {
		CloudVersionMatchmakerCaptchaTurnstile {
			secret_key,
			site_key,
		}
	}
}
