use super::{util::get_namespace_url, SideKickHandler};
use clap::Parser;
use global_error::prelude::*;
use serde::Serialize;

#[derive(Parser)]
pub struct Opts {
	/// The namespace to get the link for
	#[structopt(short, long)]
	namespace: String,
}

#[derive(Serialize)]
pub struct Output {
	pub output: String,
}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self, ctx: &toolchain_core::Ctx) -> GlobalResult<Output> {
		let mut url = get_namespace_url(self.namespace.clone(), ctx).await?;
		url.path_segments_mut().unwrap().push("lobbies");
		Ok(Output {
			output: url.to_string(),
		})
	}
}
