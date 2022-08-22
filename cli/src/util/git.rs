use anyhow::Result;
use std::path::Path;
use tokio::process::Command;

pub async fn check_ignore(path: &Path) -> Result<bool> {
	let output = Command::new("git")
		.arg("check-ignore")
		.arg(path)
		.output()
		.await?;
	Ok(output.status.success())
}
