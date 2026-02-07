//! Documentation Lookup Handlers
//!
//! Extracted from doc_handlers.rs for norm compliance.

use std::path::PathBuf;

use super::ModelDaemon;
use crate::retrieval::daemon::protocol::{
	DaemonResponse, DocEntryResponse,
};
use crate::retrieval::docgen::DocStore;

/// Handle GetDoc request - lookup a single symbol
pub fn handle_get_doc(
	daemon: &ModelDaemon,
	project_path: &str,
	symbol_name: &str,
) -> DaemonResponse {
	let path = PathBuf::from(project_path);
	let canonical = path.canonicalize().unwrap_or(path);

	let stores = daemon.doc_stores.lock().unwrap();
	match stores.get(&canonical) {
		Some(store) => match store.get_by_name(symbol_name)
		{
			Some(e) => DaemonResponse::Doc(Some(
				doc_entry_to_response(e),
			)),
			None => DaemonResponse::Doc(None),
		},
		None => {
			drop(stores);
			match DocStore::load(&canonical) {
				Ok(s) => match s.get_by_name(symbol_name) {
					Some(e) => DaemonResponse::Doc(Some(
						doc_entry_to_response(e),
					)),
					None => DaemonResponse::Doc(None),
				},
				Err(_) => DaemonResponse::Doc(None),
			}
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
	let canonical = path.canonicalize().unwrap_or(path);

	let stores = daemon.doc_stores.lock().unwrap();
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
			match DocStore::load(&canonical) {
				Ok(s) => {
					let results: Vec<DocEntryResponse> = s
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
	}
}

/// Convert a DocEntry to DocEntryResponse for IPC
pub fn doc_entry_to_response(
	entry: &crate::retrieval::docgen::DocEntry,
) -> DocEntryResponse {
	DocEntryResponse {
		name: entry.name.clone(),
		kind: entry.kind.to_string(),
		file_path: entry.file_path.display().to_string(),
		line: entry.line,
		user_comment: entry.user_comment.clone(),
		llm_doc: entry.llm_doc.clone(),
		signature: entry.signature.clone(),
		depends_on: entry.links.depends_on.clone(),
		depended_by: entry.links.depended_by.clone(),
		external_deps: entry.links.external_deps.clone(),
		status: entry.status.to_string(),
	}
}
