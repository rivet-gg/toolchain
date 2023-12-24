use cli_core::ctx;
use global_error::prelude::*;
use serde_json::json;
use sysinfo::System;
use tokio::{
	sync::{Mutex, OnceCell},
	task::JoinSet,
	time::Duration,
};

use crate::util::{cmd, global_config};

pub static JOIN_SET: OnceCell<Mutex<JoinSet<()>>> = OnceCell::const_new();
pub static GAME_ID: OnceCell<String> = OnceCell::const_new();

/// Get the global join set for telemetry futures.
async fn join_set() -> &'static Mutex<JoinSet<()>> {
	JOIN_SET
		.get_or_init(|| async { Mutex::new(JoinSet::new()) })
		.await
}

/// Waits for all telemetry events to finish.
pub async fn wait_all() {
	let mut join_set = join_set().await.lock().await;
	match tokio::time::timeout(Duration::from_secs(5), async move {
		while join_set.join_next().await.is_some() {}
	})
	.await
	{
		Ok(_) => {}
		Err(_) => {
			println!("Timed out waiting for telemetry to finish. If your network blocks outgoing connections to our telemetry servers, see `rivet --help` on how to disable telemetry.")
		}
	}
}

// This API key is safe to hardcode. It will not change and is intended to be public.
const POSTHOG_API_KEY: &str = "phc_P6XQOd4QdSPhvgSj7ywfwhvZolwgdkfa6G7ytcqNLTU";

fn build_client() -> async_posthog::Client {
	async_posthog::client(POSTHOG_API_KEY)
}

/// Builds a new PostHog event with associated data.
///
/// This is slightly expensive, so it should not be used frequently.
pub async fn capture_event<F: FnOnce(&mut async_posthog::Event) -> GlobalResult<()>>(
	game_id: Option<&String>,
	name: &str,
	mutate: Option<F>,
) -> GlobalResult<()> {
	let api_endpoint = global_config::read_project(|x| x.cluster.api_endpoint.clone())
		.await?
		.unwrap_or_else(|| ctx::DEFAULT_API_ENDPOINT.to_string());
	let telemetry_disabled = global_config::read_project(|x| x.telemetry.disabled).await?;
	let args = std::env::args().collect::<Vec<_>>();

	let distinct_id = if let Some(game_id) = game_id {
		format!("game:{game_id}")
	} else {
		"game:uninitiated".to_string()
	};

	if telemetry_disabled {
		return Ok(());
	}

	let mut event = async_posthog::Event::new(name, &distinct_id);

	// Helps us understand what version of the CLI is being used.
	let version = json!({
		"git_sha": env!("VERGEN_GIT_SHA"),
		"git_branch": env!("VERGEN_GIT_BRANCH"),
		"build_semver": env!("VERGEN_BUILD_SEMVER"),
		"build_timestamp": env!("VERGEN_BUILD_TIMESTAMP"),
		"build_target": env!("VERGEN_CARGO_TARGET_TRIPLE"),
		"build_profile": env!("VERGEN_CARGO_PROFILE"),
		"rustc_version": env!("VERGEN_RUSTC_SEMVER")
	});

	// Add properties
	if let Some(game_id) = game_id {
		event.insert_prop(
			"$groups",
			&json!({
				"game_id": game_id,
			}),
		)?;
	}

	event.insert_prop(
		"$set",
		&json!({
			"game_id": game_id,
			"api_endpoint": api_endpoint,
			"version": version,
			"sys": {
				"name": System::name(),
				"kernel_version": System::kernel_version(),
				"os_version": System::os_version(),
				"host_name": System::host_name(),
				"cpu_arch": System::cpu_arch(),
			},
		}),
	)?;

	event.insert_prop("api_endpoint", api_endpoint)?;
	event.insert_prop("args", args)?;

	// Customize the event properties
	if let Some(mutate) = mutate {
		mutate(&mut event)?;
	}

	// Capture event
	join_set().await.lock().await.spawn(async move {
		match build_client().capture(event).await {
			Ok(_) => {}
			Err(_) => {
				// Fail silently
			}
		}
	});

	Ok(())
}
