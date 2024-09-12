use anyhow::*;
use std::path::PathBuf;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
	// Get path to artifacts path
	let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
	let mut script_path = PathBuf::from(manifest_dir);
	script_path.push("..");
	script_path.push("..");
	script_path.push("scripts");
	script_path.push("backend");
	script_path.push("build_artifacts.ts");

	// Run script
	let output = Command::new("deno")
		.arg("run")
		.arg("-A")
		.arg(&script_path)
		.output()?;
	if !output.status.success() {
		panic!("build artifacts failed");
	}

	Ok(())
}
