use anyhow::*;
use merkle_hash::MerkleTree;
use std::path::{Path, PathBuf};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
	let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
	let out_dir = std::env::var("OUT_DIR")?;

	let mut script_path = PathBuf::from(manifest_dir.clone());
	script_path.push("..");
	script_path.push("..");
	script_path.push("scripts");
	script_path.push("backend");
	script_path.push("build_artifacts.ts");

	let mut backend_path = PathBuf::from(manifest_dir.clone());
	backend_path.push("..");
	backend_path.push("backend");

	// Install deno
	let deno_dir = Path::new(&out_dir).join("deno");
	let deno_exec = rivet_deno_embed::get_executable(&deno_dir).await?;

	// Run script
	let status = Command::new(&deno_exec.executable_path)
		.arg("run")
		.arg("-A")
		.arg(&script_path)
		// shhh font awesome can suck it, we'll fix this soon (tm)
		.env(
			"FONTAWESOME_PACKAGE_TOKEN",
			"E7A94808-3467-4150-B90D-EABDAEB9E0B4",
		)
		.status()?;
	if !status.success() {
		panic!("build artifacts failed");
	}
	println!("cargo:rerun-if-changed={}", script_path.display());

	// Format
	let status = Command::new(&deno_exec.executable_path)
		.arg("task")
		.arg("format")
		.current_dir(&backend_path)
		.status()?;
	if !status.success() {
		panic!("format failed");
	}

	// Cache dependencies
	let status = Command::new(&deno_exec.executable_path)
		.arg("task")
		.arg("cache")
		.current_dir(&backend_path)
		.status()?;
	if !status.success() {
		panic!("cache dependencies failed");
	}

	// Check backend
	let status = Command::new(&deno_exec.executable_path)
		.arg("task")
		.arg("check")
		.current_dir(&backend_path)
		.status()?;
	if !status.success() {
		panic!("check files failed");
	}

	// Hash backend
	let mut backend_path = PathBuf::from(manifest_dir);
	backend_path.pop();
	backend_path.push("backend");
	println!(
		"cargo:rustc-env=BACKEND_HASH={}",
		hash_directory(&backend_path)?
	);

	// TODO: Add back OGBEE-129
	// println!("cargo:rerun-if-changed={}", backend_path.display());

	Ok(())
}

fn hash_directory<P: AsRef<Path>>(path: P) -> Result<String> {
	let tree = MerkleTree::builder(&path.as_ref().display().to_string()).build()?;
	let hash = tree
		.root
		.item
		.hash
		.iter()
		.map(|b| format!("{:02x}", b))
		.collect::<Vec<String>>()
		.join("");
	Ok(hash)
}
