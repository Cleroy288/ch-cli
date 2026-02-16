//! Documentation Lookup Handlers
//!
//! Extracted from doc_handlers.rs for norm compliance.

use std::path::{Path, PathBuf};

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

	let Ok(stores) = daemon.doc_stores.lock() else {
		return lookup_from_disk(&canonical, symbol_name);
	};
	match stores.get(&canonical) {
		Some(store) => {
			match store.get_by_name(symbol_name) {
				Some(entry) => DaemonResponse::Doc(Some(
					doc_entry_to_response(entry),
				)),
				None => DaemonResponse::Doc(None),
			}
		}
		None => {
			drop(stores);
			lookup_from_disk(&canonical, symbol_name)
		}
	}
}

/// Lookup a single symbol from disk store
fn lookup_from_disk(
	canonical: &Path,
	symbol_name: &str,
) -> DaemonResponse {
	match DocStore::load(canonical) {
		Ok(store) => match store.get_by_name(symbol_name)
		{
			Some(entry) => DaemonResponse::Doc(Some(
				doc_entry_to_response(entry),
			)),
			None => DaemonResponse::Doc(None),
		},
		Err(_) => DaemonResponse::Doc(None),
	}
}

/// Handle GetDocByFile — lookup by file + name
pub fn handle_get_doc_by_file(
	daemon: &ModelDaemon,
	project_path: &str,
	file_path: &str,
	symbol_name: &str,
) -> DaemonResponse {
	let path = PathBuf::from(project_path);
	let canonical = path.canonicalize().unwrap_or(path);
	let file = Path::new(file_path);

	let Ok(stores) = daemon.doc_stores.lock() else {
		return lookup_file_from_disk(
			&canonical, file, symbol_name,
		);
	};
	match stores.get(&canonical) {
		Some(store) => match store
			.get_by_file_and_name(file, symbol_name)
		{
			Some(entry) => DaemonResponse::Doc(Some(
				doc_entry_to_response(entry),
			)),
			None => DaemonResponse::Doc(None),
		},
		None => {
			drop(stores);
			lookup_file_from_disk(
				&canonical, file, symbol_name,
			)
		}
	}
}

/// Lookup symbol from disk by file + name
fn lookup_file_from_disk(
	canonical: &Path,
	file_path: &Path,
	symbol_name: &str,
) -> DaemonResponse {
	match DocStore::load(canonical) {
		Ok(store) => {
			match store
				.get_by_file_and_name(
					file_path, symbol_name,
				) {
				Some(entry) => DaemonResponse::Doc(
					Some(doc_entry_to_response(entry)),
				),
				None => DaemonResponse::Doc(None),
			}
		}
		Err(_) => DaemonResponse::Doc(None),
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
