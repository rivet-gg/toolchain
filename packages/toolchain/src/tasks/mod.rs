pub mod backend_start;
pub mod backend_stop;
pub mod check_login_state;
pub mod deploy;
pub mod exec_command;
pub mod game_server_start;
pub mod game_server_stop;
pub mod get_bootstrap_data;
pub mod get_settings_paths;
pub mod open;
pub mod postgres_reset;
pub mod postgres_start;
pub mod postgres_status;
pub mod postgres_stop;
pub mod show_term;
pub mod start_device_link;
pub mod unlink;
pub mod wait_for_login;

crate::task_registry!(
	backend_start::Task,
	backend_stop::Task,
	check_login_state::Task,
	deploy::Task,
	exec_command::Task,
	game_server_start::Task,
	game_server_stop::Task,
	get_bootstrap_data::Task,
	get_settings_paths::Task,
	open::Task,
	postgres_reset::Task,
	postgres_start::Task,
	postgres_status::Task,
	postgres_stop::Task,
	show_term::Task,
	start_device_link::Task,
	unlink::Task,
	wait_for_login::Task,
);
