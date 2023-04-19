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
pub struct CloudGamesCreateGameBuildRequest {
	/// Represent a resource's readable display name.
	#[serde(rename = "display_name")]
	pub display_name: String,
	#[serde(rename = "image_file")]
	pub image_file: Box<crate::models::UploadPrepareFile>,
	/// A tag given to the game build.
	#[serde(rename = "image_tag")]
	pub image_tag: String,
}

impl CloudGamesCreateGameBuildRequest {
	pub fn new(
		display_name: String,
		image_file: crate::models::UploadPrepareFile,
		image_tag: String,
	) -> CloudGamesCreateGameBuildRequest {
		CloudGamesCreateGameBuildRequest {
			display_name,
			image_file: Box::new(image_file),
			image_tag,
		}
	}
}
