use anyhow::*;
use windows::Win32::{
	Foundation::{CloseHandle, HANDLE},
	System::{
		Console::{GenerateConsoleCtrlEvent, CTRL_BREAK_EVENT},
		Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE},
	},
};

pub async fn send_terminate_signal(pid: u32) -> Result<()> {
	unsafe {
		// Attempt to terminate the process gracefully
		if !GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32).as_bool() {
			bail!("failed to terminate process")
		}
	}

	Ok(())
}

pub async fn kill_process_tree(pid: u32) -> Result<()> {
	let pid = child.id().expect("Failed to get PID");

	unsafe {
		let process_handle = OpenProcess(PROCESS_TERMINATE, false, pid);
		if process_handle.is_invalid() {
			return Err("Failed to open process".into());
		}

		if TerminateProcess(process_handle, 1).as_bool() {
			CloseHandle(process_handle);
			Ok(())
		} else {
			CloseHandle(process_handle);
			bail!("Failed to terminate process");
		}
	}
}
