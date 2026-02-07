//! Documentation-related request handlers
//!
//! Handles doc generation start, status queries,
//! symbol lookup, and doc search requests.

use std::path::PathBuf;

use super::ModelDaemon;
use crate::retrieval::daemon::protocol::{
	DaemonResponse, DocEntryResponse,
};

/// Handle StartDocGen request - spawn background thread
///
/// Returns immediately with current status. The actual
/// generation runs in a separate thread.
pub fn handle_start_doc_gen(
	daemon: &mut ModelDaemon,
	project_path: &str,
	force: bool,
) -> DaemonResponse {
	let path = PathBuf::from(project_path);
	let canonical =
		path.canonicalize().unwrap_or(path.clone());

	// check if already running
	let already = super::doc_handlers_helpers
		::check_already_running(daemon);
	if let Some(resp) = already {
		return resp;
	}

	// check if store is ready (skip if not forced)
	if !force {
		if let Some(resp) = super::doc_handlers_helpers
			::check_store_ready(daemon, &canonical)
		{
			return resp;
		}
	}

	// prepare store and pending entries
	super::doc_handlers_helpers
		::prepare_store(daemon, &canonical);

	// get symbols and populate store
	let (total, pending_ids) =
		match super::doc_handlers_helpers
			::populate_store(daemon, &canonical)
		{
			Ok(v) => v,
			Err(resp) => return resp,
		};

	if pending_ids.is_empty() {
		return DaemonResponse::DocGenStatus {
			total,
			completed: total,
			pending: 0,
			is_ready: true,
			in_progress: false,
		};
	}

	super::doc_handlers_helpers::spawn_background_gen(
		daemon,
		canonical,
		total,
		pending_ids,
	)
}

/// Handle DocGenStatus request
pub fn handle_doc_gen_status(
	daemon: &ModelDaemon,
	project_path: &str,
) -> DaemonResponse {
	super::doc_status::handle_doc_gen_status(
		daemon,
		project_path,
	)
}

/// Handle GetDoc request - lookup a single symbol
pub fn handle_get_doc(
	daemon: &ModelDaemon,
	project_path: &str,
	symbol_name: &str,
) -> DaemonResponse {
	super::doc_lookup::handle_get_doc(
		daemon,
		project_path,
		symbol_name,
	)
}

/// Handle SearchDocs request
pub fn handle_search_docs(
	daemon: &ModelDaemon,
	project_path: &str,
	query: &str,
	limit: usize,
) -> DaemonResponse {
	super::doc_lookup::handle_search_docs(
		daemon,
		project_path,
		query,
		limit,
	)
}

/// Convert a DocEntry to DocEntryResponse for IPC
pub fn doc_entry_to_response(
	entry: &crate::retrieval::docgen::DocEntry,
) -> DocEntryResponse {
	super::doc_lookup::doc_entry_to_response(entry)
}

