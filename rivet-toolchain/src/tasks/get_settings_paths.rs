use global_error::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{paths, util::task};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {
	pub project_path: PathBuf,
	pub user_path: PathBuf,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"get_settings_path"
	}

	async fn run(_task: task::TaskCtx, _input: Self::Input) -> GlobalResult<Self::Output> {
		Ok(Output {
			project_path: paths::project_settings_config_file()?,
			user_path: paths::user_settings_config_file()?,
		})
	}
}
