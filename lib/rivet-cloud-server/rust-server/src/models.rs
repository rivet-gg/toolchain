#[allow(unused_imports)]
use chrono;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// A service performance summary.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SvcPerf {
	/// The name of the service.
	pub svc_name: std::string::String,
	/// RFC3339 timestamp.
	pub ts: chrono::DateTime<chrono::Utc>,
	/// Unsigned 64 bit integer.
	pub duration: i64,
	/// A universally unique identifier.
	pub req_id: std::option::Option<std::string::String>,
	/// A list of performance spans.
	pub spans: std::vec::Vec<LogsPerfSpan>,
	/// A list of performance marks.
	pub marks: std::vec::Vec<LogsPerfMark>,
}

/// A performance mark.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogsPerfMark {
	/// The label given to this performance mark.
	pub label: std::string::String,
	/// RFC3339 timestamp.
	pub ts: chrono::DateTime<chrono::Utc>,
	/// A universally unique identifier.
	pub ray_id: std::option::Option<std::string::String>,
	/// A universally unique identifier.
	pub req_id: std::option::Option<std::string::String>,
}

/// A performance span.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogsPerfSpan {
	/// The label given to this performance span.
	pub label: std::string::String,
	/// RFC3339 timestamp.
	pub start_ts: chrono::DateTime<chrono::Utc>,
	/// RFC3339 timestamp.
	pub finish_ts: std::option::Option<chrono::DateTime<chrono::Utc>>,
	/// A universally unique identifier.
	pub req_id: std::option::Option<std::string::String>,
}

/// An error given by failed content validation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidationError {
	/// A list of strings denoting the origin of a validation error.
	pub path: std::vec::Vec<std::string::String>,
}

/// A group's billing invoice.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupBillingInvoice {
	/// A URL to this invoice's CSV breakdown.
	pub csv_url: std::string::String,
	/// A URL to this invoice's PDF document.
	pub pdf_url: std::string::String,
	/// RFC3339 timestamp.
	pub period_start_ts: chrono::DateTime<chrono::Utc>,
	/// RFC3339 timestamp.
	pub period_end_ts: chrono::DateTime<chrono::Utc>,
}

/// A group's billing transfer.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupBillingTransfer {
	/// Payment amount (in hundreths USD, 100 = $1.00).
	pub amount: i64,
	/// A description of this transfer.
	pub description: std::option::Option<std::string::String>,
	/// RFC3339 timestamp.
	pub created_ts: chrono::DateTime<chrono::Utc>,
	/// A value denoting the status of a billing transfer.
	pub status: GroupBillingStatus,
}

/// A value denoting the status of a billing transfer.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupBillingStatus {
	#[allow(missing_docs)] // documentation missing in model
	Processing,
	#[allow(missing_docs)] // documentation missing in model
	Refunded,
	#[allow(missing_docs)] // documentation missing in model
	Succeeded,
	/// Unknown contains new variants that have been added since this code was generated.
	Unknown(String),
}
impl std::convert::From<&str> for GroupBillingStatus {
	fn from(s: &str) -> Self {
		match s {
			"processing" => GroupBillingStatus::Processing,
			"refunded" => GroupBillingStatus::Refunded,
			"succeeded" => GroupBillingStatus::Succeeded,
			other => GroupBillingStatus::Unknown(other.to_owned()),
		}
	}
}
impl std::str::FromStr for GroupBillingStatus {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		Ok(GroupBillingStatus::from(s))
	}
}
impl GroupBillingStatus {
	/// Returns the `&str` value of the enum member.
	pub fn as_str(&self) -> &str {
		match self {
			GroupBillingStatus::Processing => "processing",
			GroupBillingStatus::Refunded => "refunded",
			GroupBillingStatus::Succeeded => "succeeded",
			GroupBillingStatus::Unknown(s) => s.as_ref(),
		}
	}
	/// Returns all the `&str` values of the enum members.
	pub fn values() -> &'static [&'static str] {
		&["processing", "refunded", "succeeded"]
	}
}
impl AsRef<str> for GroupBillingStatus {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

/// A group's billing payment.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupBillingPayment {
	/// Payment amount (in hundreths USD, 100 = $1.00).
	pub amount: i64,
	/// A description of this payment.
	pub description: std::option::Option<std::string::String>,
	/// Whether or not this payment is from an invoice.
	pub from_invoice: bool,
	/// RFC3339 timestamp.
	pub created_ts: chrono::DateTime<chrono::Utc>,
	/// A value denoting the status of a billing transfer.
	pub status: GroupBillingStatus,
}

