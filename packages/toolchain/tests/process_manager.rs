use std::{sync::Arc, time::Duration};

use anyhow::*;
use rivet_toolchain::util::{
	process_manager::*,
	task::{TaskCtx, TaskCtxInner, TaskEvent},
};
use tokio::{
	sync::{broadcast, mpsc},
	time::sleep,
};

async fn setup_test_environment() -> Result<Arc<ProcessManager>> {
	Ok(ProcessManager::new("test_process", Duration::from_secs(5)))
}

fn create_task_ctx() -> (TaskCtx, mpsc::UnboundedReceiver<TaskEvent>) {
	let (log_tx, log_rx) = mpsc::unbounded_channel();
	let (_shutdown_tx, shutdown_rx) = broadcast::channel(1);
	let task = TaskCtxInner::new(log_tx, shutdown_rx);
	(task, log_rx)
}

fn build_deno_cmd(script: &str) -> (String, Vec<String>) {
	(
		"deno".to_string(),
		vec![
			"eval".to_string(),
			"--quiet".to_string(),
			script.to_string(),
		],
	)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_process_manager_lifecycle() -> Result<()> {
	let process_manager = setup_test_environment().await?;

	// Create a TaskCtx
	let (task, mut log_rx) = create_task_ctx();

	// Build command
	let script = r#"
        console.log(`ENV_VAR: ${Deno.env.get("ENV_VAR")}`);
        console.log("stdout test");
        console.error("stderr test");
        await new Promise(resolve => setTimeout(resolve, 2000));
        console.log("exiting now");
        Deno.exit(42);
    "#;

	let (command, args) = build_deno_cmd(script);

	let envs = vec![("ENV_VAR".to_string(), "test_value".to_string())];
	let current_dir = std::env::current_dir()?.to_string_lossy().to_string();

	// Start the process
	let handle = tokio::spawn({
		let process_manager = process_manager.clone();
		async move {
			process_manager
				.start(task, move || async move {
					Ok(CommandOpts {
						command,
						args,
						envs,
						current_dir,
					})
				})
				.await
		}
	});

	// Collect logs
	let mut stdout_logs = Vec::new();
	let mut stderr_logs = Vec::new();
	while let Some(event) = log_rx.recv().await {
		match event {
			TaskEvent::Log(log) => {
				if let Some(log) = log.strip_prefix("[stdout] ") {
					stdout_logs.push(log.to_string());
				} else if let Some(log) = log.strip_prefix("[stderr] ") {
					stderr_logs.push(log.to_string());
				}
			}
			TaskEvent::Result { .. } => break,
			_ => {}
		}
	}

	// Wait for the process to finish and get the exit code
	let exit_code = handle.await??;

	// Verify process is not running
	assert!(!process_manager.is_running().await?);

	// Verify exit code
	assert_eq!(exit_code, Some(42));

	// Verify logs
	assert_eq!(
		stdout_logs,
		vec!["ENV_VAR: test_value", "stdout test", "exiting now",]
	);
	assert_eq!(stderr_logs, vec!["stderr test",]);

	// Restart the process
	let script = r#"
        console.log("Restarted process");
        await new Promise(resolve => setTimeout(resolve, 2000));
        Deno.exit(0);
    "#;

	let (command, args) = build_deno_cmd(script);

	let envs = Vec::new();
	let current_dir = std::env::current_dir()?.to_string_lossy().to_string();

	let (task, _log_rx) = create_task_ctx();
	let handle = tokio::spawn({
		let process_manager = process_manager.clone();
		async move {
			process_manager
				.start(task, move || async move {
					Ok(CommandOpts {
						command,
						args,
						envs,
						current_dir,
					})
				})
				.await
		}
	});

	// Wait a bit to ensure the process has started
	sleep(Duration::from_millis(200)).await;

	// Verify process is running
	assert!(process_manager.is_running().await?);

	// Wait for the process to finish
	let exit_code = handle.await??;

	// Verify exit code of restarted process
	assert_eq!(exit_code, Some(0));

	// Verify process is not running after completion
	assert!(!process_manager.is_running().await?);

	Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_process_manager_stop_graceful() -> Result<()> {
	let process_manager = setup_test_environment().await?;

	// Create a TaskCtx
	let (task, _log_rx) = create_task_ctx();

	// Start a long-running process with custom exit code on SIGTERM
	let script = r#"
        const signal = Deno.build.os === "windows" ? "SIGBREAK" : "SIGTERM";
        Deno.addSignalListener(signal, () => {
            console.log("Exiting with code 42");
            Deno.exit(42);
        });
        console.log("Starting long process");
        while (true) {
            await new Promise(resolve => setTimeout(resolve, 1000));
        }
    "#;

	let (command, args) = build_deno_cmd(script);

	let envs = Vec::new();
	let current_dir = std::env::current_dir()?.to_string_lossy().to_string();

	// Start the process
	let handle = tokio::spawn({
		let process_manager = process_manager.clone();
		async move {
			process_manager
				.start(task, move || async move {
					Ok(CommandOpts {
						command,
						args,
						envs,
						current_dir,
					})
				})
				.await
		}
	});

	// Wait a bit to ensure the process has started
	sleep(Duration::from_millis(200)).await;

	// Verify process is running
	assert!(process_manager.is_running().await?, "process not running");
	assert!(!handle.is_finished(), "handle not running");

	// Stop the process
	assert!(process_manager.stop().await?, "did not stop process");
	assert!(
		!process_manager.stop().await?,
		"stop should not return true if no process"
	);

	// Verify process is not running
	assert!(
		!process_manager.is_running().await?,
		"process is still running"
	);

	// Wait for the process to finish and get the exit code with a 1 second timeout
	let exit_code = tokio::time::timeout(Duration::from_secs(1), handle)
		.await
		.expect("timeout waiting for process to finish")??;

	// Verify custom exit code
	assert_eq!(exit_code, Some(42));

	Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_process_manager_stop_timeout() -> Result<()> {
	let process_manager = setup_test_environment().await?;

	// Create a TaskCtx
	let (task, _log_rx) = create_task_ctx();

	// Start a process that ignores SIGTERM
	let script = r#"
        const signal = Deno.build.os === "windows" ? "SIGBREAK" : "SIGTERM";
        Deno.addSignalListener(signal, () => {
            console.log("Caught term, ignoring");
        });
        console.log("Starting process that ignores SIGTERM");
        while (true) {
            await new Promise(resolve => setTimeout(resolve, 1000));
        }
    "#;

	let (command, args) = build_deno_cmd(script);

	let envs = Vec::new();
	let current_dir = std::env::current_dir()?.to_string_lossy().to_string();

	// Start the process
	let handle = tokio::spawn({
		let process_manager = process_manager.clone();
		async move {
			process_manager
				.start(task, move || async move {
					Ok(CommandOpts {
						command,
						args,
						envs,
						current_dir,
					})
				})
				.await
		}
	});

	// Wait a bit to ensure the process has started
	sleep(Duration::from_millis(200)).await;

	// Verify process is running
	assert!(process_manager.is_running().await?, "process not running");

	// Attempt to stop the process in the background
	let stop_handle = tokio::spawn({
		let process_manager = process_manager.clone();
		async move { process_manager.stop().await }
	});

	// Verify the process is still running before the end of the grace period
	sleep(process_manager.kill_grace - Duration::from_millis(500)).await;
	assert!(
		process_manager.is_running().await?,
		"process stopped too early"
	);

	// Verify process is killed after the grace period
	//
	// We've already slept `grace - 500ms` at this point, so we'll check
	// 500ms after the expected kill
	sleep(Duration::from_secs(1)).await;
	assert!(
		!process_manager.is_running().await?,
		"process is still running"
	);

	// Wait for the stop task to complete and check its result
	let stop_result = stop_handle.await?;
	assert!(stop_result?, "did not stop process");

	// Wait for the process to finish and get the exit code
	let exit_code = handle.await??;

	// Verify exit code. This is None on Unix bc SIGKILL terminates immediately.
	// This is Some(1) on Windows bc TerminateProcess exits with 1.
	assert_eq!(exit_code, None, "Unexpected exit code");

	Ok(())
}
