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
pub struct EeCloudGroupsBillingGetBillingResponse {
	#[serde(rename = "group")]
	pub group: Box<crate::models::EeBillingGroup>,
	#[serde(rename = "watch")]
	pub watch: Box<crate::models::WatchResponse>,
}

impl EeCloudGroupsBillingGetBillingResponse {
	pub fn new(
		group: crate::models::EeBillingGroup,
		watch: crate::models::WatchResponse,
	) -> EeCloudGroupsBillingGetBillingResponse {
		EeCloudGroupsBillingGetBillingResponse {
			group: Box::new(group),
			watch: Box::new(watch),
		}
	}
}
