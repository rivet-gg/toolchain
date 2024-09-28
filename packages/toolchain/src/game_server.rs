use lazy_static::lazy_static;
use std::{sync::Arc, time::Duration};

use crate::util::process_manager::ProcessManager;

pub const VERSION_BUILD_TAG: &str = "version";
pub const ENABLED_BUILD_TAG: &str = "enabled";
pub const CURRENT_BUILD_TAG: &str = "current";

lazy_static! {
	pub static ref PROCESS_MANAGER: Arc<ProcessManager> =
		ProcessManager::new("game_server", Duration::from_secs(2));
}
