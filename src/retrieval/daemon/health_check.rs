//! Health Check Functions
//!
//! Extracted from health.rs for norm compliance.

use std::path::Path;
use std::time::Instant;

use super::client::DaemonClient;
use super::health::HealthStatus;
use super::lifecycle::daemon_status;

/// Build a HealthStatus with measured latency
fn build_status(
	healthy: bool,
	latency_ms: u64,
	error: Option<String>,
	pid: Option<u32>,
) -> HealthStatus {
	HealthStatus {
		healthy,
		latency_ms,
		error,
		pid,
	}
}

/// Check daemon health with latency measurement
///
/// Performs a ping request and measures response time.
pub fn health_check(socket_path: &Path) -> HealthStatus {
	let status = daemon_status(socket_path);
	if !status.running {
		return build_status(
			false, 0,
			Some("Daemon process not running".to_string()),
			None,
		);
	}
	ping_and_measure(status.pid)
}

/// Ping daemon and build health status with latency
fn ping_and_measure(pid: Option<u32>) -> HealthStatus {
	let start = Instant::now();
	let client = DaemonClient::new();
	let elapsed =
		|| start.elapsed().as_millis() as u64;

	match client.ping() {
		Ok(true) => {
			build_status(true, elapsed(), None, pid)
		}
		Ok(false) => build_status(
			false, elapsed(),
			Some("Ping returned false".to_string()),
			pid,
		),
		Err(err) => build_status(
			false, elapsed(),
			Some(err.to_string()),
			pid,
		),
	}
}
