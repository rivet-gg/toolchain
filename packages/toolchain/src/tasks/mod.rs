pub mod backend_choose_local_port;
pub mod backend_dev;
pub mod backend_sdk_gen;
pub mod check_login_state;
pub mod check_system_requirements;
pub mod deploy;
pub mod exec_command;
pub mod get_bootstrap_data;
pub mod get_hub_link;
pub mod get_settings_paths;
pub mod open;
pub mod show_term;
pub mod start_device_link;
pub mod unlink;
pub mod wait_for_login;

crate::task_registry!(
	backend_choose_local_port::Task,
	backend_dev::Task,
	backend_sdk_gen::Task,
	check_login_state::Task,
	check_system_requirements::Task,
	deploy::Task,
	exec_command::Task,
	get_bootstrap_data::Task,
	get_hub_link::Task,
	open::Task,
	show_term::Task,
	start_device_link::Task,
	unlink::Task,
	wait_for_login::Task,
	get_settings_paths::Task,
);
