use serde::Deserialize;

pub mod cdn;
pub mod kv;
pub mod mm;

#[derive(Debug, Deserialize)]
pub struct Version {
	#[serde(default)]
	pub cdn: Option<cdn::Cdn>,
	#[serde(default)]
	pub matchmaker: Option<mm::Matchmaker>,
	#[serde(default)]
	pub kv: Option<kv::Kv>,
}
