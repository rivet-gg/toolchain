/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// PartyCreateInviteConfig : Configuration for creating a party invite.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartyCreateInviteConfig {
	/// An alias used to join a given party. This alias must be unique for all invites for your game. Pass this alias to `rivet.api.party.common#CreatedInvite$alias` to consume the invite.
	#[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
	pub alias: Option<String>,
}

impl PartyCreateInviteConfig {
	/// Configuration for creating a party invite.
	pub fn new() -> PartyCreateInviteConfig {
		PartyCreateInviteConfig { alias: None }
	}
}
