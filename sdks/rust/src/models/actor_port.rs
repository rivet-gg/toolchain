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
pub struct ActorPort {
	#[serde(rename = "internal_port", skip_serializing_if = "Option::is_none")]
	pub internal_port: Option<i32>,
	#[serde(rename = "protocol")]
	pub protocol: crate::models::ActorPortProtocol,
	#[serde(rename = "public_hostname", skip_serializing_if = "Option::is_none")]
	pub public_hostname: Option<String>,
	#[serde(rename = "public_port", skip_serializing_if = "Option::is_none")]
	pub public_port: Option<i32>,
	#[serde(rename = "routing")]
	pub routing: Box<crate::models::ActorPortRouting>,
}

impl ActorPort {
	pub fn new(
		protocol: crate::models::ActorPortProtocol,
		routing: crate::models::ActorPortRouting,
	) -> ActorPort {
		ActorPort {
			internal_port: None,
			protocol,
			public_hostname: None,
			public_port: None,
			routing: Box::new(routing),
		}
	}
}
