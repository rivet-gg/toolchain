/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudGroupBillingStatus : A value denoting the status of a billing transfer.

/// A value denoting the status of a billing transfer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CloudGroupBillingStatus {
	#[serde(rename = "succeeded")]
	Succeeded,
	#[serde(rename = "processing")]
	Processing,
	#[serde(rename = "refunded")]
	Refunded,
}

impl ToString for CloudGroupBillingStatus {
	fn to_string(&self) -> String {
		match self {
			Self::Succeeded => String::from("succeeded"),
			Self::Processing => String::from("processing"),
			Self::Refunded => String::from("refunded"),
		}
	}
}

impl Default for CloudGroupBillingStatus {
	fn default() -> CloudGroupBillingStatus {
		Self::Succeeded
	}
}