//! Tests for retrieval::daemon::lifecycle

use std::path::Path;

use ch_cli::retrieval::daemon::lifecycle::{
	default_paths, process_running,
};
use ch_cli::retrieval::daemon::lifecycle_helpers::{
	pid_file_from_socket,
};

/// Test pid_file_from_socket returns .pid extension
#[test]
fn test_pid_file_from_socket() {
	let socket = Path::new("/tmp/ch-cli.sock");
	let pid = pid_file_from_socket(socket);
	assert_eq!(pid, Path::new("/tmp/ch-cli.pid"));
}

/// Test pid_file_from_socket with nested path
#[test]
fn test_pid_file_from_socket_nested() {
	let socket =
		Path::new("/var/run/ch-cli/daemon.sock");
	let pid = pid_file_from_socket(socket);
	assert_eq!(
		pid,
		Path::new("/var/run/ch-cli/daemon.pid")
	);
}

/// Test default_paths returns expected paths
#[test]
fn test_default_paths() {
	let (socket, pid) = default_paths();

	// Should use home directory with .ch-cli prefix
	assert!(
		socket.to_string_lossy().contains(".ch-cli")
	);
	assert!(pid.to_string_lossy().contains(".ch-cli"));

	// Socket should end with .sock
	assert!(socket.extension().is_some());
	assert_eq!(
		socket.extension().unwrap(),
		"sock"
	);

	// PID should end with .pid
	assert!(pid.extension().is_some());
	assert_eq!(pid.extension().unwrap(), "pid");
}

/// Test process_running with current process
#[test]
fn test_process_running_self() {
	let pid = std::process::id();
	assert!(process_running(pid));
}

/// Test process_running with invalid PID
#[test]
fn test_process_running_invalid() {
	// PID 99999999 is unlikely to exist
	assert!(!process_running(99999999));
}
