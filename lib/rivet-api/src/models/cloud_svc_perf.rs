/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudSvcPerf : A service performance summary.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudSvcPerf {
	/// Unsigned 64 bit integer.
	#[serde(rename = "duration")]
	pub duration: i64,
	/// A list of performance marks.
	#[serde(rename = "marks")]
	pub marks: Vec<crate::models::CloudLogsPerfMark>,
	#[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
	pub req_id: Option<uuid::Uuid>,
	/// A list of performance spans.
	#[serde(rename = "spans")]
	pub spans: Vec<crate::models::CloudLogsPerfSpan>,
	/// The name of the service.
	#[serde(rename = "svc_name")]
	pub svc_name: String,
	/// RFC3339 timestamp.
	#[serde(rename = "ts")]
	pub ts: String,
}

impl CloudSvcPerf {
	/// A service performance summary.
	pub fn new(
		duration: i64,
		marks: Vec<crate::models::CloudLogsPerfMark>,
		spans: Vec<crate::models::CloudLogsPerfSpan>,
		svc_name: String,
		ts: String,
	) -> CloudSvcPerf {
		CloudSvcPerf {
			duration,
			marks,
			req_id: None,
			spans,
			svc_name,
			ts,
		}
	}
}
