/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServersPortProtocol {
	#[serde(rename = "http")]
	Http,
	#[serde(rename = "https")]
	Https,
	#[serde(rename = "tcp")]
	Tcp,
	#[serde(rename = "tcp_tls")]
	TcpTls,
	#[serde(rename = "udp")]
	Udp,
}

impl ToString for ServersPortProtocol {
	fn to_string(&self) -> String {
		match self {
			Self::Http => String::from("http"),
			Self::Https => String::from("https"),
			Self::Tcp => String::from("tcp"),
			Self::TcpTls => String::from("tcp_tls"),
			Self::Udp => String::from("udp"),
		}
	}
}

impl Default for ServersPortProtocol {
	fn default() -> ServersPortProtocol {
		Self::Http
	}
}
