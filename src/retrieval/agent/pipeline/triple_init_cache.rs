//! Triple hybrid cache loading logic.
//!
//! Checks for persisted vector files and loads them
//! to skip expensive GPU re-embedding on subsequent runs.

use std::io::Write;
use std::path::Path;

use crate::indexer::Symbol;
use crate::retrieval::hybrid::TripleHybridSearch;
use crate::retrieval::RetrievalResult;

use super::core::RetrievalPipeline;
use super::triple_init::{build_triple_search, wire_graph_data};

/// Try loading triple hybrid search from persistent cache.
/// Returns true if cache was loaded successfully.
pub(super) fn try_load_triple_cache(
	pipeline: &mut RetrievalPipeline,
	symbols: &[Symbol],
	index_dir: &Path,
) -> bool {
	if !pipeline.config.flags.enable_persistence {
		return false;
	}
	if !has_triple_vectors(index_dir) {
		return false;
	}

	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Loading cached triple index..."
	);

	load_and_wire_cache(pipeline, symbols, index_dir)
}

/// Check if code/vectors.bin (or legacy .json) exists
fn has_triple_vectors(index_dir: &Path) -> bool {
	let code_dir = index_dir.join("code");
	code_dir.join("vectors.bin").exists()
		|| code_dir.join("vectors.json").exists()
}

/// Attempt to load + wire cached triple hybrid
fn load_and_wire_cache(
	pipeline: &mut RetrievalPipeline,
	symbols: &[Symbol],
	index_dir: &Path,
) -> bool {
	match build_triple_search(pipeline, index_dir) {
		Ok(mut hybrid) => {
			if hybrid.vector_store.is_empty() {
				return false;
			}
			wire_graph_data(pipeline, &mut hybrid, symbols);
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Triple index loaded from cache"
			);
			pipeline.triple_hybrid = Some(hybrid);
			true
		}
		Err(err) => {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Cache load failed: {}",
				err
			);
			false
		}
	}
}

/// Detect vector dimension mismatch and clear stale cache
pub(super) fn check_dim_mismatch(
	hybrid: &mut TripleHybridSearch,
	daemon: &crate::retrieval::daemon::DaemonClient,
) -> RetrievalResult<()> {
	let cached_dim = hybrid.vector_store.vector_dim();
	if cached_dim == 0 {
		return Ok(()); // no cached vectors
	}

	let probe = daemon.embed(vec!["dim probe".to_string()])?;
	let model_dim = probe
		.first()
		.map(|vec| vec.len())
		.unwrap_or(0);

	if model_dim > 0 && cached_dim != model_dim {
		let _ = writeln!(
			std::io::stderr().lock(),
			"[pipeline] Dimension mismatch: \
			cached={}, model={}. Clearing vector cache.",
			cached_dim, model_dim
		);
		hybrid.vector_store.clear();
	}

	Ok(())
}
