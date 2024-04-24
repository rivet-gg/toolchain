use clap::Parser;
use cli_core::rivet_api::apis;
use global_error::prelude::*;

use tabled::Tabled;

use crate::{commands::deploy, util::term};

#[derive(Parser)]
pub enum SubCommand {
	/// List all versions
	List,

	/// Get details about a version
	Get {
		/// The version ID
		version: String,
	},

	/// Show version's dashboard
	#[clap(alias = "dashboard", alias("dash"))]
	View { version: String },

	/// Deprecated. Use `rivet deploy`.
	///
	/// Pushes the build and site and creates a new version
	#[clap(
		hide = true,
		alias = "push-and-create",
		alias = "create",
		alias = "publish"
	)]
	Deploy(deploy::Opts),
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::List => {
				let game_res = unwrap!(
					apis::cloud_games_api::cloud_games_get_game_by_id(
						&ctx.openapi_config_cloud,
						&ctx.game_id,
						None,
					)
					.await
				);
				let game = &game_res.game;
				let namespaces = &game.namespaces;

				#[derive(Tabled)]
				struct Version {
					#[tabled(rename = "Name")]
					display_name: String,
					#[tabled(rename = "Namespaces")]
					namespaces: String,
					#[tabled(rename = "Created")]
					created: String,
					#[tabled(rename = "ID")]
					version_id: String,
				}

				let mut version = game
					.versions
					.iter()
					.map(|version| {
						let ns = namespaces
							.iter()
							.filter(|ns| ns.version_id == version.version_id)
							.map(|ns| ns.display_name.as_str())
							.collect::<Vec<_>>()
							.join(", ");

						Ok(Version {
							display_name: version.display_name.clone(),
							namespaces: ns,
							created: version.create_ts.clone(),
							version_id: version.version_id.to_string(),
						})
					})
					.collect::<GlobalResult<Vec<_>>>()?;
				version.reverse();
				term::table(&version);

				Ok(())
			}
			SubCommand::Get { version } => {
				print_version(ctx, &version).await?;

				Ok(())
			}
			SubCommand::Deploy(opts) => opts.execute(ctx).await,
			SubCommand::View { version } => {
				// Check the version exists
				unwrap!(
					apis::cloud_games_versions_api::cloud_games_versions_get_game_version_by_id(
						&ctx.openapi_config_cloud,
						&ctx.game_id,
						&version
					)
					.await
				);

				eprintln!("{}", term::link(dashboard_url(&ctx, &ctx.game_id, version)));

				Ok(())
			}
		}
	}
}

/// Prints information about a game version
async fn print_version(ctx: &cli_core::Ctx, version_id: &str) -> GlobalResult<()> {
	let version_res = unwrap!(
		apis::cloud_games_versions_api::cloud_games_versions_get_game_version_by_id(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			&version_id,
		)
		.await
	);
	let version = &version_res.version;

	println!("{version:#?}");

	Ok(())
}

pub fn dashboard_url(ctx: &cli_core::Ctx, game_id: &str, version_id: &str) -> String {
	format!(
		"{}/developer/games/{game_id}/versions/{version_id}",
		ctx.bootstrap.origins.hub,
		game_id = game_id,
		version_id = version_id
	)
}

pub fn rivet_game_url(domain_cdn: &str, game_name_id: &str, namespace_name_id: &str) -> String {
	if namespace_name_id == "prod" {
		format!("https://{game_name_id}.{domain_cdn}/")
	} else {
		format!("https://{game_name_id}--{namespace_name_id}.{domain_cdn}/")
	}
}
