use std::{time::Duration, sync::Arc};
use lazy_static::lazy_static;

use crate::util::process_manager::ProcessManager;

lazy_static! {
    pub static ref PROCESS_MANAGER: Arc<ProcessManager> = ProcessManager::new("game_server", Duration::from_secs(2));
}
