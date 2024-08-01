use clap::Parser;
use console::Term;
use global_error::prelude::*;
use serde::Serialize;
use serde_json::{json, Value};

use crate::util::{
	global_config,
	struct_fmt::{self, Format},
};

pub mod backend_dev;
pub mod backend_gen_sdk;
pub mod deploy;
pub mod exec_command;
pub mod generate_config;
pub mod get_bootstrap_data;
pub mod get_cli_version;
pub mod get_link;
pub mod get_lobbies_link;
pub mod get_logs_link;
pub mod get_namespace_dev_token;
pub mod get_namespace_pub_token;
pub mod get_versions_link;
pub mod kill_process;
pub mod show_term;
pub mod unlink;
pub mod util;
pub mod wait_for_login;

pub trait SideKickHandler: Serialize {
	fn print(&self) {
		println!("{}", serde_json::to_string_pretty(self).unwrap());
	}
}

#[derive(Parser)]
pub enum SubCommand {
	/// Run an arbritrary command in a terminal window. Primarily used for showing logs from
	/// arbirtrary commands.
	///
	/// Prefer using the `--show-terminal` flag for Rivet-specific commands.
	ShowTerm(show_term::Opts),
	/// Executes a command and writes stdout and stderr to separate files.
	///
	/// Used to run commands in game engiens that don't support piping stdout & stderr.
	ExecCommand(exec_command::Opts),
	/// Sends a SIGINT to a given PID.
	KillProcess(kill_process::Opts),
	/// Get the link for the user to sign in
	GetLink(get_link::Opts),
	/// Long poll the server to check if the user has signed in
	WaitForLogin(wait_for_login::Opts),
	/// Check if the CLI is logged in already
	CheckLoginState,
	/// Get the token from the CLI
	GetBootstrapData(get_bootstrap_data::Opts),
	/// Get a link to the `manage versions` page
	GetVersion(get_versions_link::Opts),
	/// Get a link to the `logs` page
	GetLobbies(get_lobbies_link::Opts),
	/// Get the version of the CLI
	GetLogs(get_logs_link::Opts),
	/// Deploy a version
	Deploy(deploy::Opts),
	/// Get the CLI version
	GetCliVersion(get_cli_version::Opts),
	/// Get a public namespace token
	GetNamespacePublicToken(get_namespace_pub_token::Opts),
	/// Get a development namespace token
	GetNamespaceDevelopmentToken(get_namespace_dev_token::Opts),
	/// Generate config
	GenerateConfig(generate_config::Opts),
	BackendDev(backend_dev::Opts),
	BackendGenerateSdk(backend_gen_sdk::Opts),
	/// Unlink current game
	Unlink(unlink::Opts),
}

/// Any response that can come from the sidekick. There should only be a single
/// response from any sidekick call, though it might include multiple messages.
/// This is so a single schema can be parsed by whatever is consuming the
/// sidekick output.
#[derive(Serialize)]
pub struct SideKickResponse(pub Value);

fn serialize_output(v: GlobalResult<impl Serialize>) -> GlobalResult<Value> {
	Ok(unwrap!(serde_json::to_value(&unwrap!(v))))
}

pub enum PreExecuteHandled {
	Yes,
	No,
}

