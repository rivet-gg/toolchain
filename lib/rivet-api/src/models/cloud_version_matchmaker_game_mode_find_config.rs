/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerGameModeFindConfig : Configures the requirements and authentication for the /find endpoint. If this value is not set in the config, the /find endpoint is still enabled.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudVersionMatchmakerGameModeFindConfig {
	/// Sets whether or not the /find endpoint is enabled.
	#[serde(rename = "enabled")]
	pub enabled: bool,
	#[serde(
		rename = "identity_requirement",
		skip_serializing_if = "Option::is_none"
	)]
	pub identity_requirement:
		Option<crate::models::CloudVersionMatchmakerGameModeIdentityRequirement>,
	#[serde(rename = "verification", skip_serializing_if = "Option::is_none")]
	pub verification: Option<Box<crate::models::CloudVersionMatchmakerGameModeVerificationConfig>>,
}

impl CloudVersionMatchmakerGameModeFindConfig {
	/// Configures the requirements and authentication for the /find endpoint. If this value is not set in the config, the /find endpoint is still enabled.
	pub fn new(enabled: bool) -> CloudVersionMatchmakerGameModeFindConfig {
		CloudVersionMatchmakerGameModeFindConfig {
			enabled,
			identity_requirement: None,
			verification: None,
		}
	}
}
