//! Helper functions for doc generation handlers
//!
//! Private helpers used by handle_start_doc_gen:
//! progress checks, store prep, and background spawn.

use std::path::PathBuf;
use std::sync::atomic::Ordering;

use super::ModelDaemon;
use crate::indexer::{
	IndexManager, IndexState, SymbolKind,
};
use crate::retrieval::daemon::protocol::DaemonResponse;
use crate::retrieval::docgen::DocStore;

/// Check if doc gen is already running
pub(super) fn check_already_running(
	daemon: &ModelDaemon,
) -> Option<DaemonResponse> {
	let Ok(progress) =
		daemon.doc_gen_progress.lock()
	else {
		return None;
	};
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
	let Ok(stores) = daemon.doc_stores.lock() else {
		return None;
	};
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
	ensure_store_exists(daemon, canonical);
	sync_store_with_index(daemon, canonical);
}

/// Ensure a doc store exists for the given path
fn ensure_store_exists(
	daemon: &mut ModelDaemon,
	canonical: &PathBuf,
) {
	let Ok(mut stores) =
		daemon.doc_stores.lock()
	else {
		return;
	};
	if !stores.contains_key(canonical) {
		let store =
			DocStore::load(canonical).unwrap_or_else(
				|_| DocStore::new(canonical),
			);
		stores.insert(canonical.clone(), store);
	}
}

/// Sync doc store entries with current index state
fn sync_store_with_index(
	daemon: &mut ModelDaemon,
	canonical: &PathBuf,
) {
	let Ok(mut stores) =
		daemon.doc_stores.lock()
	else {
		return;
	};
	if let Some(store) = stores.get_mut(canonical) {
		if let Ok(state) = IndexState::load(canonical)
		{
			store.sync_with_index(&state);
		}
	}
}

/// Result type for store population
type PopulateResult =
	Result<(usize, Vec<String>), Box<DaemonResponse>>;

/// Result of fetching symbols for doc generation
type FetchSymbolsResult =
	Result<Vec<crate::indexer::Symbol>, Box<DaemonResponse>>;

/// Fetch symbols from cache or fresh index
fn fetch_symbols(
	daemon: &mut ModelDaemon,
	canonical: &PathBuf,
) -> FetchSymbolsResult {
	if let Some(cached) =
		daemon.project_cache.get(canonical)
	{
		return Ok(cached.symbols.clone());
	}
	let manager = IndexManager::new()
		.with_semantic_analysis()
		.with_reference_extraction();
	match manager.index_project(canonical) {
		Ok(result) => Ok(result.symbols),
		Err(err) => Err(Box::new(
			DaemonResponse::Error(
				format!("indexing error: {}", err),
			),
		)),
	}
}

/// Get symbols and populate the store
///
/// Returns (total, pending_ids) or error response.
pub(super) fn populate_store(
	daemon: &mut ModelDaemon,
	canonical: &PathBuf,
) -> PopulateResult {
	let all_symbols = fetch_symbols(daemon, canonical)?;
	let symbols: Vec<_> = all_symbols
		.into_iter()
		.filter(|sym| matches!(
			sym.kind,
			SymbolKind::Function
				| SymbolKind::Method
				| SymbolKind::Struct
				| SymbolKind::Enum
				| SymbolKind::Trait
		))
		.collect();
	let mut stores = daemon.doc_stores.lock()
		.map_err(|_| Box::new(DaemonResponse::Error(
			"lock poisoned".into(),
		)))?;
	let Some(store) = stores.get_mut(canonical) else {
		return Err(Box::new(DaemonResponse::Error(
			"store not found".into(),
		)));
	};
	store.populate_from_symbols(&symbols);
	Ok((store.len(), store.get_pending_ids()))
}

/// Initialize progress state before spawning
fn init_doc_gen_progress(
	daemon: &mut ModelDaemon,
	pending_count: usize,
) {
	let Ok(mut progress) =
		daemon.doc_gen_progress.lock()
	else {
		return;
	};
	progress.total = pending_count;
	progress.completed = 0;
	progress.failed = 0;
	progress.is_running = true;
	daemon
		.doc_gen_cancel
		.store(false, Ordering::SeqCst);
}

/// Build doc gen context from daemon shared state
fn build_doc_gen_context(
	daemon: &ModelDaemon,
) -> super::doc_generation::DocGenContext {
	super::doc_generation::DocGenContext {
		stores: daemon.doc_stores.clone(),
		generator: daemon.doc_generator.clone(),
		progress: daemon.doc_gen_progress.clone(),
		cancel: daemon.doc_gen_cancel.clone(),
	}
}

/// Spawn background doc generation thread
pub(super) fn spawn_background_gen(
	daemon: &mut ModelDaemon,
	canonical: PathBuf,
	total: usize,
	pending_ids: Vec<String>,
) -> DaemonResponse {
	let pending_count = pending_ids.len();
	init_doc_gen_progress(daemon, pending_count);

	let graph_and_symbols = daemon
		.project_cache
		.get(&canonical)
		.and_then(|cached| {
			cached.graph.as_ref().map(|graph| {
				(graph.clone(), cached.symbols.clone())
			})
		});

	let ctx = build_doc_gen_context(daemon);
	let path = canonical.clone();
	let handle = std::thread::spawn(move || {
		super::doc_generation::run_doc_gen_background(
			ctx, path, pending_ids, graph_and_symbols,
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
