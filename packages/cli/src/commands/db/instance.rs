use clap::Parser;
use clap::Subcommand;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::tasks;

use crate::util::task::run_task;
use crate::util::task::run_task_simple;
use crate::util::task::TaskOutputStyle;

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
		run_task_simple::<tasks::postgres::start::Task>(tasks::postgres::start::Input {}).await
	}
}

/// Stop the development database
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOpts {}

impl StopOpts {
	pub async fn execute(&self) -> ExitCode {
		run_task_simple::<tasks::postgres::stop::Task>(tasks::postgres::stop::Input {}).await
	}
}

/// Get the status of the development database
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusOpts {}

impl StatusOpts {
	pub async fn execute(&self) -> ExitCode {
		match run_task::<tasks::postgres::status::Task>(
			TaskOutputStyle::PlainNoResult,
			tasks::postgres::status::Input {},
		)
		.await
		{
			Result::Ok(output) => {
				println!("{}", output.status);
				ExitCode::SUCCESS
			}
			Err(e) => {
				eprintln!("Error: {e:?}");
				ExitCode::from(1)
			}
		}
	}
}

/// Delete the database's data.
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetOpts {}

impl ResetOpts {
	pub async fn execute(&self) -> ExitCode {
		run_task_simple::<tasks::postgres::reset::Task>(tasks::postgres::reset::Input {}).await
	}
}
