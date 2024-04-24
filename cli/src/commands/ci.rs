use clap::Parser;
use cli_core::rivet_api::models;
use global_error::prelude::*;
use std::collections::HashMap;
use tokio::fs;

use crate::{commands::config, util::paths};

#[derive(Parser)]
pub enum SubCommand {
	/// Generates a workflow
	Generate {
		#[clap(subcommand)]
		command: GenerateOpts,
	},
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		match self {
			SubCommand::Generate { command } => command.execute(ctx).await,
		}
	}
}

#[derive(Parser)]
pub enum GenerateOpts {
	#[clap(name = "github")]
	GitHub(GenerateGitHubOpts),
}

impl GenerateOpts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		match self {
			GenerateOpts::GitHub(opts) => gen_github(ctx, opts).await,
		}
	}
}

#[derive(Parser)]
pub struct GenerateGitHubOpts {
	/// Path to generate the workflow file at
	#[clap(long)]
	path: Option<String>,

	#[clap(long)]
	allow_idle_lobbies: bool,
}

async fn gen_github(ctx: &cli_core::Ctx, opts: &GenerateGitHubOpts) -> GlobalResult<()> {
	let relative_path = opts
		.path
		.clone()
		.unwrap_or_else(|| ".github/workflows/rivet-deploy.yml".to_string());
	let path = paths::project_root()?.join(relative_path);
	let path_parent = unwrap!(path.parent(), "path has no parent");

	// Create parent path for workflow
	fs::create_dir_all(&path_parent).await?;

	// Generate workflow file
	let workflow = gen_github_workflow(ctx, opts).await?;

	// Write workflow file
	fs::write(&path, workflow).await?;

	rivet_term::status::success("Config written", path.display());

	Ok(())
}

// TODO: Strings in this workflow are not appropriately escaped
async fn gen_github_workflow(
	ctx: &cli_core::Ctx,
	opts: &GenerateGitHubOpts,
) -> GlobalResult<String> {
	let mut version = config::read_config(Vec::new(), None).await?;

	let mut workflow = String::new();

	workflow.push_str(&common_substitute(
		ctx,
		include_str!("../../tpl/ci/github/head.yml"),
	));

	let mut needs_jobs = Vec::new(); // Job dependencies for final deploy step
	let mut overrides = Vec::new(); // Overrides to pass to the `rivet deploy` command

	// Image jobs
	let mut existing_dockerfiles = HashMap::<String, String>::new(); // Map of Dockerfile path to job name
	if let Some(matchmaker) = version.matchmaker.as_mut() {
		if !opts.allow_idle_lobbies {
			validate_idle_lobbies(&matchmaker.idle_lobbies)?;
		}

		if let Some(docker) = matchmaker.docker.as_mut() {
			if docker.image_id.is_none() && docker.image.is_none() {
				append_dockerfile(
					ctx,
					&mut workflow,
					&mut needs_jobs,
					&mut overrides,
					"matchmaker",
					docker.dockerfile.as_ref().map(String::as_str),
					&mut existing_dockerfiles,
				)?;
			}
		}

		if let Some(game_modes) = matchmaker.game_modes.as_mut() {
			for (game_mode_id, game_mode) in game_modes.iter_mut() {
				if !opts.allow_idle_lobbies {
					validate_idle_lobbies(&game_mode.idle_lobbies)?;
				}

				if let Some(docker) = game_mode.docker.as_mut() {
					if docker.image_id.is_none() && docker.image.is_none() {
						append_dockerfile(
							ctx,
							&mut workflow,
							&mut needs_jobs,
							&mut overrides,
							&format!("matchmaker.{game_mode_id}"),
							docker.dockerfile.as_ref().map(String::as_str),
							&mut existing_dockerfiles,
						)?;
					}
				}
			}
		}
	}

	// CDN job
	if let Some(cdn) = &version.cdn {
		if cdn.site_id.is_none() {
			needs_jobs.push("build_cdn".to_string());
			overrides.push(format!(
				"cdn.site_id=\"${{{{ needs.build_cdn.outputs.site_id }}}}\""
			));
			let build_command =
				unwrap!(cdn.build_command.as_ref(), "cdn.build_command is required");
			let build_output = unwrap!(cdn.build_output.as_ref(), "cdn.build_output is required");

			let cdn_job = common_substitute(ctx, include_str!("../../tpl/ci/github/job-cdn.yml"))
				.replace("__BUILD_COMMAND__", build_command)
				.replace("__BUILD_OUTPUT__", build_output);
			workflow.push_str(&cdn_job);
		}
	}

	// Foot
	let needs_jobs = needs_jobs.join(", ");
	let overrides = overrides
		.iter()
		.map(|x| format!("--override '{x}'"))
		.collect::<Vec<_>>()
		.join(" ");
	let foot = common_substitute(ctx, include_str!("../../tpl/ci/github/foot.yml"))
		.replace("__NEEDS_JOBS__", &needs_jobs)
		.replace("__DEPLOY_OVERRIDES__", &overrides);
	workflow.push_str(&foot);

	Ok(workflow)
}

fn common_substitute(ctx: &cli_core::Ctx, input: &str) -> String {
	input
		.replace(
			"__RIVET_CLI_VERSION__",
			&format!("v{}", env!("VERGEN_BUILD_SEMVER")),
		)
		.replace("__RIVET_API_ENDPOINT__", &ctx.api_endpoint)
}

/// Validate that idle lobbies are not used in CI workflows
fn validate_idle_lobbies(
	idle_lobbies: &Option<Box<models::CloudVersionMatchmakerGameModeIdleLobbiesConfig>>,
) -> GlobalResult<()> {
	if let Some(il) = idle_lobbies {
		ensure!(
			il.min == 0,
			"creating ci workflows with idle lobbies will spawn unsued lobbies, rerun with --allow-idle-lobbies to ignore"
		);
	}

	Ok(())
}

fn append_dockerfile(
	ctx: &cli_core::Ctx,
	workflow: &mut String,
	needs_jobs: &mut Vec<String>,
	overrides: &mut Vec<String>,
	config_path: &str,
	dockerfile_path: Option<&str>,
	existing_dockerfiles: &mut HashMap<String, String>,
) -> GlobalResult<()> {
	let dockerfile_path = dockerfile_path.unwrap_or("Dockerfile");

	// Build Dockerfile. Deduplicate build commands for the same Dockerfile.
	let job_name = if let Some(x) = existing_dockerfiles.get(dockerfile_path) {
		x.to_string()
	} else {
		let job_name = format!("build_job_{}", config_path.replace(".", "-"));
		needs_jobs.push(job_name.clone());
		existing_dockerfiles.insert(dockerfile_path.to_string(), job_name.clone());

		// TODO: Specify build path
		let image_job = common_substitute(ctx, include_str!("../../tpl/ci/github/job-image.yml"))
			.replace("__JOB_NAME__", &job_name)
			.replace("__DOCKERFILE_PATH__", dockerfile_path);
		workflow.push_str(&image_job);

		job_name
	};

	// Add override for Dockerfile
	overrides.push(format!(
		"{config_path}.docker.image_id=\"${{{{ needs.{job_name}.outputs.image_id }}}}\""
	));

	Ok(())
}
