/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GameStatSortingMethod : A value denoting the sorting method of a game statistic.

/// A value denoting the sorting method of a game statistic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum GameStatSortingMethod {
	#[serde(rename = "desc")]
	Desc,
	#[serde(rename = "asc")]
	Asc,
}

impl ToString for GameStatSortingMethod {
	fn to_string(&self) -> String {
		match self {
			Self::Desc => String::from("desc"),
			Self::Asc => String::from("asc"),
		}
	}
}

impl Default for GameStatSortingMethod {
	fn default() -> GameStatSortingMethod {
		Self::Desc
	}
}
