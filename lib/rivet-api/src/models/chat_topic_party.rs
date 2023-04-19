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
pub struct ChatTopicParty {
	#[serde(rename = "party")]
	pub party: Box<crate::models::PartyHandle>,
}

impl ChatTopicParty {
	pub fn new(party: crate::models::PartyHandle) -> ChatTopicParty {
		ChatTopicParty {
			party: Box::new(party),
		}
	}
}
