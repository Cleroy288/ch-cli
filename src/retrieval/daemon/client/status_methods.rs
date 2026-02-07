//! Status and Control Methods
//!
//! Provides daemon status, ping, and shutdown operations.

use std::path::Path;
use std::time::Duration;

use crate::retrieval::daemon::protocol::DaemonStatus;
use crate::retrieval::RetrievalResult;

use super::core_requests;

/// Get daemon status
pub fn get_status(
	socket_path: &Path,
	timeout: Duration,
) -> RetrievalResult<DaemonStatus> {
	core_requests::status(socket_path, timeout)
}

/// Ping daemon to check if alive
pub fn ping_daemon(
	socket_path: &Path,
	timeout: Duration,
) -> RetrievalResult<bool> {
	core_requests::ping(socket_path, timeout)
}

/// Shutdown the daemon
pub fn shutdown_daemon(
	socket_path: &Path,
	timeout: Duration,
) -> RetrievalResult<()> {
	core_requests::shutdown(socket_path, timeout)
}
