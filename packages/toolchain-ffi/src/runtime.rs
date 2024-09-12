use lazy_static::lazy_static;
use std::{future::Future, sync::Mutex, time::Duration};

lazy_static! {
	static ref RUNTIME: Mutex<Option<tokio::runtime::Runtime>> = Mutex::new(None);
}

/// Sets up the shared runtime.
pub fn setup() {
	let mut runtime_lock = RUNTIME.lock().expect("failed to lock runtime");
	if runtime_lock.is_none() {
		let runtime = tokio::runtime::Builder::new_multi_thread()
			.worker_threads(2)
			.enable_all()
			.build()
			.expect("failed to build runtime");
		*runtime_lock = Some(runtime);
	}
}

fn get_runtime() -> Option<tokio::runtime::Handle> {
	RUNTIME
		.lock()
		.expect("failed to lock runtime")
		.as_ref()
		.map(|runtime| runtime.handle().clone())
}

/// Spawns a new future
pub fn spawn<Output: Send + 'static>(fut: impl Future<Output = Output> + Send + 'static) {
	if let Some(runtime) = get_runtime() {
		runtime.spawn(fut);
	} else {
		eprintln!("runtime shut down, cannot spawn task")
	}
}

/// Shuts down the runtime.
pub fn shutdown() {
	let mut runtime_lock = RUNTIME.lock().expect("failed to lock runtime");
	if let Some(runtime) = runtime_lock.take() {
		runtime.shutdown_timeout(Duration::from_millis(100));
	} else {
		eprintln!("runtime already shut down");
	}
}
