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
					#[tabled(rename = "Upload ID")]
					upload_id: String,
					#[tabled(rename = "URL")]
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
							upload_id: custom_avatar
								.upload_id()
								.context("custom_avatar.upload_id")?
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
							url: custom_avatar
								.url()
								.context("custom_avatar.url")?
								.to_string(),
						})
					})
					.collect::<Result<Vec<_>>>()?;

				term::table(&custom_avatars);

				Ok(())
			}
		}
	}
}
