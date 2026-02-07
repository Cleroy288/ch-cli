//! Documentation Daemon Requests
//!
//! Provides documentation generation and retrieval operations.

use std::path::Path;
use std::time::Duration;

use super::connection::send_request;
use crate::retrieval::daemon::protocol::{
	DaemonRequest, DaemonResponse, DocEntryResponse,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

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

/// Start documentation generation for a project (returns immediately)
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
	match response {
		DaemonResponse::DocGenStatus {
			total,
			completed,
			pending,
			is_ready,
			in_progress,
		} => Ok(DocGenStatus {
			total,
			completed,
			pending,
			is_ready,
			in_progress,
		}),
		DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
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
	match response {
		DaemonResponse::DocGenStatus {
			total,
			completed,
			pending,
			is_ready,
			in_progress,
		} => Ok(DocGenStatus {
			total,
			completed,
			pending,
			is_ready,
			in_progress,
		}),
		DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}

/// Get documentation for a specific symbol
pub fn get_doc(
	socket_path: &Path,
	timeout: Duration,
	project_path: String,
	symbol_name: String,
) -> RetrievalResult<Option<DocEntryResponse>> {
	let request = DaemonRequest::GetDoc { project_path, symbol_name };
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::Doc(doc) => Ok(doc),
		DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}

/// Search documentation
pub fn search_docs(
	socket_path: &Path,
	timeout: Duration,
	project_path: String,
	query: String,
	limit: usize,
) -> RetrievalResult<Vec<DocEntryResponse>> {
	let request = DaemonRequest::SearchDocs { project_path, query, limit };
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::DocResults(results) => Ok(results),
		DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}
