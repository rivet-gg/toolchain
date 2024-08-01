use clap::Parser;
use global_error::prelude::*;
use serde::Serialize;
use std::{fs::File, process::Command};

use super::SideKickHandler;

#[derive(Parser)]
pub struct Opts {
	#[clap(long)]
	pid: i32,
}

#[derive(Serialize)]
pub struct Output {}

impl SideKickHandler for Output {}

impl Opts {
	pub async fn execute(&self) -> GlobalResult<Output> {
		send_sigint(self.pid);
		Ok(Output {})
	}
}

#[cfg(unix)]
fn send_sigint(pid: i32) -> GlobalResult<i32> {
	use nix::sys::signal::{kill, Signal};
	use nix::unistd::Pid;

	kill(Pid::from_raw(pid), Signal::SIGINT)?;
	Ok(0)
}

#[cfg(windows)]
fn send_sigint(pid: i32) -> GlobalResult<i32> {
	use std::ptr;
	use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
	use winapi::um::winnt::PROCESS_TERMINATE;

	unsafe {
		let process_handle = OpenProcess(PROCESS_TERMINATE, 0, pid as u32);
		if process_handle == ptr::null_mut() {
			return Err(GlobalError::new("Failed to open process"));
		}

		let result = TerminateProcess(process_handle, 0);
		if result == 0 {
			return Err(GlobalError::new("Failed to terminate process"));
		}

		Ok(0)
	}
}
