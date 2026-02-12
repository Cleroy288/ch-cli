//! Project-related request handlers
//!
//! Handles indexing and searching projects.

use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime};

use super::types::CachedProject;
use super::ModelDaemon;
use crate::indexer::IndexManager;
use crate::retrieval::daemon::protocol::{
	CachedSearchResult, DaemonResponse,
};
use crate::retrieval::hybrid::{
	HybridSearch, HybridSearchResult,
};

/// Handle IndexProject request - index and cache
pub fn handle_index_project(
	daemon: &mut ModelDaemon,
	project_path: &str,
	force: bool,
) -> DaemonResponse {
	let path = PathBuf::from(project_path);
	let canonical =
		path.canonicalize().unwrap_or(path.clone());

	if !force {
		if let Some(cached) =
			daemon.project_cache.get(&canonical)
		{
			return DaemonResponse::ProjectIndexed {
				symbol_count: cached.symbols.len(),
				cached: true,
				index_time_ms: 0,
			};
		}
	}

	match do_index_project(daemon, &canonical) {
		Ok(resp) => resp,
		Err(msg) => DaemonResponse::Error(msg),
	}
}

/// Build hybrid search index for symbols
fn build_hybrid_index(
	symbols: &[crate::indexer::Symbol],
) -> Result<HybridSearch, String> {
	let mut hybrid = HybridSearch::new()
		.map_err(|err| {
			format!("hybrid search error: {}", err)
		})?;
	hybrid.index_symbols(symbols).map_err(|err| {
		format!("index symbols error: {}", err)
	})?;
	Ok(hybrid)
}

/// Get current unix timestamp in seconds
fn unix_timestamp_secs() -> u64 {
	SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.map(|dur| dur.as_secs())
		.unwrap_or(0)
}

/// Insert indexed project into daemon cache
fn cache_project(
	daemon: &mut ModelDaemon,
	canonical: &Path,
	result: crate::indexer::IndexResult,
	hybrid: HybridSearch,
) -> usize {
	let count = result.symbols.len();
	daemon.project_cache.insert(
		canonical.to_path_buf(),
		CachedProject {
			symbols: result.symbols,
			graph: result.semantic_graph,
			hybrid,
			last_indexed: unix_timestamp_secs(),
		},
	);
	super::utils::evict_lru_if_needed(daemon);
	count
}

/// Perform the actual indexing and caching
fn do_index_project(
	daemon: &mut ModelDaemon,
	canonical: &PathBuf,
) -> Result<DaemonResponse, String> {
	let start = Instant::now();
	let manager = IndexManager::new()
		.with_persistence()
		.with_semantic_analysis();
	let result = manager
		.index_project(canonical)
		.map_err(|err| format!("indexing error: {}", err))?;

	let hybrid = build_hybrid_index(&result.symbols)?;
	let symbol_count =
		cache_project(daemon, canonical, result, hybrid);

	Ok(DaemonResponse::ProjectIndexed {
		symbol_count,
		cached: false,
		index_time_ms: start.elapsed().as_millis() as u64,
	})
}

/// Convert search hits to cached search results
fn convert_hits(
	hits: Vec<HybridSearchResult>,
) -> Vec<CachedSearchResult> {
	hits.into_iter()
		.map(|hit| CachedSearchResult {
			symbol_name: hit.symbol.name.clone(),
			symbol_kind: hit.symbol.kind.to_string(),
			file_path: hit.symbol.location.file
				.display().to_string(),
			line: hit.symbol.location.line,
			score: hit.score,
			rerank_score: hit.rerank_score,
		})
		.collect()
}

/// Handle SearchProject request
pub fn handle_search_project(
	daemon: &ModelDaemon,
	project_path: &str,
	query: &str,
	limit: usize,
) -> DaemonResponse {
	let path = PathBuf::from(project_path);
	let canonical = path.canonicalize().unwrap_or(path);

	let cached = match daemon.project_cache.get(&canonical) {
		Some(proj) => proj,
		None => return DaemonResponse::Error(
			"Project not cached".to_string(),
		),
	};
	match cached.hybrid.search(query, limit) {
		Ok(hits) => {
			DaemonResponse::SearchResults(
				convert_hits(hits),
			)
		}
		Err(err) => DaemonResponse::Error(
			format!("search error: {}", err),
		),
	}
}

/// Handle ProjectStatus request - check if a project is cached
pub fn handle_project_status(
	daemon: &ModelDaemon,
	project_path: &str,
) -> DaemonResponse {
	let path = PathBuf::from(project_path); // project path
	let canonical = path.canonicalize().unwrap_or(path); // normalized path

	match daemon.project_cache.get(&canonical) {
		Some(cached) => DaemonResponse::ProjectCacheStatus {
			cached: true,
			symbol_count: cached.symbols.len(),
			last_indexed: cached.last_indexed,
		},
		None => DaemonResponse::ProjectCacheStatus {
			cached: false,
			symbol_count: 0,
			last_indexed: 0,
		},
	}
}

/// Handle EvictProject request - remove from cache
#[allow(clippy::print_stderr)]
pub fn handle_evict_project(
	daemon: &mut ModelDaemon,
	project_path: &str,
) -> DaemonResponse {
	let path = PathBuf::from(project_path);
	let canonical = path.canonicalize().unwrap_or(path);

	if daemon.project_cache.remove(&canonical).is_some() {
		eprintln!(
			"[daemon] Evicted project: {:?}",
			canonical
		);
		DaemonResponse::Ok
	} else {
		DaemonResponse::Error(
			"Project not in cache".to_string(),
		)
	}
}
