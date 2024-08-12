use serde_json;
use std::future::Future;
use tokio::time::Duration;
use toolchain::tasks::RunConfig;

pub fn run_task(run_config: String, name: String, input_json: String) -> String {
	let run_config = serde_json::from_str::<RunConfig>(&run_config).unwrap();
	let task_config = toolchain::tasks::get_task_config(&name);
	let name_inner = name.clone();
	let output_json = block_on(
		async move { toolchain::tasks::run_task_json(run_config, &name_inner, &input_json).await },
		BlockOnOpts {
			multithreaded: task_config.prefer_multithreaded,
		},
	);
	output_json.output
}

const FORCE_MULTI_THREAD: bool = true;

struct BlockOnOpts {
	multithreaded: bool,
}

/// Create a temporary Tokio runtime to run the given future.
fn block_on<Output>(fut: impl Future<Output = Output>, opts: BlockOnOpts) -> Output {
	// Build temporary runtime
	let mut builder = if opts.multithreaded || FORCE_MULTI_THREAD {
		tokio::runtime::Builder::new_multi_thread()
	} else {
		tokio::runtime::Builder::new_current_thread()
	};
	let rt = builder.enable_all().build().unwrap();

	// Run future
	let output = rt.block_on(fut);

	// Give tasks time to shut down
	// rt.shutdown_background();
	rt.shutdown_timeout(Duration::from_secs(1));

	output
}