/// A region summary.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegionSummary {
	/// A universally unique identifier.
	pub region_id: std::string::String,
	/// The server provider of this region.
	pub provider: std::string::String,
	/// A universal number given to this region.
	pub universal_region: i16,
	/// Represent a resource's readable display name.
	pub provider_display_name: std::string::String,
	/// Represent a resource's readable display name.
	pub region_display_name: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupBankSource {
	/// The bank account number of this group's bank source.
	pub account_number: std::string::String,
	/// The bank routing number of this group's bank source.
	pub routing_number: std::string::String,
}

/// A group billing summary.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupBillingSummary {
	/// A list of multiple game lobby expenses.
	pub games: std::vec::Vec<GameLobbyExpenses>,
	/// A group's available balance.
	pub balance: i64,
}

/// Game lobby expenses.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameLobbyExpenses {
	/// A game handle.
	pub game: GameHandle,
	/// A list of namespace summaries.
	pub namespaces: std::vec::Vec<NamespaceSummary>,
	/// A list of multiple region tier expenses.
	pub expenses: std::vec::Vec<RegionTierExpenses>,
}

/// Region tier expenses.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegionTierExpenses {
	/// A universally unique identifier.
	pub namespace_id: std::string::String,
	/// A universally unique identifier.
	pub region_id: std::string::String,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub tier_name_id: std::string::String,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub lobby_group_name_id: std::string::String,
	/// How long a region tier has been active (in milliseconds).
	pub uptime: i64,
	/// Amount of expenses for this region tier (in hundred-thousandths USD, 100,000 = $1.00).
	pub expenses: i64,
}

/// A namespace summary.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NamespaceSummary {
	/// A universally unique identifier.
	pub namespace_id: std::string::String,
	/// RFC3339 timestamp.
	pub create_ts: chrono::DateTime<chrono::Utc>,
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// A universally unique identifier.
	pub version_id: std::string::String,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub name_id: std::string::String,
}

/// A game handle.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameHandle {
	/// A universally unique identifier.
	pub game_id: std::string::String,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub name_id: std::string::String,
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// The URL of this game's logo image.
	pub logo_url: std::option::Option<std::string::String>,
	/// The URL of this game's banner image.
	pub banner_url: std::option::Option<std::string::String>,
}

/// A region server tier.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegionTier {
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub tier_name_id: std::string::String,
	/// Together with the denominator, denotes the portion of the CPU a given server uses.
	pub rivet_cores_numerator: i32,
	/// Together with the numerator, denotes the portion of the CPU a given server uses.
	pub rivet_cores_denominator: i32,
	/// CPU frequency (MHz).
	pub cpu: i64,
	/// Allocated memory (MB).
	pub memory: i64,
	/// Allocated disk space (MB).
	pub disk: i64,
	/// Internet bandwidth (MB).
	pub bandwidth: i64,
	/// Price billed for every second this server is running (in quadrillionth USD, 1,000,000,000,000 = $1.00).
	pub price_per_second: i64,
}

/// A presigned request used to upload files. Upload your file to the given URL via a PUT request.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UploadPresignedRequest {
	/// The name of the file to upload. This is the same as the one given in the upload prepare file.
	pub path: std::string::String,
	/// The URL of the presigned request for which to upload your file to.
	pub url: std::string::String,
}

/// A file being prepared to upload.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UploadPrepareFile {
	/// The path/filename of the file.
	pub path: std::string::String,
	/// The MIME type of the file.
	pub content_type: std::option::Option<std::string::String>,
	/// Unsigned 64 bit integer.
	pub content_length: i64,
}

/// A CDN site summary.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CdnSiteSummary {
	/// A universally unique identifier.
	pub site_id: std::string::String,
	/// A universally unique identifier.
	pub upload_id: std::string::String,
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// RFC3339 timestamp.
	pub create_ts: chrono::DateTime<chrono::Utc>,
	/// Unsigned 64 bit integer.
	pub content_length: i64,
	/// Whether or not this site has completely been uploaded.
	pub complete: bool,
}

/// A build summary.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildSummary {
	/// A universally unique identifier.
	pub build_id: std::string::String,
	/// A universally unique identifier.
	pub upload_id: std::string::String,
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// RFC3339 timestamp.
	pub create_ts: chrono::DateTime<chrono::Utc>,
	/// Unsigned 64 bit integer.
	pub content_length: i64,
	/// Whether or not this build has completely been uploaded.
	pub complete: bool,
}

/// Metrics relating to a job service.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SvcMetrics {
	/// The job name.
	pub job: std::string::String,
	/// CPU metrics.
	pub cpu: std::vec::Vec<f32>,
	/// Memory metrics.
	pub memory: std::vec::Vec<i64>,
	/// Peak memory metrics.
	pub memory_max: std::vec::Vec<i64>,
	/// Total allocated memory (MB).
	pub allocated_memory: i64,
}

