use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identity {
	pub custom_display_names: Vec<CustomDisplayName>,
	pub custom_avatars: Vec<CustomAvatar>,
}

#[derive(Debug, Deserialize)]
pub struct CustomDisplayName {
	display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CustomAvatar {
	upload_id: String,
}

impl Identity {
	pub fn build_model(
		&self,
		_game: &rivet_cloud::model::GameFull,
	) -> Result<rivet_cloud::model::IdentityVersionConfig, Error> {
		use rivet_cloud::model::*;

		let custom_display_names = self
			.custom_display_names
			.iter()
			.map(|custom_display_name| {
				CustomDisplayName::builder()
					.display_name(&custom_display_name.display_name)
					.build()
			})
			.collect::<Vec<_>>();
		let custom_avatars = self
			.custom_avatars
			.iter()
			.map(|custom_avatar| {
				CustomAvatar::builder()
					.upload_id(&custom_avatar.upload_id)
					.build()
			})
			.collect::<Vec<_>>();

		Ok(IdentityVersionConfig::builder()
			.set_custom_display_names(Some(custom_display_names))
			.set_custom_avatars(Some(custom_avatars))
			.build())
	}
}
