/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EeCloudGroupsBillingGetBillingUsageResponse {
	#[serde(rename = "games")]
	pub games: Vec<crate::models::EeBillingGameUsage>,
}

impl EeCloudGroupsBillingGetBillingUsageResponse {
	pub fn new(
		games: Vec<crate::models::EeBillingGameUsage>,
	) -> EeCloudGroupsBillingGetBillingUsageResponse {
		EeCloudGroupsBillingGetBillingUsageResponse { games }
	}
}
