//! Daemon Lifecycle Management
//!
//! Provides functions to start, stop, and check the status of the model daemon.

use std::fs;
use std::path::Path;
use std::process::Command;

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

use super::protocol::DaemonStatus;
use crate::retrieval::{RetrievalConfig, RetrievalError, RetrievalResult};

/// Health check result with status and latency
#[derive(Debug, Clone)]
pub struct HealthStatus {
	/// Whether the daemon is healthy and responding
	pub healthy: bool,
	/// Response latency in milliseconds
	pub latency_ms: u64,
	/// Error message if unhealthy
	pub error: Option<String>,
	/// PID of the daemon process
	pub pid: Option<u32>,
}

impl Default for HealthStatus {
	fn default() -> Self {
		Self {
			healthy: false,
			latency_ms: 0,
			error: Some("Daemon not running".to_string()),
			pid: None,
		}
	}
}

/// Get the PID file path from socket path
fn pid_file_from_socket(socket_path: &Path) -> std::path::PathBuf {
	socket_path.with_extension("pid")
}

/// Read PID from PID file
fn read_pid(pid_file: &Path) -> Option<u32> {
	fs::read_to_string(pid_file)
		.ok()
		.and_then(|s| s.trim().parse().ok())
}

/// Check if a process is running by PID
fn process_running(pid: u32) -> bool {
	// use kill(pid, 0) to check if process exists
	kill(Pid::from_raw(pid as i32), None).is_ok()
}

/// Get the status of the daemon
pub fn daemon_status(socket_path: &Path) -> DaemonStatus {
	let pid_file = pid_file_from_socket(socket_path);

	// check if PID file exists
	let pid = match read_pid(&pid_file) {
		Some(p) => p,
		None => {
			return DaemonStatus::default();
		}
	};

	// check if process is running
	if !process_running(pid) {
		// stale PID file, clean up
		let _ = fs::remove_file(&pid_file);
		let _ = fs::remove_file(socket_path);
		return DaemonStatus::default();
	}

	// daemon is running
	DaemonStatus {
		running: true,
		pid: Some(pid),
		loaded_models: Vec::new(), // will be filled by Status request
		device: super::protocol::DeviceStatus::default(),
		uptime_secs: 0,
	}
}

/// Start the daemon if not already running
pub fn start_daemon(socket_path: &Path) -> RetrievalResult<()> {
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
		.map_err(|e| RetrievalError::DaemonNotRunning(format!("failed to spawn: {}", e)))?;

	// wait a bit for daemon to start
	std::thread::sleep(std::time::Duration::from_millis(100));

	Ok(())
}

/// Stop the daemon
pub fn stop_daemon(socket_path: &Path) -> RetrievalResult<()> {
	let pid_file = pid_file_from_socket(socket_path);

	// read PID
	let pid = match read_pid(&pid_file) {
		Some(p) => p,
		None => return Ok(()), // not running
	};

	let pid_t = Pid::from_raw(pid as i32);

	// send SIGTERM
	let _ = kill(pid_t, Signal::SIGTERM);

	// wait for process to exit (up to 5 seconds)
	for _ in 0..50 {
		if !process_running(pid) {
			break;
		}
		std::thread::sleep(std::time::Duration::from_millis(100));
	}

	// force kill if still running
	if process_running(pid) {
		let _ = kill(pid_t, Signal::SIGKILL);
	}

	// clean up files
	let _ = fs::remove_file(&pid_file);
	let _ = fs::remove_file(socket_path);

	Ok(())
}

/// Restart the daemon
pub fn restart_daemon(socket_path: &Path) -> RetrievalResult<()> {
	stop_daemon(socket_path)?;
	std::thread::sleep(std::time::Duration::from_millis(100));
	start_daemon(socket_path)
}

/// Get the default daemon paths from config
pub fn default_paths() -> (std::path::PathBuf, std::path::PathBuf) {
	let config = RetrievalConfig::default();
	(config.socket_path, config.pid_file)
}

/// Ensure daemon is running and ready for queries
/// Starts daemon in background if not running, waits for models to load
/// Returns true if daemon is ready, false if failed to start
pub fn ensure_daemon_ready() -> bool {
	let config = RetrievalConfig::default();
	let socket_path = &config.socket_path;

	// Check if already running
	let status = daemon_status(socket_path);
	if status.running {
		return true;
	}

	// Not running, start it
	eprintln!("[daemon] Starting daemon in background...");
	if let Err(e) = start_daemon(socket_path) {
		eprintln!("[daemon] Failed to start: {}", e);
		return false;
	}

	// Wait for daemon to be ready (check with ping)
	eprintln!("[daemon] Waiting for models to load...");
	let client = super::DaemonClient::new();

	// Wait up to 30 seconds for daemon to be ready
	for i in 0..60 {
		std::thread::sleep(std::time::Duration::from_millis(500));

		match client.ping() {
			Ok(true) => {
				eprintln!("[daemon] Ready after {}s", (i + 1) / 2);
				return true;
			}
			_ => {
				// Still loading, continue waiting
				if i % 4 == 0 && i > 0 {
					eprintln!("[daemon] Still loading... ({}s)", (i + 1) / 2);
				}
			}
		}
	}

	eprintln!("[daemon] Timeout waiting for daemon to be ready");
	false
}

