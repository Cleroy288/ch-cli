//! Documentation Daemon Requests
//!
//! Provides doc lookup and search operations.

use std::path::Path;
use std::time::Duration;

use super::connection::send_request;
use crate::retrieval::daemon::protocol::{
	DaemonRequest, DaemonResponse, DocEntryResponse,
};
use crate::retrieval::{
	RetrievalError, RetrievalResult,
};

// Re-export doc gen types from split module
pub use super::doc_gen_requests::{
	doc_gen_status, start_doc_gen, DocGenStatus,
};

/// Query parameters for documentation search
pub struct DocSearchQuery {
	/// project path to search in
	pub project_path: String,
	/// search query text
	pub query: String,
	/// max results to return
	pub limit: usize,
}

/// Get documentation for a specific symbol
pub fn get_doc(
	socket_path: &Path,
	timeout: Duration,
	project_path: String,
	symbol_name: String,
) -> RetrievalResult<Option<DocEntryResponse>> {
	let request = DaemonRequest::GetDoc {
		project_path,
		symbol_name,
	};
	match send_request(
		socket_path, timeout, &request,
	)? {
		DaemonResponse::Doc(doc) => Ok(doc),
		DaemonResponse::Error(msg) => {
			Err(RetrievalError::DaemonCommunication(
				msg,
			))
		}
		other => {
			Err(RetrievalError::DaemonCommunication(
				format!(
					"unexpected response: {:?}",
					other
				),
			))
		}
	}
}

/// Parameters for file-based doc lookup
pub struct DocByFileQuery {
	/// project root path
	pub project_path: String,
	/// file path relative to project root
	pub file_path: String,
	/// symbol name (leaf only)
	pub symbol_name: String,
}

/// Get doc for a symbol in a specific file
pub fn get_doc_by_file(
	socket_path: &Path,
	timeout: Duration,
	query: DocByFileQuery,
) -> RetrievalResult<Option<DocEntryResponse>> {
	let request = DaemonRequest::GetDocByFile {
		project_path: query.project_path,
		file_path: query.file_path,
		symbol_name: query.symbol_name,
	};
	match send_request(
		socket_path, timeout, &request,
	)? {
		DaemonResponse::Doc(doc) => Ok(doc),
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

/// Search documentation
pub fn search_docs(
	socket_path: &Path,
	timeout: Duration,
	search: DocSearchQuery,
) -> RetrievalResult<Vec<DocEntryResponse>> {
	let request = DaemonRequest::SearchDocs {
		project_path: search.project_path,
		query: search.query,
		limit: search.limit,
	};
	match send_request(
		socket_path, timeout, &request,
	)? {
		DaemonResponse::DocResults(results) => {
			Ok(results)
		}
		DaemonResponse::Error(msg) => {
			Err(RetrievalError::DaemonCommunication(
				msg,
			))
		}
		other => {
			Err(RetrievalError::DaemonCommunication(
				format!(
					"unexpected response: {:?}",
					other
				),
			))
		}
	}
}
