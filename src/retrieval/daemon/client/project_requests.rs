//! Project Daemon Requests
//!
//! Provides project indexing, searching, and cache management operations.

use std::path::Path;
use std::time::Duration;

use super::connection::send_request;
use crate::retrieval::daemon::protocol::{CachedSearchResult, DaemonRequest, DaemonResponse};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Index a project and cache it in daemon memory
/// Returns (symbol_count, cached, index_time_ms)
pub fn index_project(
	socket_path: &Path,
	timeout: Duration,
	project_path: &str,
	force: bool,
) -> RetrievalResult<(usize, bool, u64)> {
	let request = DaemonRequest::IndexProject {
		project_path: project_path.to_string(),
		force,
	};
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::ProjectIndexed {
			symbol_count, cached, index_time_ms
		} => Ok((symbol_count, cached, index_time_ms)),
		DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}

/// Search a cached project
pub fn search_project(
	socket_path: &Path,
	timeout: Duration,
	project_path: &str,
	query: &str,
	limit: usize,
) -> RetrievalResult<Vec<CachedSearchResult>> {
	let request = DaemonRequest::SearchProject {
		project_path: project_path.to_string(),
		query: query.to_string(),
		limit,
	};
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::SearchResults(results) => Ok(results),
		DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}

/// Get project cache status
/// Returns (cached, symbol_count, last_indexed)
pub fn project_status(
	socket_path: &Path,
	timeout: Duration,
	project_path: &str,
) -> RetrievalResult<(bool, usize, u64)> {
	let request = DaemonRequest::ProjectStatus {
		project_path: project_path.to_string(),
	};
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::ProjectCacheStatus { cached, symbol_count, last_indexed } => {
			Ok((cached, symbol_count, last_indexed))
		}
		DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}

/// Evict a project from daemon cache
pub fn evict_project(
	socket_path: &Path,
	timeout: Duration,
	project_path: &str,
) -> RetrievalResult<()> {
	let request = DaemonRequest::EvictProject {
		project_path: project_path.to_string(),
	};
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::Ok => Ok(()),
		DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}
