//! Tests for retrieval::daemon::health

use std::path::Path;

use ch_cli::retrieval::daemon::health::{
	health_check, HealthStatus,
};

/// Test HealthStatus default values
#[test]
fn test_health_status_default() {
	let status = HealthStatus::default();

	assert!(!status.healthy);
	assert_eq!(status.latency_ms, 0);
	assert!(status.error.is_some());
	assert_eq!(
		status.error.unwrap(),
		"Daemon not running"
	);
	assert!(status.pid.is_none());
}

/// Test health check when daemon is not running
#[test]
fn test_health_check_no_daemon() {
	let socket_path =
		Path::new("/tmp/ch-cli-test-nonexistent.sock");

	let health = health_check(socket_path);

	assert!(!health.healthy);
	assert_eq!(health.latency_ms, 0);
	assert!(health.error.is_some());
	assert!(
		health.error.unwrap().contains("not running")
	);
	assert!(health.pid.is_none());
}
