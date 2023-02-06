pub fn display_name_from_date() -> String {
	// HACK: We can't allow multiple whitespace in a row, unsure why Chrono does
	// this
	format!("{}", chrono::Utc::now().format("%c")).replace("  ", " ")
}
