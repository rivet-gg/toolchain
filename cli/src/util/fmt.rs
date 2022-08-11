use chrono::TimeZone;

pub fn date(date: &rivetctl::types::DateTime) -> String {
	chrono::Local
		.timestamp(date.secs(), date.subsec_nanos())
		.format("%c")
		.to_string()
}
