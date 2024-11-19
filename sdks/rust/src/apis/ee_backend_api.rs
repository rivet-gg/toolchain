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

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`ee_backend_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EeBackendCreateError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ee_backend_deploy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EeBackendDeployError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ee_backend_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EeBackendGetError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ee_backend_get_db_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EeBackendGetDbUrlError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ee_backend_get_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EeBackendGetEventsError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ee_backend_get_variables`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EeBackendGetVariablesError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ee_backend_prepare_deploy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EeBackendPrepareDeployError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ee_backend_provision_database`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EeBackendProvisionDatabaseError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ee_backend_update_variables`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EeBackendUpdateVariablesError {
	Status400(crate::models::ErrorBody),
	Status403(crate::models::ErrorBody),
	Status404(crate::models::ErrorBody),
	Status408(crate::models::ErrorBody),
	Status429(crate::models::ErrorBody),
	Status500(crate::models::ErrorBody),
	UnknownValue(serde_json::Value),
}

/// Creates a new backend environment.
pub async fn ee_backend_create(
	configuration: &configuration::Configuration,
	game_id: &str,
	environment_id: &str,
	body: serde_json::Value,
) -> Result<crate::models::EeBackendCreateResponse, Error<EeBackendCreateError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/games/{game_id}/environments/{environment_id}/backend",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		environment_id = crate::apis::urlencode(environment_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};
	local_var_req_builder = local_var_req_builder.json(&body);

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<EeBackendCreateError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

pub async fn ee_backend_deploy(
	configuration: &configuration::Configuration,
	game_id: &str,
	environment_id: &str,
	ee_backend_deploy_request: crate::models::EeBackendDeployRequest,
) -> Result<crate::models::EeBackendDeployResponse, Error<EeBackendDeployError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/games/{game_id}/environments/{environment_id}/backend/deploy",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		environment_id = crate::apis::urlencode(environment_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};
	local_var_req_builder = local_var_req_builder.json(&ee_backend_deploy_request);

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<EeBackendDeployError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

pub async fn ee_backend_get(
	configuration: &configuration::Configuration,
	game_id: &str,
	environment_id: &str,
	watch_index: Option<&str>,
) -> Result<crate::models::EeBackendGetResponse, Error<EeBackendGetError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/games/{game_id}/environments/{environment_id}/backend",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		environment_id = crate::apis::urlencode(environment_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

	if let Some(ref local_var_str) = watch_index {
		local_var_req_builder =
			local_var_req_builder.query(&[("watch_index", &local_var_str.to_string())]);
	}
	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
		let local_var_entity: Option<EeBackendGetError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

pub async fn ee_backend_get_db_url(
	configuration: &configuration::Configuration,
	game_id: &str,
	environment_id: &str,
) -> Result<crate::models::EeBackendGetDbUrlResponse, Error<EeBackendGetDbUrlError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/games/{game_id}/environments/{environment_id}/backend/db",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		environment_id = crate::apis::urlencode(environment_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
		let local_var_entity: Option<EeBackendGetDbUrlError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

pub async fn ee_backend_get_events(
	configuration: &configuration::Configuration,
	game_id: &str,
	environment_id: &str,
	watch_index: Option<&str>,
) -> Result<crate::models::EeBackendGetEventsResponse, Error<EeBackendGetEventsError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/games/{game_id}/environments/{environment_id}/backend/events",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		environment_id = crate::apis::urlencode(environment_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

	if let Some(ref local_var_str) = watch_index {
		local_var_req_builder =
			local_var_req_builder.query(&[("watch_index", &local_var_str.to_string())]);
	}
	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
		let local_var_entity: Option<EeBackendGetEventsError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Get backend variables from an backend.
pub async fn ee_backend_get_variables(
	configuration: &configuration::Configuration,
	game_id: &str,
	environment_id: &str,
) -> Result<crate::models::EeBackendGetVariablesResponse, Error<EeBackendGetVariablesError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/games/{game_id}/environments/{environment_id}/backend/variables",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		environment_id = crate::apis::urlencode(environment_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
		let local_var_entity: Option<EeBackendGetVariablesError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

pub async fn ee_backend_prepare_deploy(
	configuration: &configuration::Configuration,
	game_id: &str,
	environment_id: &str,
	ee_backend_prepare_deploy_request: crate::models::EeBackendPrepareDeployRequest,
) -> Result<crate::models::EeBackendPrepareDeployResponse, Error<EeBackendPrepareDeployError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/games/{game_id}/environments/{environment_id}/backend/deploy/prepare",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		environment_id = crate::apis::urlencode(environment_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};
	local_var_req_builder = local_var_req_builder.json(&ee_backend_prepare_deploy_request);

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<EeBackendPrepareDeployError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Provisions the database for the given backend. Idempotent.
pub async fn ee_backend_provision_database(
	configuration: &configuration::Configuration,
	game_id: &str,
	environment_id: &str,
) -> Result<(), Error<EeBackendProvisionDatabaseError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/games/{game_id}/environments/{environment_id}/backend/provision-database",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		environment_id = crate::apis::urlencode(environment_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		Ok(())
	} else {
		let local_var_entity: Option<EeBackendProvisionDatabaseError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Updates backend variables for an backend.
pub async fn ee_backend_update_variables(
	configuration: &configuration::Configuration,
	game_id: &str,
	environment_id: &str,
	ee_backend_update_variables_request: crate::models::EeBackendUpdateVariablesRequest,
) -> Result<(), Error<EeBackendUpdateVariablesError>> {
	let local_var_configuration = configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/games/{game_id}/environments/{environment_id}/backend/variables",
		local_var_configuration.base_path,
		game_id = crate::apis::urlencode(game_id),
		environment_id = crate::apis::urlencode(environment_id)
	);
	let mut local_var_req_builder =
		local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder =
			local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}
	if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
		local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
	};
	local_var_req_builder = local_var_req_builder.json(&ee_backend_update_variables_request);

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		Ok(())
	} else {
		let local_var_entity: Option<EeBackendUpdateVariablesError> =
			serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}
