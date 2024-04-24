use clap::Parser;
use commands::{sidekick::PreExecuteHandled, *};
use global_error::prelude::*;
use serde_json::json;
use std::process::ExitCode;
use util::{global_config, os};

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
	telemetry_disabled: bool,
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

	/// Wraps a given command and populates the Rivet development environment
	#[clap(alias = "execute")]
	Exec(exec::Opts),

	/// Runs a script defined in rivet.yaml
	#[clap(alias = "run")]
	Run(run::Opts),

	/// Run engine-specific commands
	Engine {
		#[clap(subcommand)]
		command: engine::SubCommand,
	},

	#[clap(hide = true)]
	Sidekick {
		#[clap(subcommand)]
		command: sidekick::SubCommand,
		#[clap(long)]
		show_terminal: bool,
		/// Indicates internally that this command is being managed by the CLI
		/// and run inside a separate terminal. This is needed to prevent the
		/// CLI from printing its own status messages.
		#[clap(long, hidden = true)]
		inside_terminal: bool,
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

	/// Manages global configuration file
	GlobalConfig {
		#[clap(subcommand)]
		command: commands::global_config::SubCommand,
	},

	/// Managed OpenGB projects as well as passthrough to the OpenGB CLI
	Backend {
		#[clap(subcommand)]
		command: backend::SubCommand,
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
					rivet_term::status::error(ty, message);
				}
				GlobalError::BadRequest { code, .. } => {
					rivet_term::status::error(code, err.message());
				}
			}

			// Capture event in Sentry (only in release mode). Also, if
			// telemetry is diabled, make sure that Sentry is also disabled.
			let telemetry_disabled = global_config::read_project(|x| x.telemetry.disabled)
				.await
				.unwrap_or(false);

			if !telemetry_disabled && cfg!(feature = "sentry") {
				// TODO: Add system context
				let event = match &err {
					GlobalError::Internal {
						ty,
						message,
						debug,
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
									.and_then(|x| {
										serde_json::from_str::<serde_json::Value>(&x).ok()
									})
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
						event.insert_prop("$sentry_url", format!("https://sentry.io/organizations/rivet-gaming/issues/?project=4506447486976000&query={event_id}"))?;

						Ok(())
					}),
				)
				.await;
				if let Err(err) = capture_res {
					eprintln!("Failed to capture event in PostHog: {:?}", err);
				}
			}

			ExitCode::FAILURE
		}
	};

	util::telemetry::wait_all().await;

	exit_code
}

async fn handle_opts() -> GlobalResult<()> {
	let term = console::Term::stderr();

	// Check if passthrough
	if let Err(err) = backend::SubCommand::try_parse_from(std::env::args().skip(1)) {
		if let clap::error::ErrorKind::UnknownArgument = err.kind() {
			let (_, clap::error::ContextValue::String(usage)) = err
				.context()
				.find(|(kind, _)| matches!(kind, clap::error::ContextKind::Usage))
				.unwrap()
			else {
				unreachable!();
			};

			if usage.ends_with("backend <SUBCOMMAND>") {
				return backend::SubCommand::passthrough(&term).await;
			}
		}
	}

	// Read opts
	let opts = read_opts().await?;

	// Handle commands that need to run before we have the context
	match &opts.command {
		SubCommand::Init(init_opts) => {
			return init_opts.execute(&term).await;
		}
		SubCommand::Unlink(opts) => {
			return opts.execute().await;
		}
		SubCommand::GlobalConfig { command } => {
			return command.execute().await;
		}
		_ => {}
	}

	// Read token
	let (api_endpoint, token) =
		global_config::read_project(|x| (x.cluster.api_endpoint.clone(), x.tokens.cloud.clone()))
			.await?;

	// Sidekick sign-in can also be called before the token is validated
	if let SubCommand::Sidekick {
		command,
		inside_terminal,
		..
	} = &opts.command
	{
		if let Ok(PreExecuteHandled::Yes) = command.pre_execute(&token, *inside_terminal).await {
			return Ok(());
		}
	}

	let Some(token) = token else {
		if !os::is_linux_and_root() {
			rivet_term::status::error("Unauthenticated", "Run `rivet init` to authenticate");
		} else {
			// On Linux, the config is stored in $HOME/.config/rivet/config.yaml. When using sudo,
			// $HOME is not the same the normal user, so it won't be able to find the config.
			rivet_term::status::error(
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
		SubCommand::Init(_) | SubCommand::Unlink(_) | SubCommand::GlobalConfig { .. } => {
			unreachable!()
		}
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
		SubCommand::Exec(opts) => opts.execute(&ctx).await?,
		SubCommand::Run(opts) => opts.execute(&ctx).await?,
		SubCommand::Engine { command } => command.execute(&ctx).await?,
		SubCommand::Unreal { command } => command.execute(&ctx).await?,
		SubCommand::CI { command } => command.execute(&ctx).await?,
		SubCommand::Backend { command } => command.execute(&ctx).await?,
		SubCommand::Sidekick {
			command,
			show_terminal,
			inside_terminal,
		} => {
			command
				.execute(&ctx, &term, show_terminal, inside_terminal)
				.await?
		}
	}

	Ok(())
}

/// Reads options from clap and reads/updates the internal config file.
async fn read_opts() -> GlobalResult<Opts> {
	let opts = Opts::parse();

	global_config::mutate_project(|config| {
		if let Some(api_endpoint) = &opts.api_endpoint {
			let api_endpoint = crate::util::api::normalize_api_endpoint(&api_endpoint)?;
			config.cluster.api_endpoint = Some(api_endpoint.clone());
		}

		if let Some(token) = &opts.token {
			config.tokens.cloud = Some(token.clone());
		}

		if opts.telemetry_disabled {
			config.telemetry.disabled = true;
		}

		GlobalResult::Ok(())
	})
	.await??;

	Ok(opts)
}
