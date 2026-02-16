//! Documentation Search Handlers
//!
//! Handles doc search requests against in-memory
//! or on-disk doc stores.

use std::path::{Path, PathBuf};

use super::ModelDaemon;
use crate::retrieval::daemon::protocol::{
	DaemonResponse, DocEntryResponse,
};
use crate::retrieval::docgen::DocStore;

use super::doc_lookup::doc_entry_to_response;

/// Search docs from disk store
fn search_from_disk(
	canonical: &Path,
	query: &str,
	limit: usize,
) -> DaemonResponse {
	match DocStore::load(canonical) {
		Ok(store) => {
			let results: Vec<DocEntryResponse> = store
				.search_by_name(query)
				.into_iter()
				.take(limit)
				.map(doc_entry_to_response)
				.collect();
			DaemonResponse::DocResults(results)
		}
		Err(_) => {
			DaemonResponse::DocResults(Vec::new())
		}
	}
}

/// Handle SearchDocs request
pub fn handle_search_docs(
	daemon: &ModelDaemon,
	project_path: &str,
	query: &str,
	limit: usize,
) -> DaemonResponse {
	let path = PathBuf::from(project_path);
	let canonical =
		path.canonicalize().unwrap_or(path);

	let Ok(stores) = daemon.doc_stores.lock() else {
		return search_from_disk(
			&canonical, query, limit,
		);
	};
	match stores.get(&canonical) {
		Some(store) => {
			let results: Vec<DocEntryResponse> = store
				.search_by_name(query)
				.into_iter()
				.take(limit)
				.map(doc_entry_to_response)
				.collect();
			DaemonResponse::DocResults(results)
		}
		None => {
			drop(stores);
			search_from_disk(
				&canonical, query, limit,
			)
		}
	}
}
