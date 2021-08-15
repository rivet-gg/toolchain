#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("not authenticated")]
	NotAuthenticated,

	#[error("io: {source}")]
	Io {
		#[from]
		source: tokio::io::Error,
	},
}
