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

	#[error("inspect fail: {source}")]
	InspectFail {
		source: aws_smithy_client::SdkError<rivet_cloud::error::InspectError>,
	},
}
