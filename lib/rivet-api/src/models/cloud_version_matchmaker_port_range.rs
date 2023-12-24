/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerPortRange : Range of ports that can be connected to.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudVersionMatchmakerPortRange {
	/// Unsigned 32 bit integer.
	#[serde(rename = "max")]
	pub max: i32,
	/// Unsigned 32 bit integer.
	#[serde(rename = "min")]
	pub min: i32,
}

impl CloudVersionMatchmakerPortRange {
	/// Range of ports that can be connected to.
	pub fn new(max: i32, min: i32) -> CloudVersionMatchmakerPortRange {
		CloudVersionMatchmakerPortRange { max, min }
	}
}