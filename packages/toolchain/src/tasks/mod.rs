pub mod auth;
pub mod backend;
pub mod deploy;
pub mod exec_command;
pub mod game_server;
pub mod get_bootstrap_data;
pub mod get_settings_paths;
pub mod open;
pub mod postgres;
pub mod show_term;

crate::task_registry!(
	auth::check_state::Task,
	auth::sign_out::Task,
	auth::start_sign_in::Task,
	auth::wait_for_sign_in::Task,
	backend::start::Task,
	backend::stop::Task,
	deploy::Task,
	exec_command::Task,
	game_server::hook::Task,
	game_server::start::Task,
	game_server::stop::Task,
	get_bootstrap_data::Task,
	get_settings_paths::Task,
	open::Task,
	postgres::reset::Task,
	postgres::start::Task,
	postgres::status::Task,
	postgres::stop::Task,
	show_term::Task,
);