/// Start daemon in background without waiting (fire and forget)
/// Used for pre-warming on CLI startup
pub fn prewarm_daemon() {
	let config = RetrievalConfig::default();
	let socket_path = &config.socket_path;

	// Check if already running
	let status = daemon_status(socket_path);
	if status.running {
		return; // Already running, nothing to do
	}

	// Start in background silently
	let _ = start_daemon(socket_path);
}

/// Check daemon health with latency measurement
/// Performs a ping request and measures response time
pub fn health_check(socket_path: &Path) -> HealthStatus {
	use super::client::DaemonClient;
	use std::time::Instant;

	// First check if daemon process is running
	let status = daemon_status(socket_path);
	if !status.running {
		return HealthStatus {
			healthy: false,
			latency_ms: 0,
			error: Some("Daemon process not running".to_string()),
			pid: None,
		};
	}

	// Try to ping the daemon
	let start = Instant::now();
	let client = DaemonClient::new();

	match client.ping() {
		Ok(true) => HealthStatus {
			healthy: true,
			latency_ms: start.elapsed().as_millis() as u64,
			error: None,
			pid: status.pid,
		},
		Ok(false) => HealthStatus {
			healthy: false,
			latency_ms: start.elapsed().as_millis() as u64,
			error: Some("Ping returned false".to_string()),
			pid: status.pid,
		},
		Err(e) => HealthStatus {
			healthy: false,
			latency_ms: start.elapsed().as_millis() as u64,
			error: Some(e.to_string()),
			pid: status.pid,
		},
	}
}

/// Auto-recover daemon if unhealthy
/// Attempts to restart the daemon and waits for it to be ready
pub fn ensure_healthy_daemon(socket_path: &Path) -> bool {
	let health = health_check(socket_path);

	if health.healthy {
		return true;
	}

	eprintln!("[daemon] Unhealthy: {:?}", health.error);
	eprintln!("[daemon] Attempting recovery...");

	// Stop existing daemon (if any)
	if let Err(e) = stop_daemon(socket_path) {
		eprintln!("[daemon] Stop failed (may not be running): {}", e);
	}

	// Wait for cleanup
	std::thread::sleep(std::time::Duration::from_millis(500));

	// Start fresh daemon
	if let Err(e) = start_daemon(socket_path) {
		eprintln!("[daemon] Start failed: {}", e);
		return false;
	}

	// Wait for daemon to be ready
	ensure_daemon_ready_with_timeout(socket_path, std::time::Duration::from_secs(60))
}

/// Wait for daemon to be ready with a timeout
pub fn ensure_daemon_ready_with_timeout(socket_path: &Path, timeout: std::time::Duration) -> bool {
	use std::time::Instant;

	let start = Instant::now();
	let check_interval = std::time::Duration::from_millis(500);

	while start.elapsed() < timeout {
		let health = health_check(socket_path);
		if health.healthy {
			eprintln!("[daemon] Recovery successful (latency: {}ms)", health.latency_ms);
			return true;
		}

		std::thread::sleep(check_interval);
	}

	eprintln!("[daemon] Recovery timed out after {:?}", timeout);
	false
}

/// Check health and recover if needed before critical operations
/// Returns true if daemon is healthy (or was recovered)
pub fn check_and_recover_if_needed(socket_path: &Path) -> bool {
	let health = health_check(socket_path);

	if health.healthy {
		return true;
	}

	// Only attempt recovery for certain error types
	if let Some(ref error) = health.error {
		if error.contains("temporarily unavailable")
			|| error.contains("Resource temporarily unavailable")
			|| error.contains("connection reset")
			|| error.contains("broken pipe")
		{
			return ensure_healthy_daemon(socket_path);
		}
	}

	// For other errors (like daemon not running), just return false
	// Let the caller decide whether to start the daemon
	false
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Test that HealthStatus default values are correct
	#[test]
	fn test_health_status_default() {
		let status = HealthStatus::default();

		assert!(!status.healthy);
		assert_eq!(status.latency_ms, 0);
		assert!(status.error.is_some());
		assert_eq!(status.error.unwrap(), "Daemon not running");
		assert!(status.pid.is_none());
	}

	/// Test health check when daemon is not running
	#[test]
	fn test_health_check_no_daemon() {
		// use a non-existent socket path
		let socket_path = Path::new("/tmp/ch-cli-test-nonexistent.sock");

		let health = health_check(socket_path);

		assert!(!health.healthy);
		assert_eq!(health.latency_ms, 0);
		assert!(health.error.is_some());
		assert!(health.error.unwrap().contains("not running"));
		assert!(health.pid.is_none());
	}
}
