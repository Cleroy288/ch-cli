//! Daemon Lifecycle Management
//!
//! Functions to start, stop, and check daemon status.


use std::fs;
use std::path::Path;
use std::process::Command;

use nix::sys::signal::Signal;

use super::protocol::DaemonStatus;
use crate::retrieval::{
	RetrievalConfig, RetrievalError, RetrievalResult,
};

#[doc(hidden)]
pub use super::lifecycle_helpers::process_running;

/// Get the status of the daemon
pub fn daemon_status(
	socket_path: &Path,
) -> DaemonStatus {
	let pid_file =
		crate::retrieval::daemon::lifecycle_helpers::pid_file_from_socket(
			socket_path,
		);

	// check if PID file exists
	let pid = match super::lifecycle_helpers::read_pid(&pid_file)
	{
		Some(p) => p,
		None => {
			return super::lifecycle_helpers::
				default_stopped_status();
		}
	};

	// check if process is running
	if !process_running(pid) {
		super::lifecycle_helpers::cleanup_stale_daemon(
			socket_path,
			&pid_file,
		);
		return super::lifecycle_helpers::
			default_stopped_status();
	}

	// daemon is running
	DaemonStatus {
		running: true,
		pid: Some(pid),
		loaded_models: Vec::new(),
		device:
			super::protocol::DeviceStatus::default(),
		uptime_secs: 0,
	}
}

/// Start the daemon if not already running
pub fn start_daemon(
	socket_path: &Path,
) -> RetrievalResult<()> {
	// check if already running
	let status = daemon_status(socket_path);
	if status.running {
		return Ok(());
	}

	// ensure parent directory exists
	if let Some(parent) = socket_path.parent() {
		fs::create_dir_all(parent)?;
	}

	// clean up stale socket
	let _ = fs::remove_file(socket_path);

	// get the path to the current executable
	let exe = std::env::current_exe()?;

	// spawn daemon process
	let _child = Command::new(&exe)
		.arg("daemon")
		.arg("run")
		.arg("--socket")
		.arg(socket_path)
		.spawn()
		.map_err(|e| {
			RetrievalError::DaemonNotRunning(format!(
				"failed to spawn: {}",
				e
			))
		})?;

	// wait a bit for daemon to start
	std::thread::sleep(
		std::time::Duration::from_millis(100),
	);

	Ok(())
}

/// Stop the daemon
pub fn stop_daemon(
	socket_path: &Path,
) -> RetrievalResult<()> {
	let pid_file =
		crate::retrieval::daemon::lifecycle_helpers::pid_file_from_socket(
			socket_path,
		);

	// read PID
	let pid = match super::lifecycle_helpers::read_pid(&pid_file)
	{
		Some(p) => p,
		None => return Ok(()), // not running
	};

	// send SIGTERM
	let _ = super::lifecycle_helpers::signal_daemon(
		pid,
		Signal::SIGTERM,
	);

	// wait for exit (up to 5 seconds)
	for _ in 0..50 {
		if !process_running(pid) {
			break;
		}
		std::thread::sleep(
			std::time::Duration::from_millis(100),
		);
	}

	// force kill if still running
	if process_running(pid) {
		let _ = super::lifecycle_helpers::signal_daemon(
			pid,
			Signal::SIGKILL,
		);
	}

	// clean up files
	super::lifecycle_helpers::cleanup_stale_daemon(
		socket_path,
		&pid_file,
	);

	Ok(())
}

/// Restart the daemon
pub fn restart_daemon(
	socket_path: &Path,
) -> RetrievalResult<()> {
	stop_daemon(socket_path)?;
	std::thread::sleep(
		std::time::Duration::from_millis(100),
	);
	start_daemon(socket_path)
}

/// Get the default daemon paths from config
pub fn default_paths() -> (
	std::path::PathBuf,
	std::path::PathBuf,
) {
	let config = RetrievalConfig::default();
	(config.socket_path, config.pid_file)
}

