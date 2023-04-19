use chrono::TimeZone;

pub fn date(date: &cli_core::rivet_cloud::types::DateTime) -> String {
	chrono::Local
		.timestamp_opt(date.secs(), date.subsec_nanos())
		.latest()
		.expect("failed to build timestamp")
		.format("%c")
		.to_string()
}
