/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`servers_logs_get_server_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServersLogsGetServerLogsError {
    Status400(crate::models::ErrorBody),
    Status403(crate::models::ErrorBody),
    Status404(crate::models::ErrorBody),
    Status408(crate::models::ErrorBody),
    Status429(crate::models::ErrorBody),
    Status500(crate::models::ErrorBody),
    UnknownValue(serde_json::Value),
}


/// Returns the logs for a given server.
pub async fn servers_logs_get_server_logs(configuration: &configuration::Configuration, server_id: &str, stream: crate::models::ServersLogStream, game_id: Option<&str>, watch_index: Option<&str>) -> Result<crate::models::ServersGetServerLogsResponse, Error<ServersLogsGetServerLogsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/servers/{server_id}/logs", local_var_configuration.base_path, server_id=crate::apis::urlencode(server_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("stream", &stream.to_string())]);
    if let Some(ref local_var_str) = game_id {
        local_var_req_builder = local_var_req_builder.query(&[("game_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = watch_index {
        local_var_req_builder = local_var_req_builder.query(&[("watch_index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServersLogsGetServerLogsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

