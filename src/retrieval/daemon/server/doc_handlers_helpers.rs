//! Helper functions for doc generation handlers
//!
//! Private helpers used by handle_start_doc_gen:
//! progress checks, store prep, and background spawn.

use std::path::PathBuf;
use std::sync::atomic::Ordering;

use super::ModelDaemon;
use crate::indexer::{IndexManager, IndexState};
use crate::retrieval::daemon::protocol::DaemonResponse;
use crate::retrieval::docgen::DocStore;

/// Check if doc gen is already running
pub(super) fn check_already_running(
	daemon: &ModelDaemon,
) -> Option<DaemonResponse> {
	let progress =
		daemon.doc_gen_progress.lock().unwrap();
	if progress.is_running {
		Some(DaemonResponse::DocGenStatus {
			total: progress.total,
			completed: progress.completed,
			pending: progress
				.total
				.saturating_sub(progress.completed),
			is_ready: false,
			in_progress: true,
		})
	} else {
		None
	}
}

/// Check if store exists and is ready
pub(super) fn check_store_ready(
	daemon: &ModelDaemon,
	canonical: &PathBuf,
) -> Option<DaemonResponse> {
	let stores = daemon.doc_stores.lock().unwrap();
	if let Some(existing) = stores.get(canonical) {
		if existing.is_ready() {
			let stats = existing.stats();
			return Some(DaemonResponse::DocGenStatus {
				total: stats.total,
				completed: stats.ready,
				pending: stats.pending,
				is_ready: true,
				in_progress: false,
			});
		}
	}
	None
}

/// Prepare doc store: load or create, sync with index
pub(super) fn prepare_store(
	daemon: &mut ModelDaemon,
	canonical: &PathBuf,
) {
	// load or create doc store
	{
		let mut stores =
			daemon.doc_stores.lock().unwrap();
		if !stores.contains_key(canonical) {
			let store =
				DocStore::load(canonical).unwrap_or_else(
					|_| DocStore::new(canonical),
				);
			stores.insert(canonical.clone(), store);
		}
	}

	// sync with index state to mark stale as Pending
	{
		let mut stores =
			daemon.doc_stores.lock().unwrap();
		if let Some(store) = stores.get_mut(canonical) {
			if let Ok(state) = IndexState::load(canonical)
			{
				store.sync_with_index(&state);
			}
		}
	}
}

/// Get symbols and populate the store
///
/// Returns (total, pending_ids) or error response.
pub(super) fn populate_store(
	daemon: &mut ModelDaemon,
	canonical: &PathBuf,
) -> Result<(usize, Vec<String>), DaemonResponse> {
	// get symbols from cache or index
	let symbols = if let Some(cached) =
		daemon.project_cache.get(canonical)
	{
		cached.symbols.clone()
	} else {
		let manager = IndexManager::new()
			.with_persistence()
			.with_semantic_analysis()
			.with_reference_extraction();
		match manager.index_project(canonical) {
			Ok(result) => result.symbols,
			Err(e) => {
				return Err(DaemonResponse::Error(
					format!("indexing error: {}", e),
				))
			}
		}
	};

	let mut stores =
		daemon.doc_stores.lock().unwrap();
	let store = stores.get_mut(canonical).unwrap();
	store.populate_from_symbols(&symbols);
	Ok((store.len(), store.get_pending_ids()))
}

/// Spawn background doc generation thread
pub(super) fn spawn_background_gen(
	daemon: &mut ModelDaemon,
	canonical: PathBuf,
	total: usize,
	pending_ids: Vec<String>,
) -> DaemonResponse {
	let pending_count = pending_ids.len();

	// set progress before spawning
	{
		let mut progress =
			daemon.doc_gen_progress.lock().unwrap();
		progress.total = pending_count;
		progress.completed = 0;
		progress.failed = 0;
		progress.is_running = true;
	}

	// reset cancel flag
	daemon
		.doc_gen_cancel
		.store(false, Ordering::SeqCst);

	// extract graph for cross-references
	let graph_and_symbols = daemon
		.project_cache
		.get(&canonical)
		.and_then(|cached| {
			cached.graph.as_ref().map(|g| {
				(g.clone(), cached.symbols.clone())
			})
		});

	// clone shared state for background thread
	let stores = daemon.doc_stores.clone();
	let generator = daemon.doc_generator.clone();
	let progress = daemon.doc_gen_progress.clone();
	let cancel = daemon.doc_gen_cancel.clone();
	let path = canonical.clone();

	let handle = std::thread::spawn(move || {
		super::doc_generation::run_doc_gen_background(
			stores,
			generator,
			progress,
			cancel,
			path,
			pending_ids,
			graph_and_symbols,
		);
	});

	daemon.doc_gen_thread = Some(handle);

	DaemonResponse::DocGenStatus {
		total,
		completed: total.saturating_sub(pending_count),
		pending: pending_count,
		is_ready: false,
		in_progress: true,
	}
}
