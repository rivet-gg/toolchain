mod runtime;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{
	collections::HashMap,
	ffi::{CStr, CString},
	os::raw::c_char,
	sync::atomic::{AtomicU64, Ordering},
	sync::{mpsc, Mutex},
};
use tokio::sync::mpsc as tokio_mpsc;
use toolchain::util::task;

static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
type TaskId = u64;

struct TaskHandle {
	abort_tx: tokio_mpsc::Sender<()>,
	event_rx: Receiver<task::TaskEvent>,
}

lazy_static::lazy_static! {
	static ref TASK_HANDLES: Mutex<HashMap<TaskId, TaskHandle>> = Mutex::new(HashMap::new());
}

#[repr(u8)]
pub enum ErrorCode {
	Success = 0,
	NullPointer = 1,
	ParseError = 2,
	LockError = 3,
	CStringNew = 4,
	TaskNotFound = 5,
}

#[repr(C)]
pub struct RunTaskResult {
	task_id: TaskId,
	error_code: u8,
}

#[no_mangle]
pub extern "C" fn rivet_run_task(name: *const c_char, input_json: *const c_char) -> RunTaskResult {
	match inner_run_task(name, input_json) {
		Ok(task_id) => RunTaskResult {
			task_id,
			error_code: ErrorCode::Success as u8,
		},
		Err(error_code) => RunTaskResult {
			task_id: 0,
			error_code: error_code as u8,
		},
	}
}

fn inner_run_task(name: *const c_char, input_json: *const c_char) -> Result<TaskId, ErrorCode> {
	// Handle null pointers
	if name.is_null() || input_json.is_null() {
		return Err(ErrorCode::NullPointer);
	}

	// Parse input
	let name = unsafe { CStr::from_ptr(name).to_str() }
		.map_err(|_| ErrorCode::ParseError)?
		.to_string();
	let input_json = unsafe { CStr::from_ptr(input_json).to_str() }
		.map_err(|_| ErrorCode::ParseError)?
		.to_string();

	runtime::setup();

	// Setup task
	let task_id = TASK_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
	let (output_tx, output_rx) = mpsc::channel();
	let (run_config, mut handles) = task::RunConfig::build();

	// Store abort sender and event receiver
	TASK_HANDLES
		.lock()
		.map_err(|_| ErrorCode::LockError)?
		.insert(
			task_id,
			TaskHandle {
				abort_tx: handles.abort_tx.clone(),
				event_rx: output_rx,
			},
		);

	// Run the task
	runtime::spawn(Box::pin(async move {
		// Spawn task
		tokio::task::spawn(async move {
			toolchain::tasks::run_task_json(run_config, &name, &input_json).await;
		});

		// Pass events to the sync context
		while let Some(event) = handles.event_rx.recv().await {
			match output_tx.send(event) {
				Ok(_) => {}
				Err(_) => {
					// Abort on receiver dropped
					break;
				}
			}
		}
	}));

	Ok(task_id)
}

#[repr(C)]
pub struct TaskEvent {
	task_id: TaskId,
	event_json: *mut c_char,
}

#[repr(C)]
pub struct PollTaskEventsResult {
	count: usize,
	error_code: u8,
}

#[no_mangle]
pub extern "C" fn rivet_poll_task_events(
	events: *mut TaskEvent,
	max_events: usize,
) -> PollTaskEventsResult {
	match inner_poll_task_events(events, max_events) {
		Ok(count) => PollTaskEventsResult {
			count,
			error_code: ErrorCode::Success as u8,
		},
		Err(error_code) => PollTaskEventsResult {
			count: 0,
			error_code: error_code as u8,
		},
	}
}

fn inner_poll_task_events(events: *mut TaskEvent, max_events: usize) -> Result<usize, ErrorCode> {
	let mut task_handles = TASK_HANDLES.lock().map_err(|_| ErrorCode::LockError)?;

	let mut count = 0;
	let mut completed_tasks = Vec::new();

	for (task_id, handle) in task_handles.iter_mut() {
		match handle.event_rx.try_recv() {
			Ok(event) => {
				let event_json =
					serde_json::to_string(&event).map_err(|_| ErrorCode::ParseError)?;

				// Store event in TaskEvent
				let event_ptr = CString::new(event_json)
					.map_err(|_| ErrorCode::CStringNew)?
					.into_raw();
				unsafe {
					if events.is_null() {
						return Err(ErrorCode::NullPointer);
					}
					(*events.add(count)).task_id = *task_id;
					(*events.add(count)).event_json = event_ptr;
				}

				count += 1;
				if count >= max_events {
					break;
				}
			}
			Err(TryRecvError::Empty) => continue,
			Err(TryRecvError::Disconnected) => {
				completed_tasks.push(*task_id);
			}
		}
	}

	// Remove completed tasks
	for task_id in completed_tasks {
		task_handles.remove(&task_id);
	}

	Ok(count)
}

#[no_mangle]
pub extern "C" fn rivet_abort_task(task_id: TaskId) -> u8 {
	match TASK_HANDLES.lock() {
		Result::Ok(mut lock) => {
			if let Some(handle) = lock.remove(&task_id) {
				runtime::spawn(async move {
					let _ = handle.abort_tx.send(()).await;
				});
				ErrorCode::Success as u8
			} else {
				ErrorCode::TaskNotFound as u8
			}
		}
		Err(_) => ErrorCode::LockError as u8,
	}
}

#[no_mangle]
pub extern "C" fn rivet_shutdown() {
	runtime::shutdown();
}

#[no_mangle]
pub extern "C" fn rivet_free_rust_string(s: *mut c_char) {
	unsafe {
		if s.is_null() {
			return;
		}
		let _ = CString::from_raw(s);
	};
}