/// A logs summary for a lobby.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogsLobbySummary {
	/// A universally unique identifier.
	pub lobby_id: std::string::String,
	/// A universally unique identifier.
	pub namespace_id: std::string::String,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub lobby_group_name_id: std::string::String,
	/// A universally unique identifier.
	pub region_id: std::string::String,
	/// RFC3339 timestamp.
	pub create_ts: chrono::DateTime<chrono::Utc>,
	/// A union representing the state of a lobby.
	pub status: LogsLobbyStatus,
}

/// A union representing the state of a lobby.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LogsLobbyStatus {
	/// A running lobby.
	Running(Unit),
	/// The status of a stopped lobby.
	Stopped(LogsLobbyStatusStopped),
}

/// The status of a stopped lobby.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogsLobbyStatusStopped {
	/// RFC3339 timestamp.
	pub stop_ts: chrono::DateTime<chrono::Utc>,
	/// Whether or not the lobby failed or stopped successfully.
	pub failed: bool,
	/// The exit code returned by the lobby's main process when stopped.
	pub exit_code: i32,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unit {}

/// Analyical information about a lobby.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnalyticsLobbySummary {
	/// A universally unique identifier.
	pub lobby_id: std::string::String,
	/// A universally unique identifier.
	pub lobby_group_id: std::string::String,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub lobby_group_name_id: std::string::String,
	/// A universally unique identifier.
	pub region_id: std::string::String,
	/// RFC3339 timestamp.
	pub create_ts: chrono::DateTime<chrono::Utc>,
	/// Whether or not this lobby is ready.
	pub is_ready: bool,
	/// Whether or not this lobby is idle.
	pub is_idle: bool,
	/// Whether or not this lobby is in a closed state.
	pub is_closed: bool,
	/// Whether or not this lobby is outdated.
	pub is_outdated: bool,
	/// Unsigned 32 bit integer.
	pub max_players_normal: i32,
	/// Unsigned 32 bit integer.
	pub max_players_direct: i32,
	/// Unsigned 32 bit integer.
	pub max_players_party: i32,
	/// Unsigned 32 bit integer.
	pub total_player_count: i32,
	/// Unsigned 32 bit integer.
	pub registered_player_count: i32,
}

/// A value denoting what type of authentication to use for a game namespace's CDN.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CdnAuthType {
	#[allow(missing_docs)] // documentation missing in model
	Basic,
	#[allow(missing_docs)] // documentation missing in model
	None,
	/// Unknown contains new variants that have been added since this code was generated.
	Unknown(String),
}
impl std::convert::From<&str> for CdnAuthType {
	fn from(s: &str) -> Self {
		match s {
			"basic" => CdnAuthType::Basic,
			"none" => CdnAuthType::None,
			other => CdnAuthType::Unknown(other.to_owned()),
		}
	}
}
impl std::str::FromStr for CdnAuthType {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		Ok(CdnAuthType::from(s))
	}
}
impl CdnAuthType {
	/// Returns the `&str` value of the enum member.
	pub fn as_str(&self) -> &str {
		match self {
			CdnAuthType::Basic => "basic",
			CdnAuthType::None => "none",
			CdnAuthType::Unknown(s) => s.as_ref(),
		}
	}
	/// Returns all the `&str` values of the enum members.
	pub fn values() -> &'static [&'static str] {
		&["basic", "none"]
	}
}
impl AsRef<str> for CdnAuthType {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

/// A docker port.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LobbyGroupRuntimeDockerPort {
	/// The label of this docker port.
	pub label: std::string::String,
	/// The port number to connect to.
	pub target_port: std::option::Option<i32>,
	/// The port range to connect to for UDP.
	pub port_range: std::option::Option<PortRange>,
	/// A proxy protocol.
	pub proxy_protocol: ProxyProtocol,
}

