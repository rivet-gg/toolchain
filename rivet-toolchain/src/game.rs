use futures_util::stream::{StreamExt, TryStreamExt};
use global_error::prelude::*;
use rivet_api::{apis, models};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::ctx::Ctx;

// TODO: Replace this with a production API
#[derive(Serialize)]
pub struct TEMPEnvironment {
	pub id: Uuid,
	pub created_at: String,
	pub slug: String,
	pub name: String,
}

impl From<models::CloudNamespaceSummary> for TEMPEnvironment {
	fn from(ns: models::CloudNamespaceSummary) -> Self {
		TEMPEnvironment {
			id: ns.namespace_id,
			created_at: ns.create_ts,
			slug: ns.name_id,
			name: ns.display_name,
		}
	}
}

pub async fn get_env(ctx: &Ctx, env_id: Uuid) -> GlobalResult<TEMPEnvironment> {
	let ns = apis::cloud_games_namespaces_api::cloud_games_namespaces_get_game_namespace_by_id(
		&ctx.openapi_config_cloud,
		&ctx.game_id.to_string(),
		&env_id.to_string(),
		None,
	)
	.await?;

	Ok(TEMPEnvironment::from(ns))
}
