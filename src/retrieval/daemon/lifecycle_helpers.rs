//! Lifecycle Helper Functions
//!
//! Internal helpers for daemon lifecycle management.

use std::fs;
use std::path::Path;

use nix::sys::signal::Signal;

use super::protocol::DaemonStatus;

/// Get PID file path from socket path
pub fn pid_file_from_socket(
	socket_path: &Path,
) -> std::path::PathBuf {
	super::lifecycle_pid::pid_file_from_socket(socket_path)
}

/// Read PID from PID file
pub fn read_pid(pid_file: &Path) -> Option<u32> {
	super::lifecycle_pid::read_pid(pid_file)
}

/// Check if process is running by PID
pub fn process_running(pid: u32) -> bool {
	super::lifecycle_pid::process_running(pid)
}

/// Clean up stale daemon files
pub fn cleanup_stale_daemon(
	socket_path: &Path,
	pid_file: &Path,
) {
	let _ = fs::remove_file(&pid_file);
	let _ = fs::remove_file(socket_path);
}

/// Send signal to daemon process
pub fn signal_daemon(
	pid: u32,
	signal: Signal,
) -> Result<(), nix::errno::Errno> {
	super::lifecycle_pid::signal_daemon(pid, signal)
}

/// Create default stopped daemon status
pub fn default_stopped_status() -> DaemonStatus {
	DaemonStatus::default()
}
