use cli_core::rivet_api::models;
use global_error::prelude::*;

pub fn version_display_name(game: &models::CloudGameFull) -> GlobalResult<String> {
	// Generate date
	//
	// Use UTC in order to ensure that the month is consistent if a team is collaborating from
	// multiple locations around the world with different time zones
	let now = chrono::Utc::now();
	let date_prefix = now.format("%Y.%m");

	// Find the max index, if exists
	let re = regex::Regex::new(&format!("{date_prefix} \\((\\d+)\\)")).unwrap();
	let mut max_index = 0;
	for version in &game.versions {
		if let Some(captures) = re.captures(&version.display_name) {
			let version_idx = unwrap!(captures.get(1)).as_str().parse::<i64>()?;
			max_index = max_index.max(version_idx);
		}
	}

	// Build new name
	let version_name = format!("{date_prefix} ({index})", index = max_index + 1);

	Ok(version_name)
}
