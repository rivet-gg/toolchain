use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::json;
use std::path::Path;
use tokio::process::Command;
use uuid::Uuid;

use crate::util::{cmd, lz4, term};

use super::{BuildCompression, BuildKind};

pub async fn create_archive(
	image_tag: &str,
	build_kind: BuildKind,
	build_compression: BuildCompression,
) -> Result<tempfile::TempPath> {
	eprintln!();
	term::status::info(
		"Archiving Image",
		format!("({}, {})", build_kind.as_ref(), build_compression.as_ref()),
	);

	// Build archive
	let build_tar_path = match build_kind {
		BuildKind::DockerImage => archive_docker_image(&image_tag).await?,
		BuildKind::OciBundle => archive_oci_bundle(&image_tag).await?,
	};

	// Compress archive
	let compressed_path = compress_archive(build_tar_path.as_ref(), build_compression).await?;

	Ok(compressed_path)
}

/// Save Docker image
async fn archive_docker_image(image_tag: &str) -> Result<tempfile::TempPath> {
	let build_tar_path = tempfile::NamedTempFile::new()?.into_temp_path();

	let mut build_cmd = Command::new("docker");
	build_cmd
		.arg("save")
		.arg("--output")
		.arg(&build_tar_path)
		.arg(&image_tag);
	cmd::execute_docker_cmd(build_cmd, "Docker failed to save image").await?;

	Ok(build_tar_path)
}