impl SubCommand {
	/// These commands run before a token is required, so they don't have access
	/// to ctx
	pub async fn pre_execute(
		&self,
		token: &Option<String>,
		show_terminal: bool,
		inside_terminal: bool,
	) -> GlobalResult<PreExecuteHandled> {
		if show_terminal {
			SubCommand::show_terminal().await?;
			return Ok(PreExecuteHandled::Yes);
		}

		let mut handled = PreExecuteHandled::Yes;
		let response = match self {
			SubCommand::ShowTerm(opts) => serialize_output(opts.execute().await),
			SubCommand::ExecCommand(opts) => serialize_output(opts.execute().await),
			SubCommand::KillProcess(opts) => serialize_output(opts.execute().await),
			SubCommand::GetLink(opts) => serialize_output(opts.execute().await),
			SubCommand::WaitForLogin(opts) => serialize_output(opts.execute().await),
			SubCommand::CheckLoginState => serialize_output(self.validate_token(&token)),
			SubCommand::GetCliVersion(opts) => serialize_output(opts.execute().await),
			SubCommand::GenerateConfig(opts) => serialize_output(opts.execute().await),
			SubCommand::BackendDev(opts) => serialize_output(opts.execute().await),
			SubCommand::BackendGenerateSdk(opts) => serialize_output(opts.execute().await),
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
					handled = PreExecuteHandled::No;

					serialize_output(Ok(String::new()))
				}
			}
		};

		if let PreExecuteHandled::Yes = handled {
			// Print the response, but only if we're not inside a terminal
			if !inside_terminal {
				SubCommand::print(response)?;
			}
		}

		Ok(handled)
	}

	pub async fn execute(
		&self,
		ctx: &toolchain_core::Ctx,
		_term: &Term,
		show_terminal: bool,
		inside_terminal: bool,
	) -> GlobalResult<()> {
		if show_terminal {
			SubCommand::show_terminal().await?;
			return Ok(());
		}

		let (_api_endpoint, _token) = global_config::read_project(|x| {
			(x.cluster.api_endpoint.clone(), x.tokens.cloud.clone())
		})
		.await?;

		let response = match self {
			SubCommand::ShowTerm(_)
			| SubCommand::ExecCommand(_)
			| SubCommand::KillProcess(_)
			| SubCommand::GetLink(_)
			| SubCommand::CheckLoginState
			| SubCommand::WaitForLogin(_)
			| SubCommand::GenerateConfig(_)
			| SubCommand::BackendDev(_)
			| SubCommand::BackendGenerateSdk(_)
			| SubCommand::GetCliVersion(_) => {
				unreachable!("This command should be handled before this")
			}
			SubCommand::GetBootstrapData(opts) => serialize_output(opts.execute(ctx).await),
			SubCommand::GetVersion(opts) => serialize_output(opts.execute(ctx).await),
			SubCommand::GetLobbies(opts) => serialize_output(opts.execute(ctx).await),
			SubCommand::GetLogs(opts) => serialize_output(opts.execute(ctx).await),
			SubCommand::Deploy(opts) => serialize_output(opts.execute(ctx).await),
			SubCommand::GetNamespacePublicToken(opts) => serialize_output(opts.execute(ctx).await),
			SubCommand::GetNamespaceDevelopmentToken(opts) => {
				serialize_output(opts.execute(ctx).await)
			}
			SubCommand::Unlink(opts) => serialize_output(opts.execute(ctx).await),
		};

		// Print the response, but only if we're not inside a terminal
		if !inside_terminal {
			SubCommand::print(response)?;
		}

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

	pub fn print(response: GlobalResult<Value>) -> GlobalResult<()> {
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

	/**
	 * Reads the raw env vars and spawns a new terminal for the given command.
	 */
	pub async fn show_terminal() -> GlobalResult<()> {
		// TODO(forest): The code doesn't handle the case where the binary
		// path or the arguments contain special characters that might need
		// to be escaped or quoted.
		let binary_path = std::env::current_exe()?;

		// TODO(forest): The command is constructed by joining the arguments
		// with spaces. This might not work correctly if any of the
		// arguments contain spaces themselves. You might need to escape or
		// quote these arguments.
		let args = std::env::args().collect::<Vec<_>>();

		// We can get rid of the first since it's the relative path to the
		// binary. Then, we need to remove any argument that starts with
		// `--show-shell`
		let mut args = args
			.into_iter()
			.skip(1)
			.filter(|x| !x.starts_with("--show-terminal"))
			.collect::<Vec<_>>();

		// Add the binary path back as the first argument
		args.insert(0, binary_path.to_str().unwrap().to_string());

		crate::util::show_term::show_term(&args).await?;

		Ok(())
	}
}
