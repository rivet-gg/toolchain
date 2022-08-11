use anyhow::{bail, Context, Result};

/// Uses the provided token to find the game ID to modify.
pub async fn infer_game_id(ctx: &rivetctl::Ctx) -> Result<String> {
	let inspect = ctx
		.client()
		.inspect()
		.send()
		.await
		.context("client.inspect")?;
	let game_cloud = if let rivetctl::rivet_cloud::model::AuthAgent::GameCloud(game_cloud) =
		inspect.agent().unwrap()
	{
		game_cloud
	} else {
		bail!("invalid token agent");
	};

	Ok(game_cloud.game_id().unwrap().to_string())
}
