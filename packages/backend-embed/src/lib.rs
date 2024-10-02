use anyhow::*;
use include_dir::{include_dir, Dir};
use std::path::PathBuf;
use tokio::fs;

const BACKEND_DIR: Dir = include_dir!("$BACKEND_PATH");
const BACKEND_HASH: &'static str = env!("BACKEND_HASH");

/// Return a path for the backend. If one does not exist, the backend dir will automatically be
/// extracted and executables will be set.
pub async fn backend_dir(data_dir: &PathBuf) -> Result<PathBuf> {
	// Create path to backend based on hash
	let backend_dir = data_dir.join("backend").join(BACKEND_HASH);

	// Write backend if does not exist
	if !backend_dir.exists() {
		fs::create_dir_all(&backend_dir).await?;
		tokio::task::block_in_place(|| BACKEND_DIR.extract(&backend_dir))?;

		// Update executables
		#[cfg(unix)]
		set_executables(&BACKEND_DIR, &backend_dir).await?;
	}

	Ok(backend_dir)
}

/// HACK: Make all binaries in `bin` folders executable. This is because
/// bundling the vendored folders strips permissions, so executables can't be ran.
#[cfg(unix)]
async fn set_executables(dir: &Dir<'_>, fs_path: &PathBuf) -> Result<()> {
	use include_dir::DirEntry;
	use std::os::unix::fs::PermissionsExt;

	for entry in dir.entries() {
		match entry {
			DirEntry::Dir(subdir) => {
				let file_name = subdir.path().file_name().unwrap_or_default();
				if file_name == "bin" || file_name == ".bin" {
					for file_entry in subdir.files() {
						let file_path = fs_path.join(file_entry.path());
						let metadata = fs::metadata(&file_path).await?;
						let mut perms = metadata.permissions();
						perms.set_mode(perms.mode() | 0o111);
						fs::set_permissions(file_path, perms).await?;
					}
				}

				Box::pin(set_executables(subdir, &fs_path)).await?;
			}
			DirEntry::File(_) => {} // Skip files at this level
		}
	}
	Ok(())
}
