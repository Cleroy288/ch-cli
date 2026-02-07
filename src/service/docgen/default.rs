//! Default implementation of DocGenService.

use std::path::Path;

use crate::domain::errors::docgen::DocGenError;
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::docgen::DocEntry;

use super::helpers::{
	doc_from_local, path_to_string, search_local,
	status_from_local,
};
use super::types::DocStatusInfo;
use super::DocGenService;

/// Default doc gen service with daemon fallback
pub struct DefaultDocGenService;

impl DefaultDocGenService {
	/// Create a new default doc gen service
	pub fn new() -> Self {
		Self
	}
}

impl DocGenService for DefaultDocGenService {
	fn start_generation(
		&self,
		path: &Path,
		force: bool,
	) -> Result<(), DocGenError> {
		let client = DaemonClient::new();
		let path_str = path_to_string(path);
		client
			.start_doc_gen(path_str, force)
			.map_err(|e| {
				DocGenError::DaemonUnavailable(
					e.to_string(),
				)
			})?;
		Ok(())
	}

	fn get_status(
		&self,
		path: &Path,
	) -> Result<DocStatusInfo, DocGenError> {
		let client = DaemonClient::new();
		let path_str = path_to_string(path);
		match client.doc_gen_status(path_str) {
			Ok(s) => Ok(DocStatusInfo {
				total: s.total,
				completed: s.completed,
				pending: s.pending,
				is_generating: s.in_progress,
				is_ready: s.is_ready,
			}),
			Err(_) => status_from_local(path),
		}
	}

	fn get_doc(
		&self,
		path: &Path,
		symbol: &str,
	) -> Result<Option<DocEntry>, DocGenError> {
		// Delegate to local store directly
		// Daemon protocol returns DocEntryResponse
		// which needs mapping — keep simple for now
		doc_from_local(path, symbol)
	}

	fn search_docs(
		&self,
		path: &Path,
		query: &str,
		limit: usize,
	) -> Result<Vec<DocEntry>, DocGenError> {
		search_local(path, query, limit)
	}
}
