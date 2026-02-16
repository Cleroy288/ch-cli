//! Status Request Handlers
//!
//! Extracted from core_requests for norm compliance.

use std::path::Path;
use std::time::Duration;

use super::connection::send_request;
use crate::retrieval::daemon::lifecycle::daemon_status;
use crate::retrieval::daemon::protocol::{
	DaemonRequest, DaemonResponse, DaemonStatus,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Get daemon status
pub fn status(
	socket_path: &Path,
	timeout: Duration,
) -> RetrievalResult<DaemonStatus> {
	// first check if daemon is running without connecting
	let status = daemon_status(socket_path);
	if !status.running {
		return Ok(status);
	}

	// daemon is running, get detailed status
	let request = DaemonRequest::Status;
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::Status(status) => Ok(status),
		DaemonResponse::Error(msg) => {
			Err(RetrievalError::DaemonCommunication(msg))
		}
		other => Err(RetrievalError::DaemonCommunication(
			format!("unexpected response: {:?}", other),
		)),
	}
}
