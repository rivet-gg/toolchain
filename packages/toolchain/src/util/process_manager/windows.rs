use anyhow::*;
use windows::Win32::{
	Foundation::{CloseHandle, HANDLE},
	System::{
		Console::{GenerateConsoleCtrlEvent, CTRL_BREAK_EVENT},
		Diagnostics::ToolHelp::{
			CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
			TH32CS_SNAPPROCESS,
		},
		ProcessStatus::{EnumProcesses, K32EnumProcessModules, K32GetModuleFileNameExW},
		Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE},
	},
};

pub fn send_terminate_signal(pid: u32) {
	unsafe {
		// Attempt to terminate the process gracefully
		if !GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32).as_bool() {
			eprintln!("failed to terminate process")
		}
	}
}

pub fn kill_process_tree(pid: u32) {
	unsafe {
		// Kill children
		let snapshot = match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
			Result::Ok(handle) => {
				let mut process_entry = PROCESSENTRY32::default();
				process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

				if Process32First(snapshot, &mut process_entry).as_bool() {
					loop {
						if process_entry.th32ParentProcessID == pid {
							if let Err(e) = kill_process_tree(process_entry.th32ProcessID) {
								eprintln!(
									"Failed to kill child process {}: {}",
									process_entry.th32ProcessID, e
								);
							}
						}
						if !Process32Next(snapshot, &mut process_entry).as_bool() {
							break;
						}
					}
				}

				CloseHandle(snapshot);
			}
			Err(e) => {
				eprintln!("Failed to create process snapshot: {}", e);
			}
		};

		// Kill parent
		let process_handle = OpenProcess(PROCESS_TERMINATE, false, pid);
		if process_handle.is_invalid() {
			if TerminateProcess(process_handle, 1).as_bool() {
				CloseHandle(process_handle);
			} else {
				let error = std::io::Error::last_os_error();
				CloseHandle(process_handle);
				eprintln!("Failed to terminate process {}: {}", pid, error);
			}
		} else {
			eprintln!("Failed to open process {}: Process may not exist", pid);
		}
	}
}
