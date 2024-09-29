use anyhow::*;
use nix::{
	sys::signal::{kill, Signal},
	unistd::Pid,
};

pub async fn send_terminate_signal(pid: u32) -> Result<()> {
	kill(Pid::from_raw(pid as i32), Signal::SIGTERM)?;

	Ok(())
}

pub async fn kill_process_tree(pid: u32) -> Result<()> {
	let pid = Pid::from_raw(pid as i32);

	kill(pid, Signal::SIGKILL)?;

	Ok(())
}
