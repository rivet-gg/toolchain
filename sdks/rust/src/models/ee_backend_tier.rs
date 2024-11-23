/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EeBackendTier {
	#[serde(rename = "shared")]
	Shared,
	#[serde(rename = "dedicated")]
	Dedicated,
}

impl ToString for EeBackendTier {
	fn to_string(&self) -> String {
		match self {
			Self::Shared => String::from("shared"),
			Self::Dedicated => String::from("dedicated"),
		}
	}
}

impl Default for EeBackendTier {
	fn default() -> EeBackendTier {
		Self::Shared
	}
}
