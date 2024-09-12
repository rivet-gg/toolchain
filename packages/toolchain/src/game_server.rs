use std::time::Duration;

use crate::util::process_manager::ProcessManager;

pub const PROCESS_MANAGER: ProcessManager = ProcessManager { key: "game_server", kill_grace: Duration::from_secs(2) };
