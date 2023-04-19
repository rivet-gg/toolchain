/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudRegionTier : A region server tier.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudRegionTier {
	/// Internet bandwidth (MB).
	#[serde(rename = "bandwidth")]
	pub bandwidth: i32,
	/// CPU frequency (MHz).
	#[serde(rename = "cpu")]
	pub cpu: i32,
	/// Allocated disk space (MB).
	#[serde(rename = "disk")]
	pub disk: i32,
	/// Allocated memory (MB).
	#[serde(rename = "memory")]
	pub memory: i32,
	/// Price billed for every second this server is running (in quadrillionth USD, 1,000,000,000,000 = $1.00).
	#[serde(rename = "price_per_second")]
	pub price_per_second: i32,
	/// Together with the numerator, denotes the portion of the CPU a given server uses.
	#[serde(rename = "rivet_cores_denominator")]
	pub rivet_cores_denominator: i32,
	/// Together with the denominator, denotes the portion of the CPU a given server uses.
	#[serde(rename = "rivet_cores_numerator")]
	pub rivet_cores_numerator: i32,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	#[serde(rename = "tier_name_id")]
	pub tier_name_id: String,
}

impl CloudRegionTier {
	/// A region server tier.
	pub fn new(
		bandwidth: i32,
		cpu: i32,
		disk: i32,
		memory: i32,
		price_per_second: i32,
		rivet_cores_denominator: i32,
		rivet_cores_numerator: i32,
		tier_name_id: String,
	) -> CloudRegionTier {
		CloudRegionTier {
			bandwidth,
			cpu,
			disk,
			memory,
			price_per_second,
			rivet_cores_denominator,
			rivet_cores_numerator,
			tier_name_id,
		}
	}
}
