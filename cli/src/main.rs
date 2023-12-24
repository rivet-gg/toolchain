use clap::Parser;
use commands::*;
use global_error::prelude::*;
use serde_json::json;
use std::{collections::BTreeMap, process::ExitCode};
use util::{
	global_config, os,
	struct_fmt::{self, Format},
	term,
};

mod commands;
mod util;

// IMPORTANT: Do not read `api_endpoint`, `token`, and `telemetry_disabled` directly from `opts`. These properties are written to the config. Use `global_config::read_project`.
#[derive(Parser)]
#[clap(
	author = "Rivet Gaming, Inc. <developer@rivet.gg>",
	about = "https://rivet.gg/",
	version = concat!(env!("VERGEN_BUILD_SEMVER"), " (", env!("VERGEN_GIT_SHA_SHORT"), ")"),
	long_version = concat!(
		"\n\n",
		"git sha: ", env!("VERGEN_GIT_SHA"), "\n",
		"git branch: ", env!("VERGEN_GIT_BRANCH"), "\n",
		"build semver: ", env!("VERGEN_BUILD_SEMVER"), "\n",
		"build timestamp: ", env!("VERGEN_BUILD_TIMESTAMP"), "\n",
		"build target: ", env!("VERGEN_CARGO_TARGET_TRIPLE"), "\n",
		"build profile: ", env!("VERGEN_CARGO_PROFILE"), "\n",
		"rustc version: ", env!("VERGEN_RUSTC_SEMVER"),
	)
)]
struct Opts {
	#[clap(subcommand)]
	command: SubCommand,

	#[clap(global = true, long, env = "RIVET_API_ENDPOINT")]
	api_endpoint: Option<String>,

	#[clap(global = true, long, env = "RIVET_TOKEN")]
	token: Option<String>,

	#[clap(global = true, long, env = "RIVET_TELEMETRY_DISABLED")]
	telemetry_disabled: Option<bool>,
}

#[derive(Parser)]
enum SubCommand {
	/// Guided setup for this project
	Init(init::Opts),

	/// Removes configured authentication token for this project
	Unlink(unlink::Opts),

	/// Pushes required resources and creates a new version
	#[clap(alias = "publish")]
	Deploy(deploy::Opts),

	/// Manages rivet.yaml config
	Config {
		#[clap(subcommand)]
		command: config::SubCommand,
	},

	/// Manages tokens
	Token {
		#[clap(subcommand)]
		command: token::SubCommand,
	},

	/// Manages the game
	Game {
		#[clap(subcommand)]
		command: game::SubCommand,
	},

	/// Manages namespaces
	#[clap(alias = "ns")]
	Namespace {
		#[clap(subcommand)]
		command: ns::SubCommand,
	},

	/// Manages versions
	Version {
		#[clap(subcommand)]
		command: version::SubCommand,
	},

	/// Manages builds for Serverless Lobbies
	#[clap(alias = "image", alias = "build")]
	Docker {
		#[clap(subcommand)]
		command: docker::SubCommand,
	},

	/// Manages sites for the CDN
	#[clap(alias = "site")]
	CDN {
		#[clap(subcommand)]
		command: cdn::SubCommand,
	},

	/// Run engine-specific commands
	Engine {
		#[clap(subcommand)]
		command: engine::SubCommand,
	},

	#[clap(hide = true)]
	Sidekick {
		#[clap(subcommand)]
		command: sidekick::SubCommand,
	},

	/// Alias of `rivet engine unreal`
	#[clap(hide = true, alias = "ue")]
	Unreal {
		#[clap(subcommand)]
		command: engine::unreal::SubCommand,
	},

	/// Run CI-specific commands
	CI {
		#[clap(subcommand)]
		command: ci::SubCommand,
	},

	/// Deprecated.
	///
	/// Manages identity avatars
	#[clap(hide = true)]
	IdentityAvatar {
		#[clap(subcommand)]
		command: avatar::SubCommand,
	},

	/// Deprecated.
	///
	/// Initiates the development environment for this project.
	#[clap(hide = true)]
	Dev {
		#[clap(subcommand)]
		command: dev::SubCommand,
	},
}

