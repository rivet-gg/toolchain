use anyhow::*;
use fd_lock::RwLock;
use reqwest;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::PathBuf;
use zip::ZipArchive;

// Mirror new Deno versions with `./scripts/deno/mirror_release.ts`
pub const DEFAULT_VERSION: &'static str = "1.46.1";

pub async fn get_or_download_default_executable(data_dir: &PathBuf) -> Result<DenoExecutable> {
	get_or_download_executable(DEFAULT_VERSION, data_dir).await
}

pub struct DenoExecutable {
	pub executable_path: PathBuf,
	pub fresh_download: bool,
}

pub async fn get_or_download_executable(
	version: &str,
	data_dir: &PathBuf,
) -> Result<DenoExecutable> {
	let executable_name = if cfg!(windows) { "deno.exe" } else { "deno" };
	let executable_path = data_dir.join("deno").join(version).join(executable_name);

	if tokio::fs::metadata(&executable_path).await.is_ok() {
		return Ok(DenoExecutable {
			executable_path,
			fresh_download: false,
		});
	}

	// Lock file
	eprintln!("[Deno] Waiting for download lockfile");
	let lock_path = data_dir.join("deno_download.lock");
	let mut lock = tokio::task::block_in_place(|| -> Result<_> {
		Result::Ok(RwLock::new(std::fs::File::create(lock_path)?))
	})?;
	let _writer = tokio::task::block_in_place(|| lock.write())?;

	// Check again if the executable exists in case another instance or thread completed the download
	if tokio::fs::metadata(&executable_path).await.is_ok() {
		return Ok(DenoExecutable {
			executable_path,
			fresh_download: false,
		});
	}

	// Get download file name
	let arch = std::env::consts::ARCH;
	let platform = std::env::consts::OS;
	let download_file = match (arch, platform) {
		("aarch64", "macos") => "deno-aarch64-apple-darwin.zip",
		("aarch64", "linux") => "deno-aarch64-unknown-linux-gnu.zip",
		("x86_64", "macos") => "deno-x86_64-apple-darwin.zip",
		("x86_64", "windows") => "deno-x86_64-pc-windows-msvc.zip",
		("x86_64", "linux") => "deno-x86_64-unknown-linux-gnu.zip",
		_ => {
			bail!(
				"unsupported architecture or platform: {} {}",
				arch,
				platform
			);
		}
	};

	// Download file
	let temp_dir = tempfile::tempdir()?;
	let zip_path = temp_dir.path().join("deno.zip");

	let download_url = format!(
		"https://releases.rivet.gg/deno/{}/{}",
		version, download_file
	);
	eprintln!("[Deno] Downloading release ({download_url})");
	let response = reqwest::get(&download_url).await?.error_for_status()?;
	let mut file = File::create(&zip_path)?;
	let mut content = response.bytes().await?;
	file.write_all(&mut content)?;

	// Extract ZIP
	eprintln!("[Deno] Extracting archive");
	let extract_dir = data_dir.join("deno").join(version);
	fs::create_dir_all(&extract_dir)?;

	let zip_file = File::open(&zip_path)?;
	let mut archive = ZipArchive::new(BufReader::new(zip_file))?;

	for i in 0..archive.len() {
		let mut file = archive.by_index(i)?;
		if file.name().ends_with(executable_name) {
			let outpath = extract_dir.join(executable_name);
			let mut outfile = File::create(&outpath)?;
			std::io::copy(&mut file, &mut outfile)?;

			#[cfg(unix)]
			{
				use std::os::unix::fs::PermissionsExt;
				let mut perms = fs::metadata(&outpath)?.permissions();
				perms.set_mode(0o755);
				fs::set_permissions(&outpath, perms)?;
			}

			break;
		}
	}

	Ok(DenoExecutable {
		executable_path,
		fresh_download: true,
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;
	use std::process::Command;

	#[tokio::test(flavor = "multi_thread")]
	async fn test_get_or_download_executable() {
		let version = DEFAULT_VERSION;

		// Create a temporary directory for testing
		let temp_dir = tempfile::tempdir().unwrap();
		let data_dir = temp_dir.path().to_path_buf();

		// Call the function to download the executable
		let deno_executable = get_or_download_executable(&version, &data_dir)
			.await
			.unwrap();

		// Assert that the executable exists at the expected path
		assert!(deno_executable.executable_path.exists());

		// Assert that the download is fresh
		assert!(deno_executable.fresh_download);

		// Run the Deno executable to check the version
		let output = Command::new(&deno_executable.executable_path)
			.arg("--version")
			.output()
			.expect("Failed to execute Deno");

		// Assert that the version output matches the expected format
		assert!(String::from_utf8_lossy(&output.stdout).contains(&format!("deno {version}")));

		// Clean up the temporary directory
		fs::remove_dir_all(data_dir).unwrap();
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn test_parallel_downloads() {
		let version = DEFAULT_VERSION;

		// Create a temporary directory for testing
		let temp_dir = tempfile::tempdir().unwrap();
		let data_dir = temp_dir.path().to_path_buf();

		// Create a JoinSet to spawn and manage tasks
		let mut join_set = tokio::task::JoinSet::new();

		// Spawn multiple tasks to download the executable concurrently
		let num_tasks = 5;
		for _ in 0..num_tasks {
			let data_dir_clone = data_dir.clone();
			join_set
				.spawn(async move { get_or_download_executable(&version, &data_dir_clone).await });
		}

		// Wait for all tasks to complete and collect the results
		let mut fresh_download_count = 0;
		while let Some(result) = join_set.join_next().await {
			let deno_executable = result.unwrap().unwrap();
			assert!(deno_executable.executable_path.exists());
			if deno_executable.fresh_download {
				fresh_download_count += 1;
			}
		}

		// Assert that only one executable was downloaded
		assert_eq!(fresh_download_count, 1);

		// Clean up the temporary directory
		fs::remove_dir_all(data_dir).unwrap();
	}
}