/// A proxy protocol.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyProtocol {
	#[allow(missing_docs)] // documentation missing in model
	Http,
	#[allow(missing_docs)] // documentation missing in model
	Https,
	#[allow(missing_docs)] // documentation missing in model
	Udp,
	/// Unknown contains new variants that have been added since this code was generated.
	Unknown(String),
}
impl std::convert::From<&str> for ProxyProtocol {
	fn from(s: &str) -> Self {
		match s {
			"http" => ProxyProtocol::Http,
			"https" => ProxyProtocol::Https,
			"udp" => ProxyProtocol::Udp,
			other => ProxyProtocol::Unknown(other.to_owned()),
		}
	}
}
impl std::str::FromStr for ProxyProtocol {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		Ok(ProxyProtocol::from(s))
	}
}
impl ProxyProtocol {
	/// Returns the `&str` value of the enum member.
	pub fn as_str(&self) -> &str {
		match self {
			ProxyProtocol::Http => "http",
			ProxyProtocol::Https => "https",
			ProxyProtocol::Udp => "udp",
			ProxyProtocol::Unknown(s) => s.as_ref(),
		}
	}
	/// Returns all the `&str` values of the enum members.
	pub fn values() -> &'static [&'static str] {
		&["http", "https", "udp"]
	}
}
impl AsRef<str> for ProxyProtocol {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

/// Range of ports that can be connected to.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortRange {
	/// Unsigned 32 bit integer.
	pub min: i32,
	/// Unsigned 32 bit integer.
	pub max: i32,
}

/// A full namespace.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NamespaceFull {
	/// A universally unique identifier.
	pub namespace_id: std::string::String,
	/// RFC3339 timestamp.
	pub create_ts: chrono::DateTime<chrono::Utc>,
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// A universally unique identifier.
	pub version_id: std::string::String,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub name_id: std::string::String,
	/// Cloud configuration for a given namespace.
	pub config: CloudNamespaceConfig,
}

/// Cloud configuration for a given namespace.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudNamespaceConfig {
	/// CDN configuration for a given namespace.
	pub cdn: CdnNamespaceConfig,
	/// Matchmaker configuration for a given namespace.
	pub matchmaker: MatchmakerNamespaceConfig,
	/// KV configuration for a given namespace.
	pub kv: KvNamespaceConfig,
}

/// KV configuration for a given namespace.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvNamespaceConfig {}

/// Matchmaker configuration for a given namespace.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MatchmakerNamespaceConfig {
	/// Unsigned 32 bit integer.
	pub lobby_count_max: i32,
	/// Unsigned 32 bit integer.
	pub max_players_per_client: i32,
	/// Unsigned 32 bit integer.
	pub max_players_per_client_vpn: i32,
	/// Unsigned 32 bit integer.
	pub max_players_per_client_proxy: i32,
	/// Unsigned 32 bit integer.
	pub max_players_per_client_tor: i32,
	/// Unsigned 32 bit integer.
	pub max_players_per_client_hosting: i32,
}

/// CDN configuration for a given namespace.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CdnNamespaceConfig {
	/// Whether or not to allow users to connect to the given namespace via domain name.
	pub enable_domain_public_auth: bool,
	/// A list of CDN domains for a given namespace.
	pub domains: std::vec::Vec<CdnNamespaceDomain>,
	/// A value denoting what type of authentication to use for a game namespace's CDN.
	pub auth_type: CdnAuthType,
	/// A list of CDN authenticated users for a given namespace.
	pub auth_user_list: std::vec::Vec<CdnNamespaceAuthUser>,
}

/// An authenticated CDN user for a given namespace.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CdnNamespaceAuthUser {
	/// A user name.
	pub user: std::string::String,
}

/// A CDN domain for a given namespace.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CdnNamespaceDomain {
	/// A valid domain name (no protocol).
	pub domain: std::string::String,
}

/// Cloud configuration for a given version.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CloudVersionConfig {
	/// CDN configuration for a given version.
	pub cdn: std::option::Option<CdnVersionConfig>,
	/// Matchmaker configuration for a given version.
	pub matchmaker: std::option::Option<MatchmakerVersionConfig>,
	/// KV configuration for a given version.
	pub kv: std::option::Option<KvVersionConfig>,
}

/// KV configuration for a given version.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvVersionConfig {}

/// Matchmaker configuration for a given version.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MatchmakerVersionConfig {
	/// A list of game modes.
	pub lobby_groups: std::vec::Vec<LobbyGroup>,
	/// Matchmaker captcha configuration.
	pub captcha: std::option::Option<MatchmakerCaptcha>,
}

/// Matchmaker captcha configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MatchmakerCaptcha {
	/// hCpatcha configuration.
	pub hcaptcha: std::option::Option<MatchmakerCaptchaHcaptcha>,
	/// Denotes how many requests a connection can make before it is required to reverify a captcha.
	pub requests_before_reverify: i32,
	/// Denotes how long a connection can continue to reconnect without having to reverify a captcha (in milliseconds).
	pub verification_ttl: i64,
}

/// hCpatcha configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MatchmakerCaptchaHcaptcha {
	/// How hard a captcha should be.
	pub level: CaptchaLevel,
}

