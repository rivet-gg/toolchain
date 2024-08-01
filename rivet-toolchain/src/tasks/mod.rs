pub mod backend_dev;
pub mod backend_sdk_gen;
pub mod check_login_state;
pub mod deploy;
pub mod exec_command;
pub mod get_bootstrap_data;
pub mod get_hub_link;
pub mod get_settings_paths;
pub mod open;
pub mod start_device_link;
pub mod unlink;
pub mod wait_for_login;

pub use crate::util::task::RunConfig;

use global_error::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use std::future::Future;

use crate::util::task::{run_task, TaskCtx};

pub struct TaskConfig {
	pub prefer_multithreaded: bool,
}

impl TaskConfig {
	pub const fn default_const() -> Self {
		Self {
			prefer_multithreaded: false,
		}
	}
}

pub trait Task {
	type Input: DeserializeOwned;
	type Output: Serialize;

	const CONFIG: TaskConfig = TaskConfig::default_const();

	fn name() -> &'static str;
	fn run(task: TaskCtx, input: Self::Input) -> impl Future<Output = GlobalResult<Self::Output>>;
}

/// Used to run tasks with raw input/output string. This is useful for binding tasks to non-Rust
/// environments, such as raw dylibs or odd engines.
macro_rules! gen_run_task {
    ( $( $task:ty ),* $(,)? ) => {
        pub async fn run_task_json(run_config: RunConfig, name: &str, input_json: &str) -> String {
            $(
                if name == <$task>::name() {
                    let input = serde_json::from_str::<<$task as Task>::Input>(&input_json)
                        .expect("deserialize task input");
                    let output = run_task::<$task>(run_config, input).await;
                    let output_serialize = output.map_err(|x| x.to_string());
                    return serde_json::to_string(&output_serialize).expect("serialize task output");
                }
            )*

            panic!("unknown task {name}")
        }

        pub fn get_task_config(name: &str) -> &'static TaskConfig {
            $(
                if name == <$task>::name() {
                    return &<$task>::CONFIG;
                }
            )*

            panic!("unknown task {name}")
        }
    };
}

gen_run_task!(
	backend_dev::Task,
	backend_sdk_gen::Task,
	check_login_state::Task,
	deploy::Task,
	exec_command::Task,
	get_bootstrap_data::Task,
	get_hub_link::Task,
	open::Task,
	start_device_link::Task,
	unlink::Task,
	wait_for_login::Task,
	get_settings_paths::Task,
);
