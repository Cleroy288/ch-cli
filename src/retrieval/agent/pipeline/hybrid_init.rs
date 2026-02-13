//! Hybrid search initialization logic.

use std::io::Write;

use crate::indexer::{IndexState, Symbol};
use crate::retrieval::hybrid::embedding_version::{
	invalidate_vectors, is_cache_current, write_version,
};
use crate::retrieval::hybrid::HybridSearch;
use crate::retrieval::RetrievalResult;

use super::core::RetrievalPipeline;

/// Initialize hybrid search with optional persistence
pub fn initialize_hybrid_search(
	pipeline: &mut RetrievalPipeline,
	symbols: &[Symbol],
) -> RetrievalResult<()> {
	let index_dir = hybrid_index_dir(pipeline);
	let tantivy_path = index_dir.join("tantivy");
	let vector_path = index_dir.join("vectors.bin");

	invalidate_stale_cache(pipeline, &index_dir);

	if try_load_hybrid_cache(
		pipeline, &tantivy_path, &vector_path,
	) {
		return Ok(());
	}

	build_fresh_hybrid(
		pipeline, symbols, &tantivy_path, &vector_path,
	)
}

/// Resolve the .rustean-index directory for this pipeline
pub(super) fn hybrid_index_dir(
	pipeline: &RetrievalPipeline,
) -> std::path::PathBuf {
	let project_path =
		std::path::Path::new(&pipeline.config.project_path);
	IndexState::index_dir(project_path)
}

/// Invalidate stale vector cache if persistence enabled
pub(super) fn invalidate_stale_cache(
	pipeline: &RetrievalPipeline,
	index_dir: &std::path::Path,
) {
	if pipeline.config.flags.enable_persistence
		&& !is_cache_current(index_dir)
	{
		invalidate_vectors(index_dir);
	}
}

/// Try loading hybrid search from persistent cache
/// Returns true if cache was loaded successfully
fn try_load_hybrid_cache(
	pipeline: &mut RetrievalPipeline,
	tantivy_path: &std::path::Path,
	vector_path: &std::path::Path,
) -> bool {
	let vectors_exist = vector_path.exists()
		|| vector_path.with_extension("json").exists();
	let can_load =
		pipeline.config.flags.enable_persistence
			&& tantivy_path.exists()
			&& vectors_exist;

	if !can_load {
		return false;
	}

	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Loading cached hybrid index..."
	);

	load_hybrid_from_paths(
		pipeline, tantivy_path, vector_path,
	)
}

/// Attempt to load hybrid from tantivy + vector paths
fn load_hybrid_from_paths(
	pipeline: &mut RetrievalPipeline,
	tantivy_path: &std::path::Path,
	vector_path: &std::path::Path,
) -> bool {
	match HybridSearch::with_paths(
		tantivy_path, vector_path,
	) {
		Ok(hybrid) => {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Hybrid index loaded"
			);
			pipeline.hybrid = Some(hybrid);
			true
		}
		Err(err) => {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Cache load failed, \
				rebuilding: {}",
				err
			);
			false
		}
	}
}

/// Build a fresh hybrid index and persist if enabled
fn build_fresh_hybrid(
	pipeline: &mut RetrievalPipeline,
	symbols: &[Symbol],
	tantivy_path: &std::path::Path,
	vector_path: &std::path::Path,
) -> RetrievalResult<()> {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Building hybrid search index..."
	);
	let mut hybrid =
		if pipeline.config.flags.enable_persistence {
			HybridSearch::with_paths(tantivy_path, vector_path)?
		} else {
			HybridSearch::new()?
		};

	hybrid.index_symbols(symbols)?;
	persist_hybrid(pipeline, &mut hybrid);
	pipeline.hybrid = Some(hybrid);
	Ok(())
}

/// Persist hybrid index to disk if enabled
fn persist_hybrid(
	pipeline: &RetrievalPipeline,
	hybrid: &mut HybridSearch,
) {
	if !pipeline.config.flags.enable_persistence {
		return;
	}
	if let Err(err) = hybrid.persist() {
		let _ = writeln!(
			std::io::stderr().lock(),
			"[pipeline] Warning: persist failed: {}",
			err
		);
	}
	let index_dir = hybrid_index_dir(pipeline);
	write_version(&index_dir);
}
