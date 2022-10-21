use anyhow::{Context, Result};
use clap::Parser;
use serde::Serialize;

use crate::util::{struct_fmt, term};

#[derive(Parser)]
pub enum SubCommand {
	Get {
		#[clap(long, value_parser)]
		format: struct_fmt::Format,
	},
	#[clap(alias = "dash")]
	Dashboard,
	Avatars {
		#[clap(subcommand)]
		subcmd: avatars::SubCommand,
	},
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Get { format } => {
				let game_res = ctx
					.client()
					.get_game_by_id()
					.game_id(&ctx.game_id)
					.send()
					.await
					.context("client.get_game_by_id")?;
				let game = game_res.game.context("game_res.game")?;
				let game_id = game.game_id().context("game.game_id")?;

				#[derive(Serialize)]
				struct Output<'a> {
					game_id: &'a str,
				}
				struct_fmt::print(format, &Output { game_id })?;

				Ok(())
			}
			SubCommand::Dashboard => {
				eprintln!("{}", term::link(dashboard_url(&ctx.game_id)));

				Ok(())
			}
			SubCommand::Avatars { subcmd } => match subcmd {
				avatars::SubCommand::List => subcmd.execute(&ctx).await,
			},
		}
	}
}

pub fn dashboard_url(game_id: &str) -> String {
	format!("https://rivet.gg/developer/games/{game_id}")
}

pub mod avatars {
	use anyhow::{Context, Result};
	use clap::Parser;
	use tabled::Tabled;

	use crate::util::{fmt, term, upload};

	#[derive(Parser)]
	pub enum SubCommand {
		List,
	}

	impl SubCommand {
		pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
			match self {
				SubCommand::List => {
					let custom_avatars_res = ctx
						.client()
						.list_game_custom_avatars()
						.game_id(&ctx.game_id)
						.send()
						.await
						.context("client.list_game_custom_avatars")?;

					#[derive(Tabled)]
					struct CustomAvatar {
						#[tabled(rename = "Url")]
						url: String,
						#[tabled(rename = "Created")]
						created: String,
						#[tabled(rename = "Size")]
						size: String,
					}

					let custom_avatars = custom_avatars_res
						.custom_avatars()
						.context("custom_avatars_res.custom_avatars")?
						.iter()
						.map(|custom_avatar| {
							Ok(CustomAvatar {
								url: custom_avatar
									.url()
									.context("custom_avatar.url")?
									.to_string(),
								created: fmt::date(
									custom_avatar
										.create_ts()
										.context("custom_avatar.create_ts")?,
								),
								size: upload::format_file_size(
									custom_avatar
										.content_length()
										.context("custom_avatar.content_length")? as u64,
								)?,
							})
						})
						.collect::<Result<Vec<_>>>()?;

					term::table(&custom_avatars);

					Ok(())
				}
			}
		}
	}
}
