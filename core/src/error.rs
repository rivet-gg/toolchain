#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("not authenticated")]
	NotAuthenticated,

	#[error("io: {source}")]
	Io {
		#[from]
		source: tokio::io::Error,
	},

	#[error("could not find home dir")]
	CouldNotFindHomeDir,

	#[error("invalid global config: {source}")]
	InvalidGlobalConfig { source: serde_json::Error },

	#[error("internal: {message}")]
	Internal { message: String },

	#[error("invalid agent kind")]
	InvalidAgentKind,

	#[error("region does not exist: {region_id}")]
	RegionDoesNotExist { region_id: String },

	#[error("inspect fail: {source}")]
	InspectFail {
		source: aws_smithy_client::SdkError<rivet_cloud::error::InspectError>,
	},

	#[error("config {key}: {message}")]
	Config { key: String, message: String },
}

impl Error {
	pub fn internal(message: impl ToString) -> Error {
		Error::Internal {
			message: message.to_string(),
		}
	}

	pub fn config(key: impl ToString, message: impl ToString) -> Error {
		Error::Config {
			key: key.to_string(),
			message: message.to_string(),
		}
	}
}
