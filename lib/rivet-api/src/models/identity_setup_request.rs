/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentitySetupRequest {
	/// Documentation at https://jwt.io/
	#[serde(
		rename = "existing_identity_token",
		skip_serializing_if = "Option::is_none"
	)]
	pub existing_identity_token: Option<String>,
}

impl IdentitySetupRequest {
	pub fn new() -> IdentitySetupRequest {
		IdentitySetupRequest {
			existing_identity_token: None,
		}
	}
}
