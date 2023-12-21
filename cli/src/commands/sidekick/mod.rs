use clap::Parser;

use console::Term;
use global_error::prelude::*;
use serde::Serialize;
use serde_json::{json, Value};

use crate::util::{
	global_config,
	struct_fmt::{self, Format},
};

pub mod check_login_state;
pub mod get_link;
pub mod get_token;
pub mod get_version;
pub mod wait_for_login;

pub trait SideKickHandler: Serialize {
	// async fn execute(&self, ctx: &cli_core::Ctx, term: &Term) ->
	// GlobalResult<SideKickResponse>;
	fn print(&self) {
		println!("{}", serde_json::to_string_pretty(self).unwrap());
	}
}

#[derive(Parser)]
pub enum SubCommand {
	/// Get the link for the user to sign in
	GetLink(get_link::Opts),
	/// Long poll the server to check if the user has signed in
	WaitForLogin(wait_for_login::Opts),
	/// Check if the CLI is logged in already
	CheckLoginState(check_login_state::Opts),
	/// Get the token from the CLI
	GetToken(get_token::Opts),
	///
	GetVersion(get_version::Opts),
}

/// Any response that can come from the sidekick. There should only be a single
/// response from any sidekick call, though it might include multiple messages.
/// This is so a single schema can be parsed by whatever is consuming the
/// sidekick output.
#[derive(Serialize)]
pub struct SideKickResponse(pub Value);

fn serialize_output(v: GlobalResult<impl Serialize>) -> GlobalResult<String> {
	Ok(format!(
		"{}",
		serde_json::to_string(&unwrap!(v)).expect("derpy error handler")
	))
}

impl SubCommand {
	/// These commands run before a token is required, so they don't have access
	/// to ctx
	pub async fn pre_execute(&self, token: &Option<String>) -> GlobalResult<()> {
		let response = match self {
			SubCommand::GetLink(opts) => serialize_output(opts.execute().await),
			SubCommand::WaitForLogin(opts) => serialize_output(opts.execute().await),
			SubCommand::CheckLoginState(_opts) => serialize_output(self.validate_token(&token)),
			_ => {
				// If the command is anything else, we need to check if a token
				// has already been provided. If not, we need to print an error
				// and return early since that's what the plugins will expect.
				if let Err(_) = self.validate_token(&token) {
					// The message has already been printed out so we can just
					// return Ok here.
					serialize_output(Ok(SideKickResponse(json!({
						"output": "Token not found. Please run `rivet sidekick get-link` to sign in."
					}))))
				} else {
					serialize_output(Ok(SideKickResponse(json!({}))))
				}
			}
		};

		// Print the response
		SubCommand::print(response)?;

		Ok(())
	}

	pub async fn execute(&self, ctx: &cli_core::Ctx, _term: &Term) -> GlobalResult<()> {
		let (_api_endpoint, _token) = global_config::read_project(|x| {
			(x.cluster.api_endpoint.clone(), x.tokens.cloud.clone())
		})
		.await?;

		let response = match self {
			SubCommand::GetLink(_)
			| SubCommand::CheckLoginState(_)
			| SubCommand::WaitForLogin(_) => {
				unreachable!("This command should be handled before this")
			}
			SubCommand::GetToken(opts) => serialize_output(opts.execute(ctx).await),
			SubCommand::GetVersion(opts) => serialize_output(opts.execute(ctx).await),
		};

		// Print the response
		SubCommand::print(response)?;

		Ok(())
	}

	pub fn validate_token(&self, token: &Option<String>) -> GlobalResult<SideKickResponse> {
		if token.is_none() {
			bail!("No Rivet token found, please do the sign in process");
		}

		Ok(SideKickResponse(json!({
			"output": "Token Valid",
		})))
	}

	pub fn print(response: GlobalResult<String>) -> GlobalResult<()> {
		match response {
			Ok(sidekick_response) => {
				struct_fmt::print(&Format::Json, &json!({ "Ok": sidekick_response }))?;
			}
			Err(global_error) => {
				struct_fmt::print(
					&Format::Json,
					&json!({
						"Err": global_error.to_string()
					}),
				)?;
			}
		}

		Ok(())
	}
}
