use clap::Parser;
use cli_core::rivet_api::{apis, models};
use global_error::prelude::*;
use serde::Serialize;
use tabled::Tabled;
use uuid::Uuid;

use crate::util::{struct_fmt, term};

#[derive(Parser)]
pub enum SubCommand {
	/// List all namespaces
	List,
	/// Get details about a specific namespace
	Get {
		/// Namespace ID
		namespace: Uuid,
		#[clap(long, value_parser)]
		format: struct_fmt::Format,
	},
	/// Create a namespace
	Create {
		/// ID to reference the namespace by
		#[clap(long = "id", alias = "name-id")]
		name_id: String,
		/// Name to display for the namespace
		#[clap(long = "name", alias = "display-name")]
		display_name: String,
		/// Initial version to publish to the namespace
		#[clap(long)]
		version: Option<Uuid>,
		#[clap(long, value_parser)]
		format: Option<struct_fmt::Format>,
	},
	/// Publish a version to a namespace
	SetVersion {
		/// The namespace ID to update
		#[clap(long, short, alias = "ns")]
		namespace: Uuid,
		/// The version ID to publish
		#[clap(long, short)]
		version: Uuid,
		#[clap(long, value_parser)]
		format: Option<struct_fmt::Format>,
	},
	/// Show the namespace dashboard
	#[clap(alias = "dashboard", alias = "dash")]
	View {
		/// The namespace ID
		namespace: Uuid,
	},
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::List => {
				let game_res = unwrap!(
					apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
						&ctx.openapi_config_cloud,
						&ctx.game_id,
						None,
					)
					.await
				);
				let game = &game_res.game;
				let game_versions = &game.versions;

				#[derive(Tabled)]
				struct Namespace {
					#[tabled(rename = "Name ID")]
					name_id: String,
					#[tabled(rename = "Name")]
					display_name: String,
					#[tabled(rename = "Version")]
					version: String,
					#[tabled(rename = "Created")]
					created: String,
					#[tabled(rename = "ID")]
					namespace_id: String,
				}

				let mut ns = game
					.namespaces
					.iter()
					.map(|ns| {
						let version_name = game_versions
							.iter()
							.find(|x| x.version_id == ns.version_id)
							.map(|x| x.display_name.clone())
							.unwrap_or_else(|| ns.version_id.to_string());

						Ok(Namespace {
							display_name: ns.display_name.clone(),
							name_id: ns.name_id.clone(),
							namespace_id: ns.namespace_id.to_string(),
							version: version_name,
							created: ns.create_ts.clone(),
						})
					})
					.collect::<GlobalResult<Vec<_>>>()?;
				ns.reverse();
				term::table(&ns);

				Ok(())
			}
			SubCommand::Get { namespace, format } => {
				print_ns(ctx, format, &namespace.to_string()).await?;

				Ok(())
			}
			SubCommand::Create {
				display_name,
				version,
				name_id,
				format,
			} => {
				// Get game
				let game_res = unwrap!(
					apis::cloud_games_games_api::cloud_games_games_get_game_by_id(
						&ctx.openapi_config_cloud,
						&ctx.game_id,
						None,
					)
					.await
				);
				let namespaces = &game_res.game.namespaces;

				// Find the default version to use
				let version_id = if let Some(version) = version {
					*version
				} else {
					unwrap!(game_res.game.versions.last(), "no versions").version_id
				};

				// Get or create namespace
				let ns_id = if let Some(ns) = namespaces.iter().find(|ns| &ns.name_id == name_id) {
					let ns_id = ns.namespace_id.to_string();
					let display_name = &ns.display_name;

					term::status::success("Found Existing", display_name);

					ns_id
				} else {
					let create_res = unwrap!(apis::cloud_games_namespaces_api::cloud_games_namespaces_create_game_namespace(
					&ctx.openapi_config_cloud,
					&ctx.game_id,
					models::CloudGamesNamespacesCreateGameNamespaceRequest {
						display_name: display_name.clone(),
						name_id: name_id.clone(),
						version_id,
					}).await);
					let ns_id = create_res.namespace_id.to_string();

					term::status::success("Created", display_name);

					ns_id
				};

				term::status::info("Dashboard", dashboard_url(&ctx, &ctx.game_id, &ns_id));

				if let Some(format) = format {
					print_ns(ctx, format, &ns_id).await?;
				}

				Ok(())
			}
			SubCommand::SetVersion {
				namespace,
				version,
				format,
			} => {
				unwrap!(apis::cloud_games_namespaces_api::cloud_games_namespaces_update_game_namespace_version(
					&ctx.openapi_config_cloud,
					&ctx.game_id,
					&namespace.to_string(),
					models::CloudGamesNamespacesUpdateGameNamespaceVersionRequest {
						version_id: *version,
					}
				).await);

				term::status::success("Version Set", "");

				if let Some(format) = format {
					print_ns(ctx, format, &namespace.to_string()).await?;
				}

				Ok(())
			}
			SubCommand::View { namespace } => {
				// Check the namespace exists
				unwrap!(apis::cloud_games_namespaces_api::cloud_games_namespaces_get_game_namespace_by_id(
					&ctx.openapi_config_cloud,
					&ctx.game_id,
					&namespace.to_string()
				).await);

				eprintln!(
					"{}",
					term::link(dashboard_url(&ctx, &ctx.game_id, &namespace.to_string()))
				);

				Ok(())
			}
		}
	}
}

async fn print_ns(
	ctx: &cli_core::Ctx,
	format: &struct_fmt::Format,
	namespace_id: &str,
) -> GlobalResult<()> {
	let ns_res = unwrap!(
		apis::cloud_games_namespaces_api::cloud_games_namespaces_get_game_namespace_by_id(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			namespace_id
		)
		.await
	);
	let ns = &ns_res.namespace;

	#[derive(Serialize)]
	struct Output<'a> {
		namespace_id: &'a str,
		created: &'a str,
		display_name: &'a str,
		version_id: &'a str,
	}
	struct_fmt::print(
		format,
		&Output {
			namespace_id: &ns.namespace_id.to_string(),
			created: &ns.create_ts,
			display_name: &ns.display_name,
			version_id: &ns.version_id.to_string(),
		},
	)?;

	Ok(())
}

pub fn dashboard_url(ctx: &cli_core::Ctx, game_id: &str, ns_id: &str) -> String {
	format!(
		"{}/developer/games/{game_id}/namespaces/{ns_id}",
		ctx.bootstrap.origins.hub
	)
}
