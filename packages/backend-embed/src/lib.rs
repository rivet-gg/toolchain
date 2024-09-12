use anyhow::*;
use include_dir::{include_dir, Dir};
use std::path::PathBuf;
use tokio::fs;

const BACKEND_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../backend");
const BACKEND_HASH: &'static str = env!("BACKEND_HASH");

/// Return a path for the backend. If one does not exist, the backend dir will automatically be
/// extracted.
pub async fn backend_dir(data_dir: &PathBuf) -> Result<PathBuf> {
	// Create path to backend base don hash
	let backend_dir = data_dir.join("backend").join(BACKEND_HASH);

	// Write backend if does not exist
	if !backend_dir.exists() {
		fs::create_dir_all(&backend_dir).await?;
		tokio::task::block_in_place(|| BACKEND_DIR.extract(&backend_dir))?;
	}

	Ok(backend_dir)
}
