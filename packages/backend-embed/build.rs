use anyhow::*;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

fn main() -> Result<()> {
	let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

	// Get path to artifacts path
	let mut script_path = PathBuf::from(manifest_dir.clone());
	script_path.push("..");
	script_path.push("..");
	script_path.push("scripts");
	script_path.push("backend");
	script_path.push("build_artifacts.ts");

	// Run script
	let status = Command::new("deno")
		.arg("run")
		.arg("-A")
		.arg(&script_path)
		// shhh font awesome can suck it, we'll fix this soon (tm)
		.env("FONTAWESOME_PACKAGE_TOKEN","E7A94808-3467-4150-B90D-EABDAEB9E0B4")
		.status()?;
	if !status.success() {
		panic!("build artifacts failed");
	}
	println!("cargo:rerun-if-changed={}", script_path.display());

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
	let mut hasher = Sha256::new();

	for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
		let path = entry.path();
		if path.is_file() {
			let mut file = File::open(path)?;
			let mut buffer = [0; 1024];
			loop {
				let count = file.read(&mut buffer)?;
				if count == 0 {
					break;
				}
				hasher.update(&buffer[..count]);
			}
		}
	}

	let result = hasher.finalize();
	Ok(format!("{:x}", result))
}
