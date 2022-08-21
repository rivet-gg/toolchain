use anyhow::{Context, Result};
use clap::Parser;
use serde::Serialize;
use tabled::Tabled;

use crate::util::{fmt, struct_fmt, term};

#[derive(Parser)]
pub enum SubCommand {
	List,
	Get {
		namespace: String,
		#[clap(long, value_parser)]
		format: struct_fmt::Format,
	},
	Create {
		#[clap(long)]
		name_id: String,
		#[clap(long)]
		display_name: String,
		#[clap(long)]
		version: String,
		#[clap(long, value_parser)]
		format: Option<struct_fmt::Format>,
	},
	SetVersion {
		#[clap(long, short, alias = "ns")]
		namespace: String,
		#[clap(long, short)]
		version: String,
		#[clap(long, value_parser)]
		format: Option<struct_fmt::Format>,
	},
	#[clap(alias = "dash")]
	Dashboard {
		namespace: String,
	},
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::List => {
				let game_res = ctx
					.client()
					.get_game_by_id()
					.game_id(&ctx.game_id)
					.send()
					.await
					.context("client.get_game_by_id")?;
				let game = game_res.game.context("game_res.game")?;
				let game_versions = game.versions().context("game.versions")?;

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
					.namespaces()
					.context("game.namespaces")?
					.iter()
					.map(|ns| {
						let version_id = ns.version_id().context("ns.version_id")?.to_string();
						let version_name = game_versions
							.iter()
							.find(|x| x.version_id().map_or(false, |id| id == version_id))
							.and_then(|x| x.display_name())
							.map_or_else(|| version_id.to_string(), |x| x.to_string());

						Ok(Namespace {
							display_name: ns.display_name().context("ns.display_name")?.to_string(),
							name_id: ns.name_id().context("ns.name_id")?.to_string(),
							namespace_id: ns.namespace_id().context("ns.namespace_id")?.to_string(),
							version: version_name,
							created: fmt::date(ns.create_ts().context("ns.create_ts")?),
						})
					})
					.collect::<Result<Vec<_>>>()?;
				ns.reverse();
				term::table(&ns);

				Ok(())
			}
			SubCommand::Get { namespace, format } => {
				print_ns(ctx, format, &namespace).await?;

				Ok(())
			}
			SubCommand::Create {
				display_name,
				version,
				name_id,
				format,
			} => {
				// Get game
				let game_res = ctx
					.client()
					.get_game_by_id()
					.game_id(&ctx.game_id)
					.send()
					.await
					.context("client.get_game_by_id")?;
				let game = game_res.game().context("game_res.game")?;
				let namespaces = game.namespaces().context("game.namespaces")?;

				// Get or create namespace
				let ns_id =
					if let Some(ns) = namespaces.iter().find(|ns| ns.name_id() == Some(&name_id)) {
						let ns_id = ns.namespace_id().context("ns.namespace_id")?;
						let display_name = ns.display_name().context("ns.display_name")?;

						eprintln!(
							"{} {}",
							term::success_fmt("Found Existing"),
							term::info_fmt(display_name),
						);

						ns_id.to_owned()
					} else {
						term::info("Creating namespace");

						let create_res = ctx
							.client()
							.create_game_namespace()
							.game_id(&ctx.game_id)
							.display_name(display_name)
							.name_id(name_id)
							.version_id(version)
							.send()
							.await
							.context("client.create_game_namespace")?;
						let ns_id = create_res
							.namespace_id()
							.context("create_res.namespace_id")?;

						eprintln!(
							"{} {}",
							term::success_fmt("Created"),
							term::info_fmt(display_name)
						);

						ns_id.to_owned()
					};
				eprintln!(
					"{} {}",
					term::label_fmt("Dashboard"),
					term::link_fmt(dashboard_url(&ctx.game_id, ns_id))
				);

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
				ctx.client()
					.update_game_namespace_version()
					.game_id(&ctx.game_id)
					.namespace_id(namespace)
					.version_id(version)
					.send()
					.await
					.context("client.update_game_namespace_version")?;

				if let Some(format) = format {
					print_ns(ctx, format, &namespace).await?;
				}

				Ok(())
			}
			SubCommand::Dashboard { namespace } => {
				// Check the namespace exists
				ctx.client()
					.get_game_namespace_by_id()
					.game_id(&ctx.game_id)
					.namespace_id(namespace)
					.send()
					.await
					.context("client.get_game_version_by_id")?;

				term::link(dashboard_url(&ctx.game_id, namespace));

				Ok(())
			}
		}
	}
}

async fn print_ns(
	ctx: &cli_core::Ctx,
	format: &struct_fmt::Format,
	namespace_id: &str,
) -> Result<()> {
	let ns_res = ctx
		.client()
		.get_game_namespace_by_id()
		.game_id(&ctx.game_id)
		.namespace_id(namespace_id)
		.send()
		.await
		.context("client.get_game_version_by_id")?;
	let ns = ns_res.namespace().context("ns_res.namespace")?;

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
			namespace_id: ns.namespace_id().context("ns.namespace_id")?,
			created: &fmt::date(ns.create_ts().context("ns.create_ts")?),
			display_name: &ns.display_name().context("ns.display_name")?,
			version_id: &ns.version_id().context("ns.version_id")?,
		},
	)?;

	Ok(())
}

pub fn dashboard_url(game_id: &str, ns_id: &str) -> String {
	format!("https://rivet.gg/developer/games/{game_id}/namespaces/{ns_id}",)
}
