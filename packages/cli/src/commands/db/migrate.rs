pub mod apply;
pub mod drop;
pub mod generate;
pub mod push;

use clap::{Parser, Subcommand};
use serde::Serialize;
use std::process::ExitCode;
use toolchain::backend::run_backend_command_passthrough;

use crate::util::global_opts::GlobalOpts;

/// Manage changes to the database schema
#[derive(Subcommand)]
pub enum SubCommand {
	Apply(ApplyOpts),
	Push(PushOpts),
	Generate(GenerateOpts),
	Drop(DropOpts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		let Ok(_) = postgres::ensure_running().await else {
			return ExitCode::FAILURE;
		};

		match &self {
			SubCommand::Apply(opts) => opts.execute().await,
			SubCommand::Push(opts) => opts.execute().await,
			SubCommand::Generate(opts) => opts.execute().await,
			SubCommand::Drop(opts) => opts.execute().await,
		}
	}
}

/// Apply pre-generated migrations to a module
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyOpts {
	#[clap(flatten)]
	#[serde(flatten)]
	global: GlobalOpts,

	pub modules: Vec<String>,
}

impl ApplyOpts {
	pub async fn execute(&self) -> ExitCode {
		run_backend_command_passthrough("db/migrate/apply.ts", self).await
	}
}

/// Delete a database
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropOpts {
	#[clap(flatten)]
	#[serde(flatten)]
	global: GlobalOpts,

	pub modules: Vec<String>,
}

impl DropOpts {
	pub async fn execute(&self) -> ExitCode {
		run_backend_command_passthrough("db/migrate/drop.ts", self).await
	}
}

/// Generate migrations for a module
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateOpts {
	#[clap(flatten)]
	#[serde(flatten)]
	global: GlobalOpts,

	pub modules: Vec<String>,
}

impl GenerateOpts {
	pub async fn execute(&self) -> ExitCode {
		run_backend_command_passthrough("db/migrate/generate.ts", self).await
	}
}

/// Push a schema to the database without migrations
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PushOpts {
	#[clap(flatten)]
	#[serde(flatten)]
	global: GlobalOpts,

	pub modules: Vec<String>,
}

impl PushOpts {
	pub async fn execute(&self) -> ExitCode {
		run_backend_command_passthrough("db/migrate/push.ts", self).await
	}
}
