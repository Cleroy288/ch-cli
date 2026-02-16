//! Project Daemon Requests
//!
//! Provides project indexing, searching, and cache management operations.

use std::path::Path;
use std::time::Duration;

use super::connection::send_request;
use crate::retrieval::daemon::protocol::{
	CachedSearchResult, DaemonRequest, DaemonResponse,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Query parameters for project search
#[allow(clippy::min_ident_chars)]
pub struct ProjectSearchQuery<'a> {
	/// project path to search in
	pub project_path: &'a str,
	/// search query text
	pub query: &'a str,
	/// max results to return
	pub limit: usize,
}

/// Index result: (symbol_count, cached, index_time_ms)
type IndexProjectResult = RetrievalResult<(usize, bool, u64)>;

/// Status result: (cached, symbol_count, last_indexed)
type ProjectStatusResult =
	RetrievalResult<(bool, usize, u64)>;

/// Index a project and cache it in daemon memory
/// Returns (symbol_count, cached, index_time_ms)
pub fn index_project(
	socket_path: &Path,
	timeout: Duration,
	project_path: &str,
	force: bool,
) -> IndexProjectResult {
	let request = DaemonRequest::IndexProject {
		project_path: project_path.to_string(),
		force,
	};
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::ProjectIndexed {
			symbol_count, cached, index_time_ms
		} => Ok((symbol_count, cached, index_time_ms)),
		DaemonResponse::Error(msg) => Err(RetrievalError::Embedding(msg)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}

/// Search a cached project
#[allow(clippy::min_ident_chars)]
pub fn search_project(
	socket_path: &Path,
	timeout: Duration,
	search: ProjectSearchQuery<'_>,
) -> RetrievalResult<Vec<CachedSearchResult>> {
	let request = DaemonRequest::SearchProject {
		project_path: search.project_path.to_string(),
		query: search.query.to_string(),
		limit: search.limit,
	};
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::SearchResults(results) => Ok(results),
		DaemonResponse::Error(msg) => Err(RetrievalError::Embedding(msg)),
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
) -> ProjectStatusResult {
	let request = DaemonRequest::ProjectStatus {
		project_path: project_path.to_string(),
	};
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::ProjectCacheStatus { cached, symbol_count, last_indexed } => {
			Ok((cached, symbol_count, last_indexed))
		}
		DaemonResponse::Error(msg) => Err(RetrievalError::DaemonCommunication(msg)),
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
		DaemonResponse::Error(msg) => Err(RetrievalError::DaemonCommunication(msg)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}
