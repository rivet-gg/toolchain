use sha2::{Digest, Sha256};
use std::{env, fs::File, io::Read, path::PathBuf, process::Command};

const NIGHTLY_VERSION: &str = "nightly-2024-09-17";

fn main() {
	let target = env::var("OVERRIDE_TARGET").unwrap_or_else(|_| env::var("TARGET").unwrap());
	let profile = "embed";

	// Install the specific nightly version
	let install_output = Command::new("rustup")
		.arg("toolchain")
		.arg("install")
		.arg(NIGHTLY_VERSION)
		.output()
		.expect("Failed to install nightly toolchain");

	if !install_output.status.success() {
		eprintln!("Failed to install nightly toolchain: {}", NIGHTLY_VERSION);
		std::process::exit(1);
	}

	let install_target_output = Command::new("rustup")
		.arg("target")
		.arg("add")
		.arg(&target)
		.arg("--toolchain")
		.arg(NIGHTLY_VERSION)
		.output()
		.expect("Failed to install nightly toolchain");

	if !install_target_output.status.success() {
		eprintln!("Failed to install target: {}", NIGHTLY_VERSION);
		std::process::exit(1);
	}

	let install_component_output = Command::new("rustup")
		.arg("component")
		.arg("add")
		.arg("rust-src")
		.arg("--toolchain")
		.arg(NIGHTLY_VERSION)
		.output()
		.expect("Failed to install nightly toolchain");

	if !install_component_output.status.success() {
		eprintln!("Failed to install rust src component: {}", NIGHTLY_VERSION);
		std::process::exit(1);
	}

	// Set the target directory to a different target since it shares the same
	// package as the current project. This way, it won't deadlock when they're
	// both trying to lock the target dir.
	let target_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("process-runner-target");

	// Build the process-runner package using the specific nightly version
	//
	// See workaround for `env_remove`:
	// https://github.com/sagiegurari/cargo-make/pull/1060
	let status = Command::new("rustup")
		.arg("run")
		.arg(NIGHTLY_VERSION)
		.arg("cargo")
		.arg("build")
		.arg("--profile")
		.arg(profile)
		.arg("--package")
		.arg("rivet-process-runner")
		.arg("--target")
		.arg(&target)
		.arg("--target-dir")
		.arg(&target_dir)
		.arg("-Z")
		.arg("build-std=std,panic_abort")
		.arg("-Z")
		.arg("build-std-features=optimize_for_size,panic_immediate_abort")
		.env_remove("CARGO")
		.env_remove("RUSTC")
		.env_remove("RUSTDOC")
		.env_remove("RUSTFLAGS")
		.status()
		.expect("failed to build process-runner package");

	assert!(
		status.success(),
		"Building process-runner package failed"
	);

	// Output binary path
	let binary_name = if target.contains("windows") {
		"rivet-process-runner.exe"
	} else {
		"rivet-process-runner"
	};
	let binary_path = target_dir.join(target).join(profile).join(binary_name);
	println!(
		"cargo:rustc-env=PROCESS_RUNNER_BINARY_PATH={}",
		binary_path.display()
	);

	// Calculate and output SHA256 hash of the binary
	let mut file = File::open(&binary_path).expect("Failed to open binary file");
	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer)
		.expect("Failed to read binary file");

	let mut hasher = Sha256::new();
	hasher.update(&buffer);
	let hash = format!("{:x}", hasher.finalize());

	println!("cargo:rustc-env=PROCESS_RUNNER_BINARY_HASH={}", hash);

	// Output rebuild on change
	println!("cargo:rerun-if-changed=../process-runner");
}
