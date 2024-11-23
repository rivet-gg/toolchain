use anyhow::*;
use serde::de::DeserializeOwned;

/// Parses a string like `foo=bar,hello=world` in to a Serde struct.
///
/// This uses `envy` under the hood. Refer to that for reference on behavior.
pub fn from_str<T: DeserializeOwned>(input: &str) -> Result<T> {
	let vars_iter = input
		.split(',')
		.map(|pair| pair.split_once('=').unwrap_or((&pair, "true")))
		.map(|(k, v)| (k.to_string(), v.to_string()));
	let output = envy::from_iter::<_, T>(vars_iter)?;
	Ok(output)
}
