use anyhow::*;
use lazy_static::lazy_static;
use postgresql_embedded::{PostgreSQL, Settings, Status, VersionReq};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::Mutex};

use crate::paths;

const DEFAULT_POSTGRES_VERSION: &str = "=16.4.0";

lazy_static! {
	/// Holds the Postgres managers for each data dir.
	static ref MANAGER_REGISTRY: Mutex<HashMap<PathBuf, Arc<PostgresManager>>> = Mutex::new(HashMap::new());
}

pub async fn get(data_dir: &PathBuf) -> Result<Arc<PostgresManager>> {
	let mut registry = MANAGER_REGISTRY.lock().await;

	if let Some(manager) = registry.get(data_dir) {
		Ok(manager.clone())
	} else {
		let manager = PostgresManager::new(data_dir).await?;
		registry.insert(data_dir.clone(), manager.clone());
		Ok(manager)
	}
}

/// Helper function to get & start postgres.
pub async fn get_and_start(data_dir: &PathBuf) -> Result<Arc<PostgresManager>> {
	let postgres = get(data_dir).await?;
	postgres.start().await?;
	Ok(postgres)
}

pub struct PostgresManager {
	data_dir: PathBuf,
	postgresql: Mutex<PostgreSQL>,
}

impl PostgresManager {
	async fn new(data_dir: &PathBuf) -> Result<Arc<Self>> {
		// Read the port if the Postgres server is already running. If the port changes, the server
		// will not be accessible.
		let state = read_state(data_dir).await?;

		let mut settings = Settings::new();
		settings.version = VersionReq::parse(DEFAULT_POSTGRES_VERSION).unwrap();
		settings.installation_dir = paths::postgres_install_dir(data_dir)?;
		settings.host = "127.0.0.1".into();
		if let Some(port) = state.port {
			settings.port = port;
		}
		settings.temporary = false;
		settings.password_file = paths::postgres_password_file(data_dir)?;
		settings.data_dir = paths::postgres_data_dir(data_dir)?;
		if let Some(password) = state.password {
			settings.password = password;
		}

		let postgresql = PostgreSQL::new(settings);

		Ok(Arc::new(Self {
			data_dir: data_dir.clone(),
			postgresql: Mutex::new(postgresql),
		}))
	}

	pub async fn start(&self) -> Result<()> {
		// Ensure data dir exists
		tokio::fs::create_dir_all(paths::postgres_base(&self.data_dir)?).await?;

		// This is idempotent
		let mut postgresql = self.postgresql.lock().await;
		postgresql.setup().await.context("PostgreSQL::setup")?;

		// Otherwise, this will kill existing processes.
		if is_running(&postgresql).await {
			// Start Postgres
			postgresql.start().await.context("PostgreSQL::start")?;

			// Write new state
			let port = postgresql.settings().port;
			let password = postgresql.settings().password.clone();
			ensure!(port != 0, "postgres port still 0");
			mutate_state(&self.data_dir, |x| {
				x.port = Some(port);
				x.password = Some(password);
				Result::Ok(())
			})
			.await?;
		}

		Ok(())
	}

	pub async fn stop(&self) -> Result<()> {
		let postgresql = self.postgresql.lock().await;
		if is_running(&postgresql).await {
			postgresql.stop().await.context("PostgreSQL::stop")?;
		}
		Ok(())
	}

	pub async fn reset(&self) -> Result<()> {
		// Stop database
		self.stop().await?;

		// Delete data dir
		let pg_data_dir = paths::postgres_data_dir(&self.data_dir)?;
		tokio::fs::remove_dir_all(&pg_data_dir).await?;

		Ok(())
	}

	pub async fn status(&self) -> Result<Status> {
		let postgresql = self.postgresql.lock().await;
		Ok(postgresql.status())
	}

	pub async fn bin_dir(&self) -> PathBuf {
		self.postgresql.lock().await.settings().binary_dir()
	}

	pub async fn url(&self, database_name: &str) -> String {
		self.postgresql.lock().await.settings().url(database_name)
	}
}

/// Checks if Postgres is running.
///
/// Adds an extra check if the port is connectable to cover the edge case where the process was
/// force killed (or the system restarted).
async fn is_running(postgresql: &PostgreSQL) -> bool {
	postgresql.status() != Status::Started
		|| !probe_tcp_addr(
			postgresql.settings().host.as_str(),
			postgresql.settings().port,
		)
		.await
}

/// Checks if can connect to a TCP addr.
async fn probe_tcp_addr(host: &str, port: u16) -> bool {
	// If port has not been chosen, fail immediately
	if port == 0 {
		return false;
	}

	let connect_future = TcpStream::connect((host, port));
	let timeout_duration = Duration::from_secs(1);

	match tokio::time::timeout(timeout_duration, connect_future).await {
		Result::Ok(Result::Ok(_)) => true,
		_ => false,
	}
}

#[derive(Serialize, Deserialize, Default)]
struct PostgresState {
	port: Option<u16>,
	password: Option<String>,
}

async fn read_state(data_dir: &PathBuf) -> Result<PostgresState> {
	let state_path = paths::postgres_state_file(data_dir)?;
	let state = match tokio::fs::read(&state_path).await {
		Result::Ok(data) => serde_json::from_slice(&data).context("parse postgres state")?,
		Err(_) => PostgresState::default(),
	};
	Ok(state)
}

/// Reads the state from the file system, passes it to a callback, and writes the changes back.
///
/// We do this instead of storing it in memory since it will change if multiple processes are
/// accessing this state.
async fn mutate_state<F, T>(data_dir: &PathBuf, cb: F) -> Result<T>
where
	F: FnOnce(&mut PostgresState) -> Result<T>,
{
	let state_path = paths::postgres_state_file(data_dir)?;
	let mut state = read_state(data_dir).await?;

	let res = cb(&mut state)?;

	let state_json = serde_json::to_string(&state)?;
	tokio::fs::create_dir_all(paths::postgres_base(data_dir)?).await?;
	tokio::fs::write(&state_path, state_json).await?;

	Ok(res)
}
