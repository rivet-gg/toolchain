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
pub struct ChatSimpleTopicGroup {
	#[serde(rename = "group")]
	pub group: uuid::Uuid,
}

impl ChatSimpleTopicGroup {
	pub fn new(group: uuid::Uuid) -> ChatSimpleTopicGroup {
		ChatSimpleTopicGroup { group }
	}
}
