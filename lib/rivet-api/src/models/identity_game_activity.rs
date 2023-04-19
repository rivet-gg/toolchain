/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityGameActivity : The game an identity is currently participating in.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityGameActivity {
	#[serde(rename = "game")]
	pub game: Box<crate::models::GameHandle>,
	/// A short activity message about the current game activity.
	#[serde(rename = "message")]
	pub message: String,
	/// JSON data seen only by the given identity and their mutual followers.
	#[serde(rename = "mutual_metadata", deserialize_with = "Option::deserialize")]
	pub mutual_metadata: Option<serde_json::Value>,
	/// JSON data seen by anyone.
	#[serde(rename = "public_metadata", deserialize_with = "Option::deserialize")]
	pub public_metadata: Option<serde_json::Value>,
}

impl IdentityGameActivity {
	/// The game an identity is currently participating in.
	pub fn new(
		game: crate::models::GameHandle,
		message: String,
		mutual_metadata: Option<serde_json::Value>,
		public_metadata: Option<serde_json::Value>,
	) -> IdentityGameActivity {
		IdentityGameActivity {
			game: Box::new(game),
			message,
			mutual_metadata,
			public_metadata,
		}
	}
}
