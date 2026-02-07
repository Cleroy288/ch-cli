//! Project-related request handlers
//!
//! Handles indexing and searching projects.

use std::path::PathBuf;
use std::time::{Instant, SystemTime};

use super::types::CachedProject;
use super::ModelDaemon;
use crate::indexer::IndexManager;
use crate::retrieval::daemon::protocol::{CachedSearchResult, DaemonResponse};
use crate::retrieval::hybrid::HybridSearch;

/// Handle IndexProject request - index a project and cache in memory
pub fn handle_index_project(
	daemon: &mut ModelDaemon,
	project_path: &str,
	force: bool,
) -> DaemonResponse {
	let path = PathBuf::from(project_path); // project path
	// normalized path
	let canonical = path.canonicalize().unwrap_or(path.clone());

	// Check if already cached and not forcing
	if !force {
		if let Some(cached) = daemon.project_cache.get(&canonical) {
			return DaemonResponse::ProjectIndexed {
				symbol_count: cached.symbols.len(),
				cached: true,
				index_time_ms: 0,
			};
		}
	}

	// Index the project with persistence
	let start = Instant::now();
	let manager = IndexManager::new()
		.with_persistence()
		.with_semantic_analysis();

	let result = match manager.index_project(&canonical) {
		Ok(r) => r,
		Err(e) => {
			let msg = format!("indexing error: {}", e);
			return DaemonResponse::Error(msg);
		}
	};

	// Build hybrid search
	let mut hybrid = match HybridSearch::new() {
		Ok(h) => h,
		Err(e) => {
			let msg = format!("hybrid search error: {}", e);
			return DaemonResponse::Error(msg);
		}
	};

	if let Err(e) = hybrid.index_symbols(&result.symbols) {
		return DaemonResponse::Error(format!("index symbols error: {}", e));
	}

	let symbol_count = result.symbols.len();
	let index_time_ms = start.elapsed().as_millis() as u64;

	// Get current timestamp
	let now = SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.map(|d| d.as_secs())
		.unwrap_or(0);

	// Cache in memory
	daemon.project_cache.insert(
		canonical,
		CachedProject {
			symbols: result.symbols,
			graph: result.semantic_graph,
			hybrid,
			last_indexed: now,
		},
	);

	// LRU eviction if needed
	super::utils::evict_lru_if_needed(daemon);

	DaemonResponse::ProjectIndexed {
		symbol_count,
		cached: false,
		index_time_ms,
	}
}

/// Handle SearchProject request - search a cached project
pub fn handle_search_project(
	daemon: &ModelDaemon,
	project_path: &str,
	query: &str,
	limit: usize,
) -> DaemonResponse {
	let path = PathBuf::from(project_path); // project path
	let canonical = path.canonicalize().unwrap_or(path); // normalized path

	let cached = match daemon.project_cache.get(&canonical) {
		Some(c) => c,
		None => return DaemonResponse::Error("Project not cached".to_string()),
	};

	// Perform hybrid search
	let results = match cached.hybrid.search(query, limit) {
		Ok(r) => r,
		Err(e) => return DaemonResponse::Error(format!("search error: {}", e)),
	};

	// Convert to CachedSearchResult
	let cached_results: Vec<CachedSearchResult> = results
		.into_iter()
		.map(|r| CachedSearchResult {
			symbol_name: r.symbol.name,
			symbol_kind: r.symbol.kind.to_string(),
			file_path: r.symbol.location.file.display().to_string(),
			line: r.symbol.location.line,
			score: r.rrf_score,
			rerank_score: r.rerank_score,
		})
		.collect();

	DaemonResponse::SearchResults(cached_results)
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

/// Handle EvictProject request - remove a project from cache
pub fn handle_evict_project(
	daemon: &mut ModelDaemon,
	project_path: &str,
) -> DaemonResponse {
	let path = PathBuf::from(project_path); // project path
	let canonical = path.canonicalize().unwrap_or(path); // normalized path

	if daemon.project_cache.remove(&canonical).is_some() {
		eprintln!("[daemon] Evicted project from cache: {:?}", canonical);
		DaemonResponse::Ok
	} else {
		DaemonResponse::Error("Project not in cache".to_string())
	}
}