fn main() -> ExitCode {
	// We use a sync main for Sentry. Read more: https://docs.sentry.io/platforms/rust/#async-main-function

	// This has a 2 second deadline to flush any remaining events which is sufficient for
	// short-lived commands.
	let _guard = sentry::init(("https://05632d74d4bc90958ed4ada487acfd8e@o4504307129188352.ingest.sentry.io/4506447486976000", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

	// Run main
	let exit_code = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(async move { main_async().await });

	exit_code
}

async fn main_async() -> ExitCode {
	let res = handle_opts().await;

	// Blanket catch for all errors
	let exit_code = match res {
		Ok(_) => ExitCode::SUCCESS,
		Err(err) => {
			// Pretty-print error
			match &err {
				GlobalError::Internal { ty, message, .. } => {
					term::status::error(ty, message);
				}
				GlobalError::BadRequest { code, .. } => {
					term::status::error(code, err.message());
				}
			}

			// Capture event in Sentry
			// TODO: Add system context
			let event = match &err {
				GlobalError::Internal {
					ty,
					message,
					debug,
					code,
					retry_immediately: _,
				} => sentry::protocol::Event {
					level: sentry::protocol::Level::Error,
					exception: sentry::protocol::Values {
						values: vec![sentry::protocol::Exception {
							ty: ty.clone(),
							value: Some(debug.clone()),
							..Default::default()
						}],
					},
					message: Some(message.clone()),
					tags: vec![(
						"internal_error_code".to_string(),
						Into::<i32>::into(*code).to_string(),
					)]
					.into_iter()
					.collect(),
					..Default::default()
				},
				GlobalError::BadRequest {
					code,
					context,
					metadata,
				} => sentry::protocol::Event {
					level: sentry::protocol::Level::Warning,
					exception: sentry::protocol::Values {
						values: vec![sentry::protocol::Exception {
							ty: "BadRequest".into(),
							value: Some(code.clone()),
							..Default::default()
						}],
					},
					message: Some(err.message()),
					extra: vec![
						("context".to_string(), json!(context)),
						(
							"metadata".to_string(),
							metadata
								.as_ref()
								.and_then(|x| serde_json::from_str::<serde_json::Value>(&x).ok())
								.unwrap_or_else(|| serde_json::Value::Null),
						),
					]
					.into_iter()
					.collect(),
					..Default::default()
				},
			};
			let event_id = sentry::capture_event(event);

			// Capture event in PostHog
			let capture_res = util::telemetry::capture_event(
			util::telemetry::GAME_ID.get(),
			"$exception",
			Some(|event: &mut async_posthog::Event| {
				event.insert_prop("errors", format!("{}", err))?;

				event.insert_prop("$sentry_event_id", event_id.to_string())?;
				event.insert_prop("$sentry_url", format!("https://sentry.io/organizations/rivet-gg/issues/?project=4506447486976000&query={event_id}"))?;

				Ok(())
			}),
		).await;
			if let Err(err) = capture_res {
				eprintln!("Failed to capture event in PostHog: {:?}", err);
			}

			ExitCode::FAILURE
		}
	};

	util::telemetry::wait_all().await;

	exit_code
}

async fn handle_opts() -> GlobalResult<()> {
	let term = console::Term::stderr();

	// Read opts
	let opts = read_opts().await?;

	// Handle init command without the context
	if let SubCommand::Init(init_opts) = &opts.command {
		return init_opts.execute(&term).await;
	}

	// Read token
	let (api_endpoint, token) =
		global_config::read_project(|x| (x.cluster.api_endpoint.clone(), x.tokens.cloud.clone()))
			.await?;

	// Sidekick sign-in can also be called before the token is valitdated
	if let SubCommand::Sidekick { command } = &opts.command {
		let response: GlobalResult<_> = match command {
			sidekick::SubCommand::GetLink { .. } => command.get_link().await,
			sidekick::SubCommand::WaitForLogin { device_link_token } => {
				command.wait_for_login(device_link_token).await
			}
			sidekick::SubCommand::CheckLoginState => command.validate_token(&token),
			_ => {
				// If the command is anything else, we need to check if a token
				// has already been provided. If not, we need to print an error
				// and return early since that's what the plugins will expect.
				if let Err(_) = command.validate_token(&token) {
					// The message has already been printed out so we can just
					// return Ok here.
					Ok(sidekick::SideKickResponse(json!({
						"output": "Token not found. Please run `rivet sidekick get-link` to sign in."
					})))
				} else {
					Ok(sidekick::SideKickResponse(json!({})))
				}
			}
		};

		// Print the response
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

		return Ok(());
	}

	let Some(token) = token else {
		if !os::is_linux_and_root() {
			term::status::error("Unauthenticated", "Run `rivet init` to authenticate");
		} else {
			// On Linux, the config is stored in $HOME/.config/rivet/config.yaml. When using sudo,
			// $HOME is not the same the normal user, so it won't be able to find the config.
			term::status::error(
                "Unauthenticated with sudo",
				"Please rerun this command without sudo or run `sudo rivet init` to authenticate as root",
			);
		}

		bail!("rivet token not found")
	};

	let ctx = cli_core::ctx::init(api_endpoint, token).await?;

	// Set game id for errors
	util::telemetry::GAME_ID.set(ctx.game_id.clone())?;

	// Handle command
	match opts.command {
		SubCommand::Init(_) => unreachable!(),
		SubCommand::Unlink(opts) => opts.execute(&ctx).await?,
		SubCommand::Deploy(opts) => opts.execute(&ctx).await?,
		SubCommand::Config { command } => command.execute(&ctx).await?,
		SubCommand::IdentityAvatar { command } => command.execute(&ctx).await?,
		SubCommand::Dev { command } => command.execute(&ctx).await?,
		SubCommand::Token { command } => command.execute(&ctx).await?,
		SubCommand::Game { command } => command.execute(&ctx).await?,
		SubCommand::Namespace { command } => command.execute(&ctx).await?,
		SubCommand::Version { command } => command.execute(&ctx).await?,
		SubCommand::Docker { command } => command.execute(&ctx).await?,
		SubCommand::CDN { command } => command.execute(&ctx).await?,
		SubCommand::Engine { command } => command.execute(&ctx).await?,
		SubCommand::Unreal { command } => command.execute(&ctx).await?,
		SubCommand::CI { command } => command.execute(&ctx).await?,
		SubCommand::Sidekick { command } => match command.execute(&ctx, &term).await {
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
		},
	}

	Ok(())
}

/// Reads options from clap and reads/updates the internal config file.
async fn read_opts() -> GlobalResult<Opts> {
	let opts = Opts::parse();

	global_config::mutate_project(|config| {
		if let Some(api_endpoint) = &opts.api_endpoint {
			config.cluster.api_endpoint = Some(api_endpoint.clone());
		}

		if let Some(token) = &opts.token {
			config.tokens.cloud = Some(token.clone());
		}

		if let Some(telemetry_disabled) = opts.telemetry_disabled {
			config.telemetry.disabled = telemetry_disabled;
		}
	})
	.await?;

	Ok(opts)
}
