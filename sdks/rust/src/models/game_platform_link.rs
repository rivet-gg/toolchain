/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GamePlatformLink : A platform link denoting a supported platform.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GamePlatformLink {
	/// Represent a resource's readable display name.
	#[serde(rename = "display_name")]
	pub display_name: String,
	/// The URL to the given game's method of distribution on this platform.
	#[serde(rename = "url")]
	pub url: String,
}

impl GamePlatformLink {
	/// A platform link denoting a supported platform.
	pub fn new(display_name: String, url: String) -> GamePlatformLink {
		GamePlatformLink { display_name, url }
	}
}
