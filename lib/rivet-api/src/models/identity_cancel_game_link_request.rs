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
pub struct IdentityCancelGameLinkRequest {
	/// Documentation at https://jwt.io/
	#[serde(rename = "identity_link_token")]
	pub identity_link_token: String,
}

impl IdentityCancelGameLinkRequest {
	pub fn new(identity_link_token: String) -> IdentityCancelGameLinkRequest {
		IdentityCancelGameLinkRequest {
			identity_link_token,
		}
	}
}
