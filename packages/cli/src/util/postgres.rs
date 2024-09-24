use std::sync::Arc;

use toolchain::{paths, postgres};

pub async fn ensure_running() -> Result<Arc<postgres::PostgresManager>, ()> {
	let data_dir = match paths::data_dir() {
		Ok(x) => x,
		Err(err) => {
			eprintln!("Failed to get data dir: {err:?}");
			return Err(());
		}
	};
	match postgres::get_and_start(&data_dir).await {
		Ok(x) => Ok(x),
		Err(err) => {
			eprintln!("Failed to get Postgres: {err:?}");
			Err(())
		}
	}
}
