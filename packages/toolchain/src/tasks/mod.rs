pub mod auth;
pub mod deploy;
pub mod get_bootstrap_data;

crate::task_registry!(
	auth::check_state::Task,
	auth::sign_out::Task,
	auth::start_sign_in::Task,
	auth::wait_for_sign_in::Task,
	deploy::Task,
	get_bootstrap_data::Task,
);
