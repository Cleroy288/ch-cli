//! Documentation-related request handlers
//!
//! Handles doc generation start, status queries,
//! symbol lookup, and doc search requests.

use std::path::PathBuf;

use super::ModelDaemon;
use crate::retrieval::daemon::protocol::{
	DaemonResponse, DocEntryResponse,
};

/// Build "all complete" status response
fn all_complete_status(total: usize) -> DaemonResponse {
	DaemonResponse::DocGenStatus {
		total,
		completed: total,
		pending: 0,
		is_ready: true,
		in_progress: false,
	}
}

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

	if let Some(resp) = check_preconditions(
		daemon, &canonical, force,
	) {
		return resp;
	}

	super::doc_handlers_helpers
		::prepare_store(daemon, &canonical);

	let (total, pending_ids) =
		match super::doc_handlers_helpers
			::populate_store(daemon, &canonical)
		{
			Ok(val) => val,
			Err(resp) => return *resp,
		};

	if pending_ids.is_empty() {
		return all_complete_status(total);
	}

	super::doc_handlers_helpers::spawn_background_gen(
		daemon, canonical, total, pending_ids,
	)
}

/// Check if doc gen is already running or ready
fn check_preconditions(
	daemon: &ModelDaemon,
	canonical: &PathBuf,
	force: bool,
) -> Option<DaemonResponse> {
	if let Some(resp) = super::doc_handlers_helpers
		::check_already_running(daemon)
	{
		return Some(resp);
	}
	if !force {
		return super::doc_handlers_helpers
			::check_store_ready(daemon, canonical);
	}
	None
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

/// Handle GetDocByFile — lookup by file + name
pub fn handle_get_doc_by_file(
	daemon: &ModelDaemon,
	project_path: &str,
	file_path: &str,
	symbol_name: &str,
) -> DaemonResponse {
	super::doc_lookup::handle_get_doc_by_file(
		daemon,
		project_path,
		file_path,
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
	super::doc_search::handle_search_docs(
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

