use tokio::io::{self, AsyncBufReadExt};

pub async fn wait_for_enter() -> io::Result<()> {
	let mut stdin = io::BufReader::new(io::stdin());
	let mut line = String::new();
	stdin.read_line(&mut line).await?;
	Ok(())
}
