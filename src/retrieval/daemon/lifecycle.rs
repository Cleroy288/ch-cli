//! Daemon Lifecycle Management
//!
//! Functions to start, stop, and check daemon status.


use std::path::Path;
use std::process::{Command, Stdio};

use nix::sys::signal::Signal;

use super::protocol::DaemonStatus;
use crate::retrieval::{
	RetrievalConfig, RetrievalError, RetrievalResult,
};

#[doc(hidden)]
pub use super::lifecycle_helpers::process_running;
pub use super::lifecycle_helpers::log_file_from_socket;

/// Build running status for a given PID
fn running_status(pid: u32) -> DaemonStatus {
	DaemonStatus {
		running: true,
		pid: Some(pid),
		loaded_models: Vec::new(),
		device:
			super::protocol::DeviceStatus::default(),
		uptime_secs: 0,
	}
}

/// Get the status of the daemon
pub fn daemon_status(
	socket_path: &Path,
) -> DaemonStatus {
	let pid_file =
		super::lifecycle_helpers::pid_file_from_socket(
			socket_path,
		);
	let pid = match super::lifecycle_helpers::read_pid(
		&pid_file,
	) {
		Some(val) => val,
		None => {
			return super::lifecycle_helpers::
				default_stopped_status();
		}
	};

	if !process_running(pid) {
		super::lifecycle_helpers::cleanup_stale_daemon(
			socket_path,
			&pid_file,
		);
		return super::lifecycle_helpers::
			default_stopped_status();
	}

	running_status(pid)
}

/// Prepare filesystem for daemon startup
fn prepare_daemon_paths(
	socket_path: &Path,
) -> RetrievalResult<()> {
	if let Some(parent) = socket_path.parent() {
		std::fs::create_dir_all(parent)?;
	}
	let _ = std::fs::remove_file(socket_path);
	Ok(())
}

/// Spawn daemon as child process with log redirection
fn spawn_daemon_process(
	socket_path: &Path,
) -> RetrievalResult<()> {
	let exe = std::env::current_exe()?;
	let log_path = log_file_from_socket(socket_path);
	let log_file = std::fs::File::create(&log_path)?;
	let log_err = log_file.try_clone()?;
	let _child = Command::new(&exe)
		.arg("daemon")
		.arg("run")
		.arg("--socket")
		.arg(socket_path)
		.stdout(Stdio::from(log_file))
		.stderr(Stdio::from(log_err))
		.spawn()
		.map_err(|err| {
			RetrievalError::DaemonNotRunning(format!(
				"failed to spawn: {}",
				err
			))
		})?;
	Ok(())
}

/// Start the daemon if not already running
pub fn start_daemon(
	socket_path: &Path,
) -> RetrievalResult<()> {
	let status = daemon_status(socket_path);
	if status.running {
		return Ok(());
	}
	prepare_daemon_paths(socket_path)?;
	spawn_daemon_process(socket_path)?;
	std::thread::sleep(
		std::time::Duration::from_millis(100),
	);
	Ok(())
}

/// Wait for process to exit, then force kill if needed
fn wait_and_kill(pid: u32) {
	let poll = std::time::Duration::from_millis(100);
	for _ in 0..50 {
		if !process_running(pid) {
			return;
		}
		std::thread::sleep(poll);
	}
	if process_running(pid) {
		let _ = super::lifecycle_helpers::signal_daemon(
			pid,
			Signal::SIGKILL,
		);
	}
}

/// Stop the daemon
pub fn stop_daemon(
	socket_path: &Path,
) -> RetrievalResult<()> {
	let pid_file =
		super::lifecycle_helpers::pid_file_from_socket(
			socket_path,
		);
	let pid = match super::lifecycle_helpers::read_pid(
		&pid_file,
	) {
		Some(val) => val,
		None => return Ok(()),
	};
	let _ = super::lifecycle_helpers::signal_daemon(
		pid,
		Signal::SIGTERM,
	);
	wait_and_kill(pid);
	super::lifecycle_helpers::cleanup_stale_daemon(
		socket_path,
		&pid_file,
	);
	Ok(())
}

/// Send SIGTERM to daemon without waiting.
///
/// Fire-and-forget: the daemon will shut itself down.
/// Used at TUI exit for instant quit.
pub fn signal_daemon_stop(
	socket_path: &Path,
) -> RetrievalResult<()> {
	let pid_file =
		super::lifecycle_helpers::pid_file_from_socket(
			socket_path,
		);
	let Some(pid) =
		super::lifecycle_helpers::read_pid(&pid_file)
	else {
		return Ok(());
	};
	let _ = super::lifecycle_helpers::signal_daemon(
		pid,
		Signal::SIGTERM,
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

