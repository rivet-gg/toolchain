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
pub struct ActorCreateActorResponse {
	#[serde(rename = "actor")]
	pub actor: Box<crate::models::ActorActor>,
}

impl ActorCreateActorResponse {
	pub fn new(actor: crate::models::ActorActor) -> ActorCreateActorResponse {
		ActorCreateActorResponse {
			actor: Box::new(actor),
		}
	}
}