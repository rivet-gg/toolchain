/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudGameFull : A full game.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudGameFull {
	/// A list of region summaries.
	#[serde(rename = "available_regions")]
	pub available_regions: Vec<crate::models::CloudRegionSummary>,
	/// The URL of this game's banner image.
	#[serde(rename = "banner_url", skip_serializing_if = "Option::is_none")]
	pub banner_url: Option<String>,
	/// RFC3339 timestamp.
	#[serde(rename = "create_ts")]
	pub create_ts: String,
	#[serde(rename = "developer_group_id")]
	pub developer_group_id: uuid::Uuid,
	/// Represent a resource's readable display name.
	#[serde(rename = "display_name")]
	pub display_name: String,
	#[serde(rename = "game_id")]
	pub game_id: uuid::Uuid,
	/// The URL of this game's logo image.
	#[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
	pub logo_url: Option<String>,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	#[serde(rename = "name_id")]
	pub name_id: String,
	/// A list of namespace summaries.
	#[serde(rename = "namespaces")]
	pub namespaces: Vec<crate::models::CloudNamespaceSummary>,
	/// Unsigned 32 bit integer.
	#[serde(rename = "total_player_count")]
	pub total_player_count: i32,
	/// A list of version summaries.
	#[serde(rename = "versions")]
	pub versions: Vec<crate::models::CloudVersionSummary>,
}

impl CloudGameFull {
	/// A full game.
	pub fn new(
		available_regions: Vec<crate::models::CloudRegionSummary>,
		create_ts: String,
		developer_group_id: uuid::Uuid,
		display_name: String,
		game_id: uuid::Uuid,
		name_id: String,
		namespaces: Vec<crate::models::CloudNamespaceSummary>,
		total_player_count: i32,
		versions: Vec<crate::models::CloudVersionSummary>,
	) -> CloudGameFull {
		CloudGameFull {
			available_regions,
			banner_url: None,
			create_ts,
			developer_group_id,
			display_name,
			game_id,
			logo_url: None,
			name_id,
			namespaces,
			total_player_count,
			versions,
		}
	}
}
