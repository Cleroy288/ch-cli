//! Helper functions for DefaultDocGenService.

use std::path::Path;

use crate::domain::errors::docgen::DocGenError;
use crate::retrieval::docgen::{DocEntry, DocStore};

use super::types::DocStatusInfo;

/// Convert path to string for daemon protocol
pub fn path_to_string(path: &Path) -> String {
	path.to_string_lossy().to_string()
}

/// Load status from local doc store
pub fn status_from_local(
	path: &Path,
) -> Result<DocStatusInfo, DocGenError> {
	match DocStore::load(path) {
		Ok(store) => {
			let total = store.len();
			let pending =
				store.get_pending().len();
			Ok(DocStatusInfo {
				total,
				completed: total - pending,
				pending,
				is_generating: false,
				is_ready: store.is_ready(),
			})
		}
		Err(_) => Ok(DocStatusInfo {
			total: 0,
			completed: 0,
			pending: 0,
			is_generating: false,
			is_ready: false,
		}),
	}
}

/// Load doc from local doc store
pub fn doc_from_local(
	path: &Path,
	symbol: &str,
) -> Result<Option<DocEntry>, DocGenError> {
	match DocStore::load(path) {
		Ok(store) => {
			Ok(store.get_by_name(symbol).cloned())
		}
		Err(_) => Ok(None),
	}
}

/// Search local doc store
pub fn search_local(
	path: &Path,
	query: &str,
	limit: usize,
) -> Result<Vec<DocEntry>, DocGenError> {
	match DocStore::load(path) {
		Ok(store) => Ok(store
			.search_by_name(query)
			.into_iter()
			.take(limit)
			.cloned()
			.collect()),
		Err(_) => Ok(vec![]),
	}
}
