use clap::Parser;
use toolchain_core::rivet_api::apis;
use global_error::prelude::*;
use tabled::Tabled;

use crate::util::{term, upload};

#[derive(Parser)]
pub enum SubCommand {
	/// List all available identity avatars
	List,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &toolchain_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::List => {
				let custom_avatars_res = unwrap!(
					apis::cloud_games_avatars_api::cloud_games_avatars_list_game_custom_avatars(
						&ctx.openapi_config_cloud,
						&ctx.game_id
					)
					.await
				);

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
					.custom_avatars
					.iter()
					.map(|custom_avatar| {
						Ok(CustomAvatar {
							upload_id: custom_avatar.upload_id.to_string(),
							created: custom_avatar.create_ts.clone(),
							size: upload::format_file_size(custom_avatar.content_length as u64)?,
							url: custom_avatar
								.url
								.clone()
								.unwrap_or_else(|| "(Upload not finished)".to_string()),
						})
					})
					.collect::<GlobalResult<Vec<_>>>()?;

				term::table(&custom_avatars);

				Ok(())
			}
		}
	}
}
