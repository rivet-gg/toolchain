use anyhow::*;
use nix::{
	sys::signal::{kill, Signal},
	unistd::Pid,
};

pub fn send_terminate_signal(pid: u32) {
	match kill(Pid::from_raw(pid as i32), Signal::SIGTERM) {
		Result::Ok(_) => (),
		Err(e) => eprintln!("Failed to send SIGTERM to process {}: {}", pid, e),
	}
}

/// Kill process tree using pkill on macOS. This is not portable to Linux distros.
#[cfg(target_os = "macos")]
pub fn kill_process_tree(pid: u32) {
	use std::process::Command;

	// Kill the children
	match Command::new("pkill")
		.args(&["-9", "-P", &pid.to_string()])
		.status()
	{
		Result::Ok(status) => {
			if !status.success() {
				eprintln!("pkill failed to kill children: status {status}");
			}
		}
		Err(err) => {
			eprintln!("pkill failed to kill children: {err}");
		}
	}

	// Kill the parent
	match Command::new("kill")
		.args(&["-9", &pid.to_string()])
		.status()
	{
		Result::Ok(status) => {
			if !status.success() {
				eprintln!("kill failed to kill parent: status {status}");
			}
		}
		Err(err) => {
			eprintln!("kill failed to kill parent: {err:?}");
		}
	};
}

// TODO: Figure out how to make libproc work with osxcross
// #[cfg(target_os = "macos")]
// pub fn kill_process_tree(pid_raw: u32) {
// 	use libproc::processes::{pids_by_type, ProcFilter};
//
// 	let pid = Pid::from_raw(pid_raw as i32);
//
// 	match pids_by_type(ProcFilter::ByParentProcess { ppid: pid_raw }) {
// 		Result::Ok(pids) => {
// 			for &child_pid in &pids {
// 				kill_process_tree(child_pid);
// 			}
// 		}
// 		Err(e) => {
// 			eprintln!("Failed to list child processes for {}: {}", pid_raw, e);
// 		}
// 	};
//
// 	if let Err(e) = kill(pid, Signal::SIGKILL) {
// 		eprintln!("Failed to kill process {}: {}", pid_raw, e);
// 	}
// }

/// Kill process tree using `/proc/` on Linux.
#[cfg(target_os = "linux")]
pub fn kill_process_tree(pid_raw: u32) {
	use std::{fs, path::Path};

	let pid = Pid::from_raw(pid_raw as i32);

	let proc_dir = format!("/proc/{}/task/{}/children", pid, pid);
	if Path::new(&proc_dir).exists() {
		match fs::read_to_string(&proc_dir) {
			Result::Ok(children) => {
				for child_pid in children.split_whitespace() {
					if let Result::Ok(child_pid) = child_pid.parse::<u32>() {
						kill_process_tree(child_pid);
					}
				}
			}
			Err(e) => eprintln!("Failed to read child processes for {}: {}", pid_raw, e),
		}
	}

	if let Err(e) = kill(pid, Signal::SIGKILL) {
		eprintln!("Failed to kill process {}: {}", pid_raw, e);
	}
}