/// Convert the Docker image to an OCI bundle
async fn archive_oci_bundle(image_tag: &str) -> Result<tempfile::TempPath> {
	let bundle_dir = tempfile::TempDir::new()?;

	// Create container and copy files to rootfs
	{
		let container_name = format!("rivet-game-{}", Uuid::new_v4());

		let mut create_cmd = Command::new("docker");
		create_cmd
			.arg("container")
			.arg("create")
			.arg("--name")
			.arg(&container_name)
			.arg(&image_tag);
		cmd::execute_docker_cmd_silent(create_cmd, "Docker failed to create container").await?;

		let mut cp_cmd = Command::new("docker");
		cp_cmd
			.arg("container")
			.arg("cp")
			.arg("--archive")
			.arg(format!("{container_name}:/"))
			.arg(bundle_dir.path().join("rootfs"));
		cmd::execute_docker_cmd_silent(cp_cmd, "Docker failed to copy files out of container")
			.await?;

		let mut rm_cmd = Command::new("docker");
		rm_cmd
			.arg("container")
			.arg("rm")
			.arg("--force")
			.arg(&container_name);
		cmd::execute_docker_cmd_silent(rm_cmd, "Docker failed to remove container").await?;
	}

	// Convert Docker image to OCI bundle
	//
	// See umoci implementation: https://github.com/opencontainers/umoci/blob/312b2db3028f823443d6a74d86b05f65701b0d0e/oci/config/convert/runtime.go#L183
	{
		#[derive(Deserialize)]
		#[serde(rename_all = "PascalCase")]
		struct DockerImage {
			config: DockerImageConfig,
		}

		#[derive(Deserialize)]
		#[serde(rename_all = "PascalCase")]
		struct DockerImageConfig {
			cmd: Option<Vec<String>>,
			entrypoint: Option<Vec<String>>,
			env: Vec<String>,
			user: String,
			#[serde(default)]
			working_dir: String,
		}

		// Inspect image
		let mut inspect_cmd = Command::new("docker");
		inspect_cmd.arg("image").arg("inspect").arg(&image_tag);
		let inspect_output = cmd::execute_docker_cmd_silent_failable(inspect_cmd).await?;
		let image = serde_json::from_slice::<Vec<DockerImage>>(&inspect_output.stdout)?;
		let image = image.into_iter().next().context("no image")?;

		// Read config
		let mut config = serde_json::from_slice::<serde_json::Value>(include_bytes!(
			"../../../static/oci-bundle-config.base.json"
		))?;

		// WORKDIR
		//
		// https://github.com/opencontainers/umoci/blob/312b2db3028f823443d6a74d86b05f65701b0d0e/oci/config/convert/runtime.go#L144
		if image.config.working_dir != "" {
			config["process"]["cwd"] = json!(image.config.working_dir);
		} else {
			config["process"]["cwd"] = json!("/");
		}

		// ENV
		//
		// https://github.com/opencontainers/umoci/blob/312b2db3028f823443d6a74d86b05f65701b0d0e/oci/config/convert/runtime.go#L149
		config["process"]["env"] = json!(image.config.env);

		// ENTRYPOINT + CMD
		//
		// https://github.com/opencontainers/umoci/blob/312b2db3028f823443d6a74d86b05f65701b0d0e/oci/config/convert/runtime.go#L157
		let args = std::iter::empty::<String>()
			.chain(image.config.entrypoint.into_iter().flatten())
			.chain(image.config.cmd.into_iter().flatten())
			.collect::<Vec<_>>();
		config["process"]["args"] = json!(args);

		// USER
		//
		// https://github.com/opencontainers/umoci/blob/312b2db3028f823443d6a74d86b05f65701b0d0e/oci/config/convert/runtime.go#L174
		//
		// Moby passwd parser: https://github.com/moby/sys/blob/c0711cde08c8fa33857a2c28721659267f49b5e2/user/user.go
		//
		// If you're you're the guy at Docker who decided to reimplement passwd in Go for funzies, please reconsider next time.
		{
			// Parse user
			let (user, group) = if let Some((u, g)) = image.config.user.split_once(":") {
				(u, Some(g))
			} else {
				(image.config.user.as_str(), None)
			};

			// Attempt to parse user to uid
			let user_int = user.parse::<u32>().ok();
			let group_int = group.and_then(|x| x.parse::<u32>().ok());

			// Parse passwd file and find user
			let users =
				crate::util::users::read_passwd_file(&bundle_dir.path().join("rootfs/etc/passwd"))?;
			let exec_user = users
				.iter()
				.find(|x| user_int.map_or(false, |uid| x.uid == uid) || x.name == user);

			// Determine uid
			let uid = if image.config.user.is_empty() {
				0
			} else if let Some(exec_user) = exec_user {
				exec_user.uid
			} else if let Some(uid) = user_int {
				uid
			} else {
				term::status::warn(
					"Cannot determine uid",
					format!(
						"{} not in passwd file, please specify a raw uid like `USER 1000:1000`",
						image.config.user
					),
				);
				0
			};

			// Parse group file and find group
			let groups =
				crate::util::users::read_group_file(&bundle_dir.path().join("rootfs/etc/group"))?;
			let exec_group = groups.iter().find(|x| {
				if let Some(group) = group {
					if let Some(gid) = group_int {
						return x.gid == gid;
					} else {
						x.name == group
					}
				} else if let Some(exec_user) = &exec_user {
					x.user_list.contains(&exec_user.name)
				} else {
					false
				}
			});

			// Determine gid
			let gid = if image.config.user.is_empty() {
				0
			} else if let Some(exec_group) = exec_group {
				exec_group.gid
			} else if let Some(gid) = group_int {
				gid
			} else {
				term::status::warn("Cannot determine gid", format!("{} not in group file, please specify a raw uid & gid like `USER 1000:1000`", image.config.user));

				0
			};

			// Validate not running as root
			//
			// See Kubernetes implementation https://github.com/kubernetes/kubernetes/blob/cea1d4e20b4a7886d8ff65f34c6d4f95efcb4742/pkg/kubelet/kuberuntime/security_context_others.go#L44C4-L44C4
			if std::env::var("_RIVET_OCI_BUNDLE_ALLOW_ROOT")
				.ok()
				.map_or(false, |x| &x == "1")
			{
				if uid == 0 {
					bail!("cannot run Docker container as root (i.e. uid 0) for security. see https://docs.docker.com/engine/reference/builder/#user")
				}
			}

			// Specify user
			config["process"]["user"]["uid"] = json!(uid);
			config["process"]["user"]["gid"] = json!(gid);

			// Add home if needed
			if let Some(home) = exec_user.as_ref().map(|x| x.home.as_str()) {
				if !home.is_empty() {
					config["process"]["env"]
						.as_array_mut()
						.unwrap()
						.push(json!(format!("HOME={home}")));
				}
			}
		}

		// Write config.json
		tokio::fs::write(
			bundle_dir.path().join("config.json"),
			serde_json::to_vec(&config)?,
		)
		.await?;
	}

	// Archive the bundle
	let build_tar_path = tempfile::NamedTempFile::new()?.into_temp_path();
	let mut archive_cmd = Command::new("tar");
	archive_cmd
		.arg("-cf")
		.arg(&build_tar_path)
		.arg(bundle_dir.path());
	cmd::error_for_output_failure(&archive_cmd.output().await?, "failed to archive oci bundle")?;

	Ok(build_tar_path)
}

async fn compress_archive(
	build_tar_path: &Path,
	compression: BuildCompression,
) -> Result<tempfile::TempPath> {
	// Compress the bundle
	let build_tar_compressed_file = tempfile::NamedTempFile::new()?;
	let build_tar_compressed_path = build_tar_compressed_file.into_temp_path();
	match compression {
		BuildCompression::None => {
			tokio::fs::rename(&build_tar_path, &build_tar_compressed_path).await?;
		}
		BuildCompression::Lz4 => {
			let build_tar_path = build_tar_path.to_owned();
			let build_tar_compressed_path = build_tar_compressed_path.to_owned();
			tokio::task::spawn_blocking(move || {
				lz4::compress(&build_tar_path, &build_tar_compressed_path)
			})
			.await??;
		}
	}

	Ok(build_tar_compressed_path)
}
