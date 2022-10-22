use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identity {
	pub custom_display_names: Vec<CustomDisplayName>,
	pub custom_avatars: Vec<CustomAvatar>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CustomDisplayName {
	Verbose { display_name: String },
	DisplayName(String),
}

impl CustomDisplayName {
	fn display_name(&self) -> &str {
		match self {
			Self::Verbose { display_name } => &display_name,
			Self::DisplayName(display_name) => &display_name,
		}
	}
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CustomAvatar {
	Verbose { upload_id: String },
	UploadId(String),
}

impl CustomAvatar {
	fn upload_id(&self) -> &str {
		match self {
			Self::Verbose { upload_id } => &upload_id,
			Self::UploadId(upload_id) => &upload_id,
		}
	}
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
					.display_name(custom_display_name.display_name())
					.build()
			})
			.collect::<Vec<_>>();
		let custom_avatars = self
			.custom_avatars
			.iter()
			.map(|custom_avatar| {
				CustomAvatar::builder()
					.upload_id(custom_avatar.upload_id())
					.build()
			})
			.collect::<Vec<_>>();

		Ok(IdentityVersionConfig::builder()
			.set_custom_display_names(Some(custom_display_names))
			.set_custom_avatars(Some(custom_avatars))
			.build())
	}
}