/// How hard a captcha should be.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptchaLevel {
	#[allow(missing_docs)] // documentation missing in model
	AlwaysOn,
	#[allow(missing_docs)] // documentation missing in model
	Difficult,
	#[allow(missing_docs)] // documentation missing in model
	Easy,
	#[allow(missing_docs)] // documentation missing in model
	Moderate,
	/// Unknown contains new variants that have been added since this code was generated.
	Unknown(String),
}
impl std::convert::From<&str> for CaptchaLevel {
	fn from(s: &str) -> Self {
		match s {
			"always_on" => CaptchaLevel::AlwaysOn,
			"difficult" => CaptchaLevel::Difficult,
			"easy" => CaptchaLevel::Easy,
			"moderate" => CaptchaLevel::Moderate,
			other => CaptchaLevel::Unknown(other.to_owned()),
		}
	}
}
impl std::str::FromStr for CaptchaLevel {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		Ok(CaptchaLevel::from(s))
	}
}
impl CaptchaLevel {
	/// Returns the `&str` value of the enum member.
	pub fn as_str(&self) -> &str {
		match self {
			CaptchaLevel::AlwaysOn => "always_on",
			CaptchaLevel::Difficult => "difficult",
			CaptchaLevel::Easy => "easy",
			CaptchaLevel::Moderate => "moderate",
			CaptchaLevel::Unknown(s) => s.as_ref(),
		}
	}
	/// Returns all the `&str` values of the enum members.
	pub fn values() -> &'static [&'static str] {
		&["always_on", "difficult", "easy", "moderate"]
	}
}
impl AsRef<str> for CaptchaLevel {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

/// A game mode.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LobbyGroup {
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub name_id: std::string::String,
	/// A list of game mode regions.
	pub regions: std::vec::Vec<LobbyGroupRegion>,
	/// Unsigned 32 bit integer.
	pub max_players_normal: i32,
	/// Unsigned 32 bit integer.
	pub max_players_direct: i32,
	/// Unsigned 32 bit integer.
	pub max_players_party: i32,
	/// A union representing the runtime a game mode runs on.
	pub runtime: LobbyGroupRuntime,
}

/// A union representing the runtime a game mode runs on.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LobbyGroupRuntime {
	/// A game mode runtime running through Docker.
	Docker(LobbyGroupRuntimeDocker),
}

/// A game mode runtime running through Docker.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LobbyGroupRuntimeDocker {
	/// A universally unique identifier.
	pub build_id: std::option::Option<std::string::String>,
	/// A list of docker arguments.
	pub args: std::vec::Vec<std::string::String>,
	/// A list of docker environment variables.
	pub env_vars: std::vec::Vec<LobbyGroupRuntimeDockerEnvVar>,
	/// The network mode the job should run on.
	pub network_mode: NetworkMode,
	/// A list of docker ports.
	pub ports: std::vec::Vec<LobbyGroupRuntimeDockerPort>,
}

/// The network mode the job should run on.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkMode {
	#[allow(missing_docs)] // documentation missing in model
	Bridge,
	#[allow(missing_docs)] // documentation missing in model
	Host,
	/// Unknown contains new variants that have been added since this code was generated.
	Unknown(String),
}
impl std::convert::From<&str> for NetworkMode {
	fn from(s: &str) -> Self {
		match s {
			"bridge" => NetworkMode::Bridge,
			"host" => NetworkMode::Host,
			other => NetworkMode::Unknown(other.to_owned()),
		}
	}
}
impl std::str::FromStr for NetworkMode {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		Ok(NetworkMode::from(s))
	}
}
impl NetworkMode {
	/// Returns the `&str` value of the enum member.
	pub fn as_str(&self) -> &str {
		match self {
			NetworkMode::Bridge => "bridge",
			NetworkMode::Host => "host",
			NetworkMode::Unknown(s) => s.as_ref(),
		}
	}
	/// Returns all the `&str` values of the enum members.
	pub fn values() -> &'static [&'static str] {
		&["bridge", "host"]
	}
}
impl AsRef<str> for NetworkMode {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

/// A docker environment variable.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LobbyGroupRuntimeDockerEnvVar {
	/// The key of this environment variable.
	pub key: std::string::String,
	/// The value of this environment variable.
	pub value: std::string::String,
}

/// A game mode region.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LobbyGroupRegion {
	/// A universally unique identifier.
	pub region_id: std::string::String,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub tier_name_id: std::string::String,
	/// Configuration for how many idle lobbies a game version should have.
	pub idle_lobbies: std::option::Option<IdleLobbiesConfig>,
}

/// Configuration for how many idle lobbies a game version should have.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdleLobbiesConfig {
	/// Unsigned 32 bit integer.
	pub min_idle_lobbies: i32,
	/// Unsigned 32 bit integer.
	pub max_idle_lobbies: i32,
}

