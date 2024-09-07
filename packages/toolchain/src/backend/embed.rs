use anyhow::*;
use include_dir::{include_dir, Dir};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use tokio::fs;

const BACKEND_DIR: Dir = include_dir!("packages/backend/");
const DENO_CONFIG: &'static str = include_str!("../../../../deno.jsonc");
const DENO_LOCKFILE: &'static str = include_str!("../../../../deno.lock");

/// Return a path for the backend. If one does not exist, the backend dir will automatically be
/// extracted.
pub async fn backend_dir() -> Result<PathBuf> {
	// Generate a hash of the included backend directory
	let mut hasher = DefaultHasher::new();
	for file in BACKEND_DIR.files() {
		file.path().hash(&mut hasher);
	}
	let backend_hash = format!("{:x}", hasher.finish());

	// Create path to backend base don hash
	let backend_dir = crate::paths::data_dir()?
		.join("backend")
		.join(format!("{backend_hash}"));

	// Write backend if does not exist
	if !backend_dir.exists() {
		fs::create_dir_all(&backend_dir).await?;
		tokio::task::block_in_place(|| BACKEND_DIR.extract(&backend_dir))?;
		fs::write(backend_dir.join("deno.jsonc"), DENO_CONFIG).await?;
		fs::write(backend_dir.join("deno.lock"), DENO_LOCKFILE).await?;
	}

	Ok(backend_dir)
}
