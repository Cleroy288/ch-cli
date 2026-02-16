//! Documentation Generation Daemon Requests
//!
//! Provides doc gen start and status operations.

use std::path::Path;
use std::time::Duration;

use super::connection::send_request;
use crate::retrieval::daemon::protocol::{
	DaemonRequest, DaemonResponse,
};
use crate::retrieval::{
	RetrievalError, RetrievalResult,
};

/// Documentation generation status
pub struct DocGenStatus {
	/// total number of entries
	pub total: usize,
	/// completed entries
	pub completed: usize,
	/// pending entries
	pub pending: usize,
	/// whether generation is complete
	pub is_ready: bool,
	/// whether generation is running in background
	pub in_progress: bool,
}

/// Parse a DaemonResponse into DocGenStatus
fn parse_doc_gen_response(
	response: DaemonResponse,
) -> RetrievalResult<DocGenStatus> {
	match response {
		DaemonResponse::DocGenStatus {
			total, completed, pending,
			is_ready, in_progress,
		} => Ok(DocGenStatus {
			total, completed, pending,
			is_ready, in_progress,
		}),
		DaemonResponse::Error(msg) => Err(
			RetrievalError::DaemonCommunication(msg),
		),
		other => Err(
			RetrievalError::DaemonCommunication(
				format!("unexpected: {:?}", other),
			),
		),
	}
}

/// Start doc generation (returns immediately)
pub fn start_doc_gen(
	socket_path: &Path,
	timeout: Duration,
	project_path: String,
	force: bool,
) -> RetrievalResult<DocGenStatus> {
	let request = DaemonRequest::StartDocGen {
		project_path,
		force,
	};
	let response =
		send_request(socket_path, timeout, &request)?;
	parse_doc_gen_response(response)
}

/// Get documentation generation status
pub fn doc_gen_status(
	socket_path: &Path,
	timeout: Duration,
	project_path: String,
) -> RetrievalResult<DocGenStatus> {
	let request =
		DaemonRequest::DocGenStatus { project_path };
	let response =
		send_request(socket_path, timeout, &request)?;
	parse_doc_gen_response(response)
}
