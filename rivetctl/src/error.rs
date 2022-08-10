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
}
