use anyhow::*;
use rivet_process_runner_shared as shared;
use rivet_toolchain::util::{
	process_manager::*,
	task::{TaskCtx, TaskCtxInner, TaskEvent},
};
use serde::{Deserialize, Serialize};
use std::{
	future::Future,
	process::{Command, Stdio},
	time::{Duration, Instant},
};
use tokio::fs::File;
use tokio::{
	sync::{broadcast, mpsc},
	time::sleep,
};

async fn setup_test_environment() -> Result<(ProcessManager, tempfile::TempDir)> {
	// Set up a temporary directory for the test
	let temp_dir = tempfile::tempdir()?;
	let temp_path = temp_dir.path().to_path_buf();
	eprintln!("Proc test dir: {}", temp_path.display());

	// Create a fake project in the meta file
	let api_endpoint = "https://fake.api.endpoint".to_string();
	let cloud_token = "fake_cloud_token".to_string();
	rivet_toolchain::config::meta::insert_project(&temp_path, api_endpoint, cloud_token).await?;

	// Create a ProcessManager with default kill_grace
	let process_manager = ProcessManager {
		key: "test_process",
		kill_grace: Duration::from_secs(5),
	};

	Ok((process_manager, temp_dir))
}

fn create_task_ctx() -> (TaskCtx, mpsc::UnboundedReceiver<TaskEvent>) {
	let (log_tx, log_rx) = mpsc::unbounded_channel();
	let (_shutdown_tx, shutdown_rx) = broadcast::channel(1);
	let task = TaskCtxInner::new(log_tx, shutdown_rx);
	(task, log_rx)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_process_manager_lifecycle() -> Result<()> {
	let (process_manager, temp_dir) = setup_test_environment().await?;

	// Create a TaskCtx
	let (task, mut log_rx) = create_task_ctx();

	// Build command
	#[cfg(windows)]
	let (command, args) = (
		"powershell".to_string(),
		vec![
			"-NoProfile".to_string(),
			"-Command".to_string(),
			r#"
                $env:ENV_VAR
                Write-Host 'Hello from stdout'
                Write-Error 'Error message'
                Start-Sleep -Seconds 2
                Write-Host 'Exiting now'
                exit 42
            "#
			.to_string(),
		],
	);

	#[cfg(not(windows))]
	let (command, args) = (
		"sh".to_string(),
		vec![
			"-c".to_string(),
			r#"
                echo "ENV_VAR: $ENV_VAR"
                echo 'Hello from stdout'
                echo 'Error message' >&2
                sleep 2
                echo 'Exiting now'
                exit 42
            "#
			.to_string(),
		],
	);
	let envs = vec![("ENV_VAR".to_string(), "test_value".to_string())];
	let current_dir = std::env::current_dir()?.to_string_lossy().to_string();
	let base_data_dir = temp_dir.path().to_path_buf();

	// Start the process
	let handle = tokio::spawn({
		let process_manager = process_manager.clone();
		let base_data_dir = base_data_dir.clone();
		async move {
			process_manager
				.start(
					StartOpts {
						task,
						base_data_dir,
						start_mode: StartMode::StartOrHook,
					},
					move || async move {
						Ok(CommandOpts {
							command,
							args,
							envs,
							current_dir,
						})
					},
				)
				.await
		}
	});

	// Collect logs
	let mut stdout_logs = Vec::new();
	let mut stderr_logs = Vec::new();
	while let Some(event) = log_rx.recv().await {
		match event {
			TaskEvent::Log(log) => {
				if log.contains("[stdout]") {
					stdout_logs.push(log);
				} else if log.contains("[stderr]") {
					stderr_logs.push(log);
				}
			}
			TaskEvent::Result { .. } => break,
			_ => {}
		}
	}

	// Wait for the process to finish and get the exit code
	let exit_code = handle.await??;

	// Verify process is not running
	assert!(!process_manager.is_running(&base_data_dir).await?);

	// Verify exit code
	assert_eq!(exit_code, Some(42));

	// Verify logs
	#[cfg(windows)]
	{
		assert!(stdout_logs.iter().any(|log| log.contains("test_value")));
	}
	#[cfg(not(windows))]
	{
		assert!(stdout_logs
			.iter()
			.any(|log| log.contains("ENV_VAR: test_value")));
	}
	assert!(stdout_logs
		.iter()
		.any(|log| log.contains("Hello from stdout")));
	assert!(stdout_logs.iter().any(|log| log.contains("Exiting now")));
	assert!(stderr_logs.iter().any(|log| log.contains("Error message")));

	// Restart the process
	#[cfg(windows)]
	let (command, args) = (
		"powershell".to_string(),
		vec![
			"-NoProfile".to_string(),
			"-Command".to_string(),
			"Write-Host 'Restarted process'; Start-Sleep -Seconds 2; exit 0".to_string(),
		],
	);

	#[cfg(not(windows))]
	let (command, args) = (
		"sh".to_string(),
		vec![
			"-c".to_string(),
			"echo 'Restarted process'; sleep 2; exit 0".to_string(),
		],
	);
	let envs = Vec::new();
	let current_dir = std::env::current_dir()?.to_string_lossy().to_string();
	let base_data_dir = temp_dir.path().to_path_buf();

	let (task, _log_rx) = create_task_ctx();
	let handle = tokio::spawn({
		let process_manager = process_manager.clone();
		let base_data_dir = base_data_dir.clone();
		async move {
			process_manager
				.start(
					StartOpts {
						task,
						base_data_dir,
						start_mode: StartMode::StartOrHook,
					},
					move || async move {
						Ok(CommandOpts {
							command,
							args,
							envs,
							current_dir,
						})
					},
				)
				.await
		}
	});

	// Wait a bit to ensure the process has started
	sleep(Duration::from_millis(200)).await;

	// Verify process is running
	assert!(process_manager.is_running(&base_data_dir).await?);

	// Wait for the process to finish
	let exit_code = handle.await??;

	// Verify exit code of restarted process
	assert_eq!(exit_code, Some(0));

	// Verify process is not running after completion
	assert!(!process_manager.is_running(&base_data_dir).await?);

	Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_process_manager_stop_graceful() -> Result<()> {
	let (process_manager, temp_dir) = setup_test_environment().await?;

	// Create a TaskCtx
	let (task, _log_rx) = create_task_ctx();

	// Start a long-running process with custom exit code on SIGTERM
	#[cfg(windows)]
	let (command, args) = (
		"powershell".to_string(),
		vec![
			"-NoProfile".to_string(),
			"-Command".to_string(),
			r#"
				$script:exitCode = 42
				$handler = {
					Write-Host "Exiting with code $script:exitCode"
					exit $script:exitCode
				}
				$null = Register-EngineEvent -SourceIdentifier PowerShell.Exiting -Action $handler
				Write-Host 'Starting long process'
				while ($true) { Start-Sleep -Seconds 1 }
			"#
			.to_string(),
		],
	);

	#[cfg(not(windows))]
	let (command, args) = (
		"sh".to_string(),
		vec![
			"-c".to_string(),
			r#"
				trap 'echo "Exiting with code 42"; exit 42' TERM
				echo 'Starting long process'
				tail -f /dev/null & wait
			"#
			.to_string(),
		],
	);
	let envs = Vec::new();
	let current_dir = std::env::current_dir()?.to_string_lossy().to_string();
	let base_data_dir = temp_dir.path().to_path_buf();

	// Start the process
	let handle = tokio::spawn({
		let process_manager = process_manager.clone();
		let base_data_dir = base_data_dir.clone();
		async move {
			process_manager
				.start(
					StartOpts {
						task,
						base_data_dir,
						start_mode: StartMode::StartOrHook,
					},
					move || async move {
						Ok(CommandOpts {
							command,
							args,
							envs,
							current_dir,
						})
					},
				)
				.await
		}
	});

	// Wait a bit to ensure the process has started
	sleep(Duration::from_millis(200)).await;

	// Verify process is running
	assert!(
		process_manager.is_running(&base_data_dir).await?,
		"process not running"
	);
	assert!(!handle.is_finished(), "handle not running");

	// Stop the process
	assert!(
		process_manager.stop(&base_data_dir).await?,
		"did not stop process"
	);
	assert!(
		!process_manager.stop(&base_data_dir).await?,
		"stop should not return true if no process"
	);

	// Verify process is not running
	assert!(
		!process_manager.is_running(&base_data_dir).await?,
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
	let (process_manager, temp_dir) = setup_test_environment().await?;

	// Create a TaskCtx
	let (task, _log_rx) = create_task_ctx();

	// Start a process that ignores SIGTERM
	#[cfg(windows)]
	let (command, args) = (
		"powershell".to_string(),
		vec![
			"-NoProfile".to_string(),
			"-Command".to_string(),
			r#"
				$handler = {
					Write-Host "Caught term, ignoring"
				}
				$null = Register-EngineEvent -SourceIdentifier PowerShell.Exiting -Action $handler
				Write-Host 'Starting process that ignores SIGTERM'
				while ($true) { Start-Sleep -Seconds 1 }
			"#
			.to_string(),
		],
	);

	#[cfg(not(windows))]
	let (command, args) = (
		"sh".to_string(),
		vec![
			"-c".to_string(),
			r#"
				trap 'echo "Caught term, ignoring"' TERM
				echo 'Starting process that ignores SIGTERM'
				while true; do
					sleep 1
				done
			"#
			.to_string(),
		],
	);
	let envs = Vec::new();
	let current_dir = std::env::current_dir()?.to_string_lossy().to_string();
	let base_data_dir = temp_dir.path().to_path_buf();

	// Start the process
	let handle = tokio::spawn({
		let process_manager = process_manager.clone();
		let base_data_dir = base_data_dir.clone();
		async move {
			process_manager
				.start(
					StartOpts {
						task,
						base_data_dir,
						start_mode: StartMode::StartOrHook,
					},
					move || async move {
						Ok(CommandOpts {
							command,
							args,
							envs,
							current_dir,
						})
					},
				)
				.await
		}
	});

	// Wait a bit to ensure the process has started
	sleep(Duration::from_millis(200)).await;

	// Verify process is running
	assert!(
		process_manager.is_running(&base_data_dir).await?,
		"process not running"
	);

	// Attempt to stop the process in the background
	let stop_handle = tokio::spawn({
		let process_manager = process_manager.clone();
		let base_data_dir = base_data_dir.clone();
		async move { process_manager.stop(&base_data_dir).await }
	});

	// Verify the process is still running before the end of the grace period
	sleep(process_manager.kill_grace - Duration::from_millis(500)).await;
	assert!(
		!process_manager.is_running(&base_data_dir).await?,
		"process stopped too early"
	);

	// Verify process is killed after the grace period
	//
	// We've already slept `grace - 500ms` at this point, so we'll check
	// 500ms after the expected kill
	sleep(Duration::from_secs(1)).await;
	assert!(
		!process_manager.is_running(&base_data_dir).await?,
		"process is still running"
	);

	// Wait for the stop task to complete and check its result
	let stop_result = stop_handle.await?;
	assert!(stop_result?, "did not stop process");

	// Wait for the process to finish and get the exit code
	let exit_code = handle.await??;

	// Verify exit code (should be None due to SIGKILL)
	assert_eq!(exit_code, None, "Unexpected exit code");

	Ok(())
}
