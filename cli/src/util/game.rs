use anyhow::{bail, Context, Result};

/// Uses the provided token to find the game ID to modify.
pub async fn infer_game_id(ctx: &cli_core::Ctx) -> Result<String> {
	let inspect = ctx
		.client()
		.inspect()
		.send()
		.await
		.context("client.inspect")?;
	let game_cloud = if let cli_core::rivet_cloud::model::AuthAgent::GameCloud(game_cloud) =
		inspect.agent().unwrap()
	{
		game_cloud
	} else {
		bail!("invalid token agent");
	};

	Ok(game_cloud.game_id().unwrap().to_string())
}
