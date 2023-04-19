/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatMessage {
	#[serde(rename = "body")]
	pub body: Box<crate::models::ChatMessageBody>,
	#[serde(rename = "chat_message_id")]
	pub chat_message_id: uuid::Uuid,
	#[serde(rename = "send_ts")]
	pub send_ts: String,
	#[serde(rename = "thread_id")]
	pub thread_id: uuid::Uuid,
}

impl ChatMessage {
	pub fn new(
		body: crate::models::ChatMessageBody,
		chat_message_id: uuid::Uuid,
		send_ts: String,
		thread_id: uuid::Uuid,
	) -> ChatMessage {
		ChatMessage {
			body: Box::new(body),
			chat_message_id,
			send_ts,
			thread_id,
		}
	}
}
