/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerGameModeVerificationConfig : Configuration that tells Rivet where to send validation requests and with what headers. When set, Rivet will send the `verification_data` property (given by the user in the find/join/create endpoint) to the given url along with the headers provided and some information about the requested lobby. The response of this request will determine if the user can join that lobby or not.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionMatchmakerGameModeVerificationConfig {
	#[serde(rename = "headers")]
	pub headers: ::std::collections::HashMap<String, String>,
	#[serde(rename = "url")]
	pub url: String,
}

impl CloudVersionMatchmakerGameModeVerificationConfig {
	/// Configuration that tells Rivet where to send validation requests and with what headers. When set, Rivet will send the `verification_data` property (given by the user in the find/join/create endpoint) to the given url along with the headers provided and some information about the requested lobby. The response of this request will determine if the user can join that lobby or not.
	pub fn new(
		headers: ::std::collections::HashMap<String, String>,
		url: String,
	) -> CloudVersionMatchmakerGameModeVerificationConfig {
		CloudVersionMatchmakerGameModeVerificationConfig { headers, url }
	}
}
