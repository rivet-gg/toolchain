use clap::Parser;
use cli_core::{ctx, rivet_api::apis};
use global_error::prelude::*;
use serde::Serialize;

use crate::commands::config::parse_config_override_args;
use crate::commands::deploy::build_and_push_compat;
use crate::commands::deploy::deploy;
use crate::util::global_config;
use crate::util::struct_fmt;

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {
	/// Name of the version to create
	#[clap(long = "name", alias = "display-name")]
	display_name: Option<String>,

	/// Override specific properties of the config
	#[clap(long = "override", short)]
	overrides: Vec<String>,

	/// Namespace ID to deploy to
	#[clap(short = 'n', long)]
	namespace: Option<String>,

	/// Number of files to upload in parallel
	#[clap(long, env = "RIVET_CONCURRENT_UPLOADS", default_value = "8")]
	concurrent_uploads: usize,

	/// Deprecated.
	///
	/// The build tag to upload
	#[clap(hide = true, long)]
	build_tag: Option<String>,

	/// Deprecated.
	///
	/// The name to assign to the build
	#[clap(hide = true, long)]
	build_name: Option<String>,

	/// Deprecated.
	///
	/// The path to the site directory to upload
	#[clap(hide = true, long)]
	site_path: Option<String>,

	/// Deprecated.
	///
	/// The name of the site that will be created
	#[clap(hide = true, long)]
	site_name: Option<String>,

	#[clap(long, value_parser)]
	format: Option<struct_fmt::Format>,
}

#[derive(Serialize)]
pub struct Output {}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<Output> {
		// Parse overrides
		let mut overrides = parse_config_override_args(&self.overrides)?;

		// Build & push site & build before creating version
		build_and_push_compat(
			ctx,
			&mut overrides,
			&self.build_tag,
			&self.build_name,
			&self.site_path,
			&self.site_name,
			self.concurrent_uploads,
			&self.format,
		)
		.await?;

		// Create version
		let output = deploy(
			ctx,
			self.display_name.as_ref().map(String::as_str),
			overrides,
			self.namespace.as_ref().map(String::as_str),
			self.concurrent_uploads,
			self.format.as_ref(),
		)
		.await?;
		struct_fmt::print_opt(self.format.as_ref(), &output)?;

		Ok(Output {})
	}
}
