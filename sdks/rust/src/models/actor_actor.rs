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
pub struct ActorActor {
	#[serde(rename = "created_at")]
	pub created_at: i64,
	#[serde(rename = "destroyed_at", skip_serializing_if = "Option::is_none")]
	pub destroyed_at: Option<i64>,
	#[serde(rename = "id")]
	pub id: uuid::Uuid,
	#[serde(rename = "lifecycle")]
	pub lifecycle: Box<crate::models::ActorLifecycle>,
	#[serde(rename = "network")]
	pub network: Box<crate::models::ActorNetwork>,
	#[serde(rename = "region")]
	pub region: String,
	#[serde(rename = "resources")]
	pub resources: Box<crate::models::ActorResources>,
	#[serde(rename = "runtime")]
	pub runtime: Box<crate::models::ActorRuntime>,
	#[serde(rename = "started_at", skip_serializing_if = "Option::is_none")]
	pub started_at: Option<i64>,
	#[serde(rename = "tags", deserialize_with = "Option::deserialize")]
	pub tags: Option<serde_json::Value>,
}

impl ActorActor {
	pub fn new(
		created_at: i64,
		id: uuid::Uuid,
		lifecycle: crate::models::ActorLifecycle,
		network: crate::models::ActorNetwork,
		region: String,
		resources: crate::models::ActorResources,
		runtime: crate::models::ActorRuntime,
		tags: Option<serde_json::Value>,
	) -> ActorActor {
		ActorActor {
			created_at,
			destroyed_at: None,
			id,
			lifecycle: Box::new(lifecycle),
			network: Box::new(network),
			region,
			resources: Box::new(resources),
			runtime: Box::new(runtime),
			started_at: None,
			tags,
		}
	}
}
