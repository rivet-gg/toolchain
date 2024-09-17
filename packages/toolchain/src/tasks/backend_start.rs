use anyhow::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{path::Path, time::Duration};

use crate::{
	backend::{self, build_backend_command, build_backend_command_raw},
	config::{self, meta},
	paths,
	util::{
		process_manager::{CommandOpts, StartMode, StartOpts},
		task::{self, backend_config_update},
	},
};

#[derive(Deserialize)]
pub struct Input {
	pub start_mode: StartMode,
}

#[derive(Serialize)]
pub struct Output {
	pub exit_code: Option<i32>,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"backend_start"
	}

	async fn run(task: task::TaskCtx, input: Self::Input) -> Result<Self::Output> {
		// Set backend port in case the process is already running. This will result in a duplicate
		// port dispatch if the backend is not running or crashed.
		if let (Some(backend_port), Some(editor_port)) =
			meta::read_project(&paths::data_dir()?, |x| (x.backend_port, x.editor_port)).await?
		{
			task.event(task::TaskEvent::PortUpdate {
				backend_port,
				editor_port,
			});
		}

		// Start or hook to backend
		let task_inner = task.clone();
		let pm_fut = backend::PROCESS_MANAGER_DEV.start(
			StartOpts {
				task: task.clone(),
				start_mode: input.start_mode,
				base_data_dir: paths::data_dir()?,
			},
			|| async move {
				// Pick dev port
				let backend_port = portpicker::pick_unused_port().context("no free ports")?;
				let editor_port = portpicker::pick_unused_port().context("no free ports")?;
				meta::mutate_project(&paths::data_dir()?, |x| {
					x.backend_port = Some(backend_port);
					x.editor_port = Some(editor_port);
				})
				.await?;

				// Build env
				let (mut cmd_env, config_path) =
					config::settings::try_read(&paths::data_dir()?, |settings| {
						let mut env = settings.backend.command_environment.clone();
						env.extend(settings.backend.dev.command_environment.clone());
						Ok((env, settings.backend.dev.config_path.clone()))
					})
					.await?;
				cmd_env.insert("RIVET_BACKEND_PORT".into(), backend_port.to_string());
				cmd_env.insert("RIVET_BACKEND_HOSTNAME".into(), "0.0.0.0".to_string());
				cmd_env.insert("RIVET_BACKEND_TERM_COLOR".into(), "never".into());
				cmd_env.insert("RIVET_EDITOR_PORT".into(), editor_port.to_string());

				// Build command
				let cmd = build_backend_command_raw(backend::BackendCommandOpts {
					command: "dev",
					opts: serde_json::json!({
						"project": config_path,
						"nonInteractive": true
					}),
					env: cmd_env,
				})
				.await?;

				// Publish commandevent
				task_inner.event(task::TaskEvent::PortUpdate {
					backend_port,
					editor_port,
				});

				Ok(CommandOpts {
					command: cmd.command.display().to_string(),
					args: cmd.args,
					envs: cmd.envs.into_iter().collect(),
					current_dir: cmd.current_dir.display().to_string(),
				})
			},
		);

		// Poll for config file updates
		let poll_config_fut = poll_config_file(task.clone());

		// Wait futures
		let exit_code = tokio::select! {
			res = pm_fut => {
				res?
			}
			res = poll_config_fut => {
				res?;
				bail!("poll_config_file exited unexpectedly");
			}
		};

		Ok(Output { exit_code })
	}
}

async fn poll_config_file(task_ctx: task::TaskCtx) -> Result<()> {
	// Read meta path from backend
	let mut interval = tokio::time::interval(Duration::from_secs(2));
	let meta_output = loop {
		interval.tick().await;

		let output = build_backend_command(backend::BackendCommandOpts {
			command: "configManifestPath",
			opts: json!({
				"project": null
			}),
			env: Default::default(),
		})
		.await?
		.output()
		.await?;

		if output.status.success() {
			break output;
		}
	};

	// Parse and validate meta path
	let meta_path = String::from_utf8(meta_output.stdout)?;
	let meta_path = meta_path.trim();
	ensure!(
		!meta_path.contains("\n"),
		"Expected exactly one line of output, got:\n{meta_path:?}"
	);
	let meta_path = Path::new(meta_path);

	// Poll the file for updates
	//
	// We do this instead of using a file watcher since file watchers are frequently broken across
	// platform and will require extensive testing.
	let mut interval = tokio::time::interval(Duration::from_secs(2));
	let mut last_file_modified = None;
	let mut last_editor_port = None;
	loop {
		interval.tick().await;

		// Check for file change
		let editor_port = if let Some(editor_port) =
			meta::read_project(&paths::data_dir()?, |x| x.editor_port).await?
		{
			editor_port
		} else {
			// The editor port has not been chosen yet.
			continue;
		};

		// Check for file change
		let file_modified = match tokio::fs::metadata(&meta_path).await {
			Result::Ok(metadata) => match metadata.modified() {
				Result::Ok(x) => x,
				Err(err) => {
					task_ctx.log(format!("Failed to read file modification time: {err}"));
					continue;
				}
			},
			Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
				// Config file does not exist yet. The backend likely has not written it.
				continue;
			}
			Err(err) => {
				task_ctx.log(format!("Failed to read file metadata: {err}"));
				continue;
			}
		};

		// Publish event if changed
		let updated_file_modified = last_file_modified.map_or(true, |x| file_modified > x);
		let updated_editor_port = last_editor_port.map_or(true, |x| x != editor_port);
		if updated_file_modified || updated_editor_port {
			last_file_modified = Some(file_modified);
			last_editor_port = Some(editor_port);

			match read_meta_and_build_event(meta_path, editor_port).await {
				Result::Ok(event) => {
					task_ctx.event(task::TaskEvent::BackendConfigUpdate(event));
				}
				Err(err) => task_ctx.log(format!("Failed to read backend meta: {err}")),
			}
		}
	}
}

/// Partial serde struct representing data we need to read from `meta.json`.
///
/// See packages/backend/toolchain/build/meta.ts
mod backend_meta {
	use serde::Deserialize;
	use std::collections::HashMap;

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct Meta {
		pub modules: HashMap<String, Module>,
	}

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct Module {
		pub config: ModuleConfig,
	}

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct ModuleConfig {
		pub name: String,
	}
}

/// Reads the meta.json from the filesystem and converts it to an event.
///
/// Uses this intermediate step to convert the data in the toolchain instead of passing the direct
/// manifest to the plugin in order to:
/// - Ensure a consistent format
/// - Reduce overhead of updates (the config is massive)
/// - Enhance with any toolchain-specific data (e.g. edtor config url)
async fn read_meta_and_build_event(
	config_path: impl AsRef<Path>,
	editor_port: u16,
) -> Result<backend_config_update::Event> {
	// Read meta
	let meta = tokio::task::block_in_place(|| {
		let file = std::fs::File::open(config_path)?;
		let meta = serde_json::from_reader::<_, backend_meta::Meta>(&file)?;
		Ok(meta)
	})?;

	// Convert to event
	let mut modules = meta
		.modules
		.into_iter()
		.map(|(slug, module)| backend_config_update::Module {
			slug: slug.clone(),
			name: module.config.name,
			config_url: format!("http://127.0.0.1:{editor_port}/#{slug}"),
			docs_url: format!("https://rivet.gg/modules/{slug}"),
		})
		.collect::<Vec<_>>();
	modules.sort_by_cached_key(|x| x.name.clone());

	Ok(backend_config_update::Event { modules })
}
