use clap::Parser;
use clap::Subcommand;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::tasks::postgres_start;
use toolchain::tasks::postgres_status;
use toolchain::tasks::postgres_stop;

use crate::util::task::run_task_simple;

/// Manage the development database
#[derive(Subcommand)]
pub enum SubCommand {
	Reset(ResetOpts),
	Start(StartOpts),
	Status(StatusOpts),
	Stop(StopOpts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match &self {
			SubCommand::Reset(opts) => opts.execute().await,
			SubCommand::Start(opts) => opts.execute().await,
			SubCommand::Status(opts) => opts.execute().await,
			SubCommand::Stop(opts) => opts.execute().await,
		}
	}
}

/// Start the development database
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartOpts {}

impl StartOpts {
	pub async fn execute(&self) -> ExitCode {
		run_task_simple::<postgres_start::Task>(postgres_start::Input {}).await
	}
}

/// Stop the development database
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOpts {}

impl StopOpts {
	pub async fn execute(&self) -> ExitCode {
		run_task_simple::<postgres_stop::Task>(postgres_stop::Input {}).await
	}
}

/// Get the status of the development database
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusOpts {}

impl StatusOpts {
	pub async fn execute(&self) -> ExitCode {
		run_task_simple::<postgres_status::Task>(postgres_status::Input {}).await
	}
}

/// Delete the database's data.
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetOpts {}

impl ResetOpts {
	pub async fn execute(&self) -> ExitCode {
		run_task_simple::<postgres_reset::Task>(postgres_reset::Input {}).await
	}
}