/// CDN configuration for a given version.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CdnVersionConfig {
	/// A universally unique identifier.
	pub site_id: std::option::Option<std::string::String>,
}

/// A full version.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionFull {
	/// A universally unique identifier.
	pub version_id: std::string::String,
	/// RFC3339 timestamp.
	pub create_ts: chrono::DateTime<chrono::Utc>,
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// Cloud configuration for a given version.
	pub config: CloudVersionConfig,
}

/// Provided by watchable endpoints used in blocking loops.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WatchResponse {
	/// Index indicating the version of the data responded. Pas this to `rivet.common#WatchQuery` to block and wait for the next response.
	pub index: std::string::String,
}

/// A full game.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameFull {
	/// A universally unique identifier.
	pub game_id: std::string::String,
	/// RFC3339 timestamp.
	pub create_ts: chrono::DateTime<chrono::Utc>,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub name_id: std::string::String,
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// A universally unique identifier.
	pub developer_group_id: std::string::String,
	/// Unsigned 32 bit integer.
	pub total_player_count: i32,
	/// The URL of this game's logo image.
	pub logo_url: std::option::Option<std::string::String>,
	/// The URL of this game's banner image.
	pub banner_url: std::option::Option<std::string::String>,
	/// A list of namespace summaries.
	pub namespaces: std::vec::Vec<NamespaceSummary>,
	/// A list of version summaries.
	pub versions: std::vec::Vec<VersionSummary>,
	/// A list of region summaries.
	pub available_regions: std::vec::Vec<RegionSummary>,
}

/// A version summary.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionSummary {
	/// A universally unique identifier.
	pub version_id: std::string::String,
	/// RFC3339 timestamp.
	pub create_ts: chrono::DateTime<chrono::Utc>,
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
}

/// A group summary.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupSummary {
	/// A universally unique identifier.
	pub group_id: std::string::String,
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// The URL of this group's avatar image.
	pub avatar_url: std::option::Option<std::string::String>,
	/// Whether or not this group is a developer.
	pub is_developer: bool,
	/// External links for this group.
	pub external: GroupExternalLinks,
}

/// External links for this group.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupExternalLinks {
	/// A link to this group's profile page.
	pub profile: std::string::String,
	/// A link to this group's chat page.
	pub chat: std::string::String,
}

/// A game summary.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameSummary {
	/// A universally unique identifier.
	pub game_id: std::string::String,
	/// RFC3339 timestamp.
	pub create_ts: chrono::DateTime<chrono::Utc>,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub name_id: std::string::String,
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// A universally unique identifier.
	pub developer_group_id: std::string::String,
	/// Unsigned 32 bit integer.
	pub total_player_count: i32,
	/// The URL of this game's logo image.
	pub logo_url: std::option::Option<std::string::String>,
	/// The URL of this game's banner image.
	pub banner_url: std::option::Option<std::string::String>,
}

/// The current authenticated agent.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthAgent {
	/// The current authenticated game cloud.
	GameCloud(AuthAgentGameCloud),
	/// The current authenticated identity.
	Identity(AuthAgentIdentity),
}

/// The current authenticated game cloud.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthAgentGameCloud {
	/// A universally unique identifier.
	pub game_id: std::string::String,
}

