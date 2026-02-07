//! Health Check Functions
//!
//! Extracted from health.rs for norm compliance.

use std::path::Path;
use std::time::Instant;

use super::client::DaemonClient;
use super::health::HealthStatus;
use super::lifecycle::daemon_status;

/// Check daemon health with latency measurement
///
/// Performs a ping request and measures response time.
pub fn health_check(socket_path: &Path) -> HealthStatus {
	// First check if daemon process is running
	let status = daemon_status(socket_path);
	if !status.running {
		return HealthStatus {
			healthy: false,
			latency_ms: 0,
			error: Some(
				"Daemon process not running".to_string(),
			),
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
