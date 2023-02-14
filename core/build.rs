use anyhow::Result;
use vergen::{vergen, Config};

fn main() -> Result<()> {
	let mut config = Config::default();
	*config.git_mut().sha_kind_mut() = vergen::ShaKind::Both;
	vergen(config)
}
