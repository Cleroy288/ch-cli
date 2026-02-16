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
#[allow(clippy::print_stderr)]
pub fn ensure_daemon_ready() -> bool {
	let config = RetrievalConfig::default();
	let socket_path = &config.socket_path;

	let status = daemon_status(socket_path);
	if status.running {
		return true;
	}

	eprintln!(
		"[daemon] Starting daemon in background..."
	);
	if let Err(err) = start_daemon(socket_path) {
		eprintln!("[daemon] Failed to start: {}", err);
		return false;
	}

	eprintln!("[daemon] Waiting for models to load...");
	wait_for_daemon_ping()
}

/// Poll daemon ping until ready or timeout (30s)
#[allow(clippy::print_stderr)]
fn wait_for_daemon_ping() -> bool {
	let client = super::DaemonClient::new();
	let poll_interval =
		std::time::Duration::from_millis(500);
	// Wait up to 30 seconds (60 * 500ms)
	for idx in 0..60 {
		std::thread::sleep(poll_interval);
		if let Ok(true) = client.ping() {
			eprintln!(
				"[daemon] Ready after {}s",
				(idx + 1) / 2
			);
			return true;
		}
		if idx % 4 == 0 && idx > 0 {
			eprintln!(
				"[daemon] Still loading... ({}s)",
				(idx + 1) / 2
			);
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
#[allow(clippy::print_stderr)]
pub fn ensure_healthy_daemon(
	socket_path: &Path,
) -> bool {
	let health = health_check(socket_path);
	if health.healthy {
		return true;
	}

	eprintln!("[daemon] Unhealthy: {:?}", health.error);
	eprintln!("[daemon] Attempting recovery...");

	if let Err(err) = stop_daemon(socket_path) {
		eprintln!(
			"[daemon] Stop failed (may not be running): {}",
			err
		);
	}

	std::thread::sleep(
		std::time::Duration::from_millis(500),
	);

	if let Err(err) = start_daemon(socket_path) {
		eprintln!("[daemon] Start failed: {}", err);
		return false;
	}

	let dur = std::time::Duration::from_secs(60);
	ensure_daemon_ready_with_timeout(socket_path, dur)
}

/// Wait for daemon to be ready with a timeout
///
/// Polls health_check at 500ms intervals until healthy
/// or timeout expires.
#[allow(clippy::print_stderr)]
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