/// The current authenticated identity.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthAgentIdentity {
	/// A universally unique identifier.
	pub identity_id: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRayPerfLogsRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGroupRequest {
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupBillingCheckoutRequest {
	/// How much money to checkout (in hundred-thousandths USD, 100,000 = $1.00).
	pub amount: i64,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConvertGroupRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGroupInvoicesListRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGroupTransfersListRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGroupPaymentsListRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGroupBillingRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRegionTiersRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportMatchmakerLobbyHistoryRequest {
	/// Unsigned 64 bit integer.
	pub query_start: i64,
	/// Unsigned 64 bit integer.
	pub query_end: i64,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteMatchmakerLobbyRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameCdnSiteRequest {
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// A list of files preparing to upload.
	pub files: std::vec::Vec<UploadPrepareFile>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListGameCdnSitesRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameBuildRequest {
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// A tag given to the game build.
	pub image_tag: std::string::String,
	/// A file being prepared to upload.
	pub image_file: UploadPrepareFile,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListGameBuildsRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCloudTokenRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetNamespaceLobbyRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListNamespaceLobbiesRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetNamespaceAnalyticsMatchmakerLiveRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetNamespaceCdnAuthTypeRequest {
	/// A value denoting what type of authentication to use for a game namespace's CDN.
	pub auth_type: CdnAuthType,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoveNamespaceCdnAuthUserRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateNamespaceCdnAuthUserRequest {
	/// A user name.
	pub user: std::string::String,
	/// A bcrypt encrypted password. An error is returned if the given string is not properly encrypted.
	pub password: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGameNamespaceMatchmakerConfigRequest {
	/// Unsigned 32 bit integer.
	pub lobby_count_max: i32,
	/// Unsigned 32 bit integer.
	pub max_players: i32,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGameNamespaceTokenDevelopmentRequest {
	#[allow(missing_docs)] // documentation missing in model
	pub hostname: std::string::String,
	/// A list of docker ports.
	pub lobby_ports: std::vec::Vec<LobbyGroupRuntimeDockerPort>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGameNamespaceRequest {
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub name_id: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateGameNamespaceMatchmakerConfigRequest {
	/// Unsigned 32 bit integer.
	pub lobby_count_max: i32,
	/// Unsigned 32 bit integer.
	pub max_players: i32,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToggleNamespaceDomainPublicAuthRequest {
	/// Whether or not to enable authentication based on domain.
	pub enabled: bool,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoveNamespaceDomainRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateNamespaceDomainRequest {
	/// A valid domain name (no protocol).
	pub domain: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameNamespaceTokenDevelopmentRequest {
	/// The hostname used for the token.
	pub hostname: std::string::String,
	/// A list of docker ports.
	pub lobby_ports: std::vec::Vec<LobbyGroupRuntimeDockerPort>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameNamespaceTokenPublicRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateGameNamespaceVersionRequest {
	/// A universally unique identifier.
	pub version_id: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGameNamespaceByIdRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameNamespaceRequest {
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// A universally unique identifier.
	pub version_id: std::string::String,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub name_id: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGameVersionRequest {
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// Cloud configuration for a given version.
	pub config: CloudVersionConfig,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGameVersionByIdRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameVersionRequest {
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// Cloud configuration for a given version.
	pub config: CloudVersionConfig,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameBannerUploadCompleteRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameBannerUploadPrepareRequest {
	/// The path/filename of the game banner.
	pub path: std::string::String,
	/// The MIME type of the game banner.
	pub mime: std::option::Option<std::string::String>,
	/// Unsigned 64 bit integer.
	pub content_length: i64,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameLogoUploadCompleteRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameLogoUploadPrepareRequest {
	/// The path/filename of the game logo.
	pub path: std::string::String,
	/// The MIME type of the game logo.
	pub mime: std::option::Option<std::string::String>,
	/// Unsigned 64 bit integer.
	pub content_length: i64,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGameRequest {
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub name_id: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGameByIdRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameRequest {
	/// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
	pub name_id: std::string::String,
	/// Represent a resource's readable display name.
	pub display_name: std::string::String,
	/// A universally unique identifier.
	pub developer_group_id: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGamesRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteUploadRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InspectRequest {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRayPerfLogsResponse {
	/// A list of service performance summaries.
	pub perf_lists: std::vec::Vec<SvcPerf>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGroupResponse {
	/// A list of validation errors.
	pub errors: std::vec::Vec<ValidationError>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupBillingCheckoutResponse {
	/// The URL of the checkout session.
	pub url: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConvertGroupResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGroupInvoicesListResponse {
	/// A list of a group's billing invoices.
	pub invoices: std::vec::Vec<GroupBillingInvoice>,
	/// The pagination anchor.
	pub anchor: std::option::Option<std::string::String>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGroupTransfersListResponse {
	/// A list of a group's billing transfers.
	pub transfers: std::vec::Vec<GroupBillingTransfer>,
	/// The ID of the last transfer listed.
	pub end_transfer_id: std::option::Option<std::string::String>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGroupPaymentsListResponse {
	/// A list of a group's billing payments.
	pub payments: std::vec::Vec<GroupBillingPayment>,
	/// The ID of the last payment listed.
	pub end_payment_id: std::option::Option<std::string::String>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGroupBillingResponse {
	/// A group billing summary.
	pub billing: GroupBillingSummary,
	#[allow(missing_docs)] // documentation missing in model
	pub bank_source: GroupBankSource,
	/// A list of region summaries.
	pub available_regions: std::vec::Vec<RegionSummary>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRegionTiersResponse {
	/// A list of region server tiers.
	pub tiers: std::vec::Vec<RegionTier>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportMatchmakerLobbyHistoryResponse {
	/// The URL to a CSV file for the given lobby history.
	pub url: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteMatchmakerLobbyResponse {
	/// Whether or not the lobby was successfully stopped.
	pub did_remove: bool,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameCdnSiteResponse {
	/// A universally unique identifier.
	pub site_id: std::string::String,
	/// A universally unique identifier.
	pub upload_id: std::string::String,
	#[allow(missing_docs)] // documentation missing in model
	pub presigned_requests: std::vec::Vec<UploadPresignedRequest>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListGameCdnSitesResponse {
	/// A list of CDN site summaries.
	pub sites: std::vec::Vec<CdnSiteSummary>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameBuildResponse {
	/// A universally unique identifier.
	pub build_id: std::string::String,
	/// A universally unique identifier.
	pub upload_id: std::string::String,
	/// A presigned request used to upload files. Upload your file to the given URL via a PUT request.
	pub image_presigned_request: UploadPresignedRequest,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListGameBuildsResponse {
	/// A list of build summaries.
	pub builds: std::vec::Vec<BuildSummary>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCloudTokenResponse {
	/// A JSON Web Token. Slightly modified to include a description prefix and use Protobufs of JSON.
	pub token: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetNamespaceLobbyResponse {
	/// A logs summary for a lobby.
	pub lobby: LogsLobbySummary,
	/// A list of URLs.
	pub stdout_presigned_urls: std::vec::Vec<std::string::String>,
	/// A list of URLs.
	pub stderr_presigned_urls: std::vec::Vec<std::string::String>,
	/// A list of service performance summaries.
	pub perf_lists: std::vec::Vec<SvcPerf>,
	/// Metrics relating to a job service.
	pub metrics: std::option::Option<SvcMetrics>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListNamespaceLobbiesResponse {
	/// A list of lobby log summaries.
	pub lobbies: std::vec::Vec<LogsLobbySummary>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetNamespaceAnalyticsMatchmakerLiveResponse {
	/// A list of analytics lobby summaries.
	pub lobbies: std::vec::Vec<AnalyticsLobbySummary>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SetNamespaceCdnAuthTypeResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoveNamespaceCdnAuthUserResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateNamespaceCdnAuthUserResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGameNamespaceMatchmakerConfigResponse {
	/// A list of validation errors.
	pub errors: std::vec::Vec<ValidationError>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGameNamespaceTokenDevelopmentResponse {
	/// A list of validation errors.
	pub errors: std::vec::Vec<ValidationError>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGameNamespaceResponse {
	/// A list of validation errors.
	pub errors: std::vec::Vec<ValidationError>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateGameNamespaceMatchmakerConfigResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToggleNamespaceDomainPublicAuthResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoveNamespaceDomainResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateNamespaceDomainResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameNamespaceTokenDevelopmentResponse {
	/// A JSON Web Token. Slightly modified to include a description prefix and use Protobufs of JSON.
	pub token: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameNamespaceTokenPublicResponse {
	/// A JSON Web Token. Slightly modified to include a description prefix and use Protobufs of JSON.
	pub token: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateGameNamespaceVersionResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGameNamespaceByIdResponse {
	/// A full namespace.
	pub namespace: NamespaceFull,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameNamespaceResponse {
	/// A universally unique identifier.
	pub namespace_id: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGameVersionResponse {
	/// A list of validation errors.
	pub errors: std::vec::Vec<ValidationError>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGameVersionByIdResponse {
	/// A full version.
	pub version: VersionFull,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameVersionResponse {
	/// A universally unique identifier.
	pub version_id: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameBannerUploadCompleteResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameBannerUploadPrepareResponse {
	/// A universally unique identifier.
	pub upload_id: std::string::String,
	/// A presigned request used to upload files. Upload your file to the given URL via a PUT request.
	pub presigned_request: UploadPresignedRequest,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameLogoUploadCompleteResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameLogoUploadPrepareResponse {
	/// A universally unique identifier.
	pub upload_id: std::string::String,
	/// A presigned request used to upload files. Upload your file to the given URL via a PUT request.
	pub presigned_request: UploadPresignedRequest,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidateGameResponse {
	/// A list of validation errors.
	pub errors: std::vec::Vec<ValidationError>,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGameByIdResponse {
	/// A full game.
	pub game: GameFull,
	/// Provided by watchable endpoints used in blocking loops.
	pub watch: WatchResponse,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameResponse {
	/// A universally unique identifier.
	pub game_id: std::string::String,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetGamesResponse {
	/// A list of game summaries.
	pub games: std::vec::Vec<GameSummary>,
	/// A list of group summaries.
	pub groups: std::vec::Vec<GroupSummary>,
	/// Provided by watchable endpoints used in blocking loops.
	pub watch: WatchResponse,
}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteUploadResponse {}

#[allow(missing_docs)] // documentation missing in model
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InspectResponse {
	/// The current authenticated agent.
	pub agent: AuthAgent,
}

