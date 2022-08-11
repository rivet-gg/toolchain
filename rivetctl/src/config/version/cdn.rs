use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Cdn {
	pub site: String,
}
