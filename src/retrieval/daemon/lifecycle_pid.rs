//! PID Management Functions
//!
//! Extracted from lifecycle_helpers.rs for norm compliance.

use std::fs;
use std::path::Path;

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

/// Get PID file path from socket path
pub fn pid_file_from_socket(
	socket_path: &Path,
) -> std::path::PathBuf {
	socket_path.with_extension("pid")
}

/// Read PID from PID file
pub fn read_pid(pid_file: &Path) -> Option<u32> {
	fs::read_to_string(pid_file)
		.ok()
		.and_then(|s| s.trim().parse().ok())
}

/// Check if process is running by PID
pub fn process_running(pid: u32) -> bool {
	// use kill(pid, 0) to check if process exists
	kill(Pid::from_raw(pid as i32), None).is_ok()
}

/// Send signal to daemon process
pub fn signal_daemon(
	pid: u32,
	signal: Signal,
) -> Result<(), nix::errno::Errno> {
	let pid_t = Pid::from_raw(pid as i32);
	kill(pid_t, signal)
}
