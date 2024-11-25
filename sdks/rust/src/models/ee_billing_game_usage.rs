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
pub struct EeBillingGameUsage {
	#[serde(rename = "game_id")]
	pub game_id: uuid::Uuid,
	#[serde(rename = "regions")]
	pub regions: Vec<crate::models::EeBillingRegionUsage>,
}

impl EeBillingGameUsage {
	pub fn new(
		game_id: uuid::Uuid,
		regions: Vec<crate::models::EeBillingRegionUsage>,
	) -> EeBillingGameUsage {
		EeBillingGameUsage { game_id, regions }
	}
}
