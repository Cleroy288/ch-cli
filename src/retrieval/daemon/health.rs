//! Daemon Health Checks and Recovery
//!
//! Provides health monitoring, auto-recovery, and
//! readiness checks for the model daemon.

use std::path::Path;

use super::lifecycle::{
	daemon_status, start_daemon, stop_daemon,
};
use crate::retrieval::RetrievalConfig;

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

/// Ensure daemon is running and ready for queries
///
/// Starts daemon in background if not running, waits
/// up to 30 seconds for models to load via ping.
pub fn ensure_daemon_ready() -> bool {
	let config = RetrievalConfig::default();
	let socket_path = &config.socket_path;

	// Check if already running
	let status = daemon_status(socket_path);
	if status.running {
		return true;
	}

	// Not running, start it
	eprintln!(
		"[daemon] Starting daemon in background..."
	);
	if let Err(e) = start_daemon(socket_path) {
		eprintln!("[daemon] Failed to start: {}", e);
		return false;
	}

	// Wait for daemon to be ready (ping loop)
	eprintln!("[daemon] Waiting for models to load...");
	let client = super::DaemonClient::new();

	// Wait up to 30 seconds (60 * 500ms)
	for i in 0..60 {
		std::thread::sleep(
			std::time::Duration::from_millis(500),
		);
		match client.ping() {
			Ok(true) => {
				eprintln!(
					"[daemon] Ready after {}s",
					(i + 1) / 2
				);
				return true;
			}
			_ => {
				if i % 4 == 0 && i > 0 {
					eprintln!(
						"[daemon] Still loading... ({}s)",
						(i + 1) / 2
					);
				}
			}
		}
	}

	eprintln!(
		"[daemon] Timeout waiting for daemon to be ready"
	);
	false
}

/// Start daemon in background without waiting
///
/// Used for pre-warming on CLI startup.
pub fn prewarm_daemon() {
	let config = RetrievalConfig::default();
	let socket_path = &config.socket_path;

	// Check if already running
	let status = daemon_status(socket_path);
	if status.running {
		return; // Already running
	}

	// Start in background silently
	let _ = start_daemon(socket_path);
}

/// Check daemon health with latency measurement
///
/// Performs a ping request and measures response time.
pub fn health_check(socket_path: &Path) -> HealthStatus {
	super::health_check::health_check(socket_path)
}

/// Auto-recover daemon if unhealthy
///
/// Stops existing daemon, starts fresh, waits for ready.
pub fn ensure_healthy_daemon(
	socket_path: &Path,
) -> bool {
	let health = health_check(socket_path);
	if health.healthy {
		return true;
	}

	eprintln!("[daemon] Unhealthy: {:?}", health.error);
	eprintln!("[daemon] Attempting recovery...");

	// Stop existing daemon (if any)
	if let Err(e) = stop_daemon(socket_path) {
		eprintln!(
			"[daemon] Stop failed (may not be running): {}",
			e
		);
	}

	// Wait for cleanup
	std::thread::sleep(
		std::time::Duration::from_millis(500),
	);

	// Start fresh daemon
	if let Err(e) = start_daemon(socket_path) {
		eprintln!("[daemon] Start failed: {}", e);
		return false;
	}

	let timeout = std::time::Duration::from_secs(60);
	ensure_daemon_ready_with_timeout(socket_path, timeout)
}

/// Wait for daemon to be ready with a timeout
///
/// Polls health_check at 500ms intervals until healthy
/// or timeout expires.
pub fn ensure_daemon_ready_with_timeout(
	socket_path: &Path,
	timeout: std::time::Duration,
) -> bool {
	use std::time::Instant;

	let start = Instant::now();
	let interval =
		std::time::Duration::from_millis(500);

	while start.elapsed() < timeout {
		let health = health_check(socket_path);
		if health.healthy {
			eprintln!(
				"[daemon] Recovery successful ({}ms)",
				health.latency_ms
			);
			return true;
		}
		std::thread::sleep(interval);
	}

	eprintln!(
		"[daemon] Recovery timed out after {:?}",
		timeout
	);
	false
}

