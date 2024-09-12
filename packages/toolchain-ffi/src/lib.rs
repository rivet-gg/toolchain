mod runtime;

use std::{
	collections::HashMap,
	ffi::{CStr, CString},
	os::raw::c_char,
	sync::atomic::{AtomicU64, Ordering},
	sync::{mpsc, Mutex},
	thread,
};
use tokio::sync::mpsc as tokio_mpsc;
use toolchain::util::task;

static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
type TaskId = u64;

struct TaskHandle {
	abort_tx: tokio_mpsc::Sender<()>,
}

lazy_static::lazy_static! {
	static ref TASK_HANDLES: Mutex<HashMap<TaskId, TaskHandle>> = Mutex::new(HashMap::new());
}

/// Callback type used to receive events from tasks.
type EventCallback = extern "C" fn(TaskId, *const c_char);

#[no_mangle]
pub extern "C" fn run_task(
	name: *const c_char,
	input_json: *const c_char,
	callback: EventCallback,
) -> TaskId {
	// Parse input
	let name_tmp = unsafe { CStr::from_ptr(name).to_str().unwrap() };
	let name = name_tmp.to_string();
	let input_json_tmp = unsafe { CStr::from_ptr(input_json).to_str().unwrap() };
	let input_json = input_json_tmp.to_string();
	println!("input {name} {input_json}");

	runtime::setup();

	// Setup task
	let task_id = TASK_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
	let (output_tx, output_rx) = mpsc::channel();
	let (run_config, mut handles) = task::RunConfig::build();

	// Store abort sender
	TASK_HANDLES.lock().unwrap().insert(
		task_id,
		TaskHandle {
			abort_tx: handles.abort_tx.clone(),
		},
	);

	// Run the task
	runtime::spawn(Box::pin(async move {
		println!("input2 {name} {input_json}");

		// Spawn task
		tokio::task::spawn(async move {
			toolchain::tasks::run_task_json(run_config, &name, &input_json).await
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

	thread::spawn(move || {
		// Pass events to callback
		while let Ok(event) = output_rx.recv() {
			// Serialize event
			let event_json = match serde_json::to_string(&event) {
				Ok(x) => x,
				Err(err) => {
					eprintln!("error with event: {err:?}");
					return;
				}
			};

			// Call the callback function
			let c_str = CString::new(event_json).unwrap();
			callback(task_id, c_str.into_raw());
		}

		// Remove task handle when finished
		TASK_HANDLES.lock().unwrap().remove(&task_id);
	});

	task_id
}

#[no_mangle]
pub extern "C" fn abort_task(task_id: TaskId) -> bool {
	if let Some(handle) = TASK_HANDLES.lock().unwrap().remove(&task_id) {
		runtime::spawn(async move {
			let _ = handle.abort_tx.send(()).await;
		});
		true
	} else {
		eprintln!("failed to abort event");
		false
	}
}

#[no_mangle]
pub extern "C" fn shutdown() {
	runtime::shutdown();
}

#[no_mangle]
pub extern "C" fn free_rust_string(s: *mut c_char) {
	unsafe {
		if s.is_null() {
			return;
		}
		let _ = CString::from_raw(s);
	};
}
