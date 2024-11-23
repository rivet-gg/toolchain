/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudBuildSummary : A build summary.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudBuildSummary {
	#[serde(rename = "build_id")]
	pub build_id: uuid::Uuid,
	/// Whether or not this build has completely been uploaded.
	#[serde(rename = "complete")]
	pub complete: bool,
	/// Unsigned 64 bit integer.
	#[serde(rename = "content_length")]
	pub content_length: i64,
	/// RFC3339 timestamp
	#[serde(rename = "create_ts")]
	pub create_ts: String,
	/// Represent a resource's readable display name.
	#[serde(rename = "display_name")]
	pub display_name: String,
	/// Tags of this build
	#[serde(rename = "tags")]
	pub tags: ::std::collections::HashMap<String, String>,
	#[serde(rename = "upload_id")]
	pub upload_id: uuid::Uuid,
}

impl CloudBuildSummary {
	/// A build summary.
	pub fn new(
		build_id: uuid::Uuid,
		complete: bool,
		content_length: i64,
		create_ts: String,
		display_name: String,
		tags: ::std::collections::HashMap<String, String>,
		upload_id: uuid::Uuid,
	) -> CloudBuildSummary {
		CloudBuildSummary {
			build_id,
			complete,
			content_length,
			create_ts,
			display_name,
			tags,
			upload_id,
		}
	}
}
