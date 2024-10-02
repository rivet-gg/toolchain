pub mod build;
pub mod gen_openapi;
pub mod gen_sdk;
pub mod get_current_version;
pub mod get_endpoint;
pub mod show;

use clap::Subcommand;
use std::process::ExitCode;

/// Manage the backend
#[derive(Subcommand)]
pub enum SubCommand {
	Build(build::Opts),
	Show(show::Opts),
	#[clap(name = "generate-openapi")]
	GenerateOpenApi(gen_openapi::Opts),
	#[clap(name = "generate-sdk")]
	GenerateSdk(gen_sdk::Opts),
	#[clap(name = "get-endpoint")]
	GetEndpoint(get_endpoint::Opts),
	#[clap(name = "get-current-version")]
	GetCurrentVersion(get_current_version::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::Build(opts) => opts.execute().await,
			SubCommand::Show(opts) => opts.execute().await,
			SubCommand::GenerateOpenApi(opts) => opts.execute().await,
			SubCommand::GenerateSdk(opts) => opts.execute().await,
			SubCommand::GetEndpoint(opts) => opts.execute().await,
			SubCommand::GetCurrentVersion(opts) => opts.execute().await,
		}
	}
}
