/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActorPatchBuildTagsRequest {
	/// Removes the given tag keys from all other builds.
	#[serde(rename = "exclusive_tags", skip_serializing_if = "Option::is_none")]
	pub exclusive_tags: Option<Vec<String>>,
	#[serde(rename = "tags", deserialize_with = "Option::deserialize")]
	pub tags: Option<serde_json::Value>,
}

impl ActorPatchBuildTagsRequest {
	pub fn new(tags: Option<serde_json::Value>) -> ActorPatchBuildTagsRequest {
		ActorPatchBuildTagsRequest {
			exclusive_tags: None,
			tags,
		}
	}
}
