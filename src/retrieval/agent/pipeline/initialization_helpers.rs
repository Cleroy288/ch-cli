//! Shared helpers for pipeline initialization.
//!
//! Logging and doc store loading used by the
//! local initialization path.

use std::io::Write;

use crate::retrieval::docgen::DocStore;

use super::core::RetrievalPipeline;

/// Log incremental vs full index stats
pub(super) fn log_index_stats(
	result: &crate::indexer::IndexResult,
) {
	if result.incremental {
		let changes = result
			.changes
			.as_ref()
			.map(|chg| chg.total_changes())
			.unwrap_or(0);
		let _ = writeln!(
			std::io::stderr().lock(),
			"[pipeline] Incremental index: {} changes",
			changes
		);
	}
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Found {} symbols",
		result.symbols.len()
	);
}

/// Load documentation store for enhanced context
pub(super) fn load_doc_store(
	pipeline: &mut RetrievalPipeline,
	project_path: &std::path::Path,
) {
	match DocStore::load(project_path) {
		Ok(store) if store.is_ready() => {
			log_doc_store_loaded(&store);
			pipeline.doc_store = Some(store);
		}
		Ok(_) => {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Doc store not ready \
				(run 'rustean docs generate')"
			);
		}
		Err(_) => {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] No doc store found \
				(run 'rustean docs generate')"
			);
		}
	}
}

/// Log doc store load success with stats
fn log_doc_store_loaded(store: &DocStore) {
	let stats = store.stats();
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Loaded doc store \
		({} entries, {}% ready)",
		stats.total,
		(stats.completion_percent() as u32)
	);
}
