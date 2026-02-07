//! Tests for retrieval::daemon::server::utils

use std::path::PathBuf;

use ch_cli::retrieval::daemon::server::lifecycle::{
	with_socket_path,
};
use ch_cli::retrieval::daemon::server::types::{
	CachedProject, MAX_CACHED_PROJECTS,
};
use ch_cli::retrieval::daemon::server::utils::{
	evict_lru_if_needed,
};
use ch_cli::retrieval::hybrid::HybridSearch;

/// Test evict_lru_if_needed does not evict
/// when cache is under limit
#[test]
fn test_evict_lru_no_eviction_needed() {
	let mut daemon =
		with_socket_path("/tmp/test-evict.sock");

	// Add projects under limit (MAX = 5)
	for i in 0..3 {
		let path = PathBuf::from(
			format!("/tmp/project{}", i),
		);
		let project = CachedProject {
			symbols: vec![],
			graph: None,
			hybrid: HybridSearch::new()
				.expect("Failed to create HybridSearch"),
			last_indexed: i as u64,
		};
		daemon.project_cache.insert(path, project);
	}

	let initial_count = daemon.project_cache.len();
	evict_lru_if_needed(&mut daemon);

	// Should not evict anything
	assert_eq!(
		daemon.project_cache.len(),
		initial_count,
	);
}

/// Test evict_lru_if_needed evicts oldest project
/// when over limit
#[test]
fn test_evict_lru_evicts_oldest() {
	let mut daemon = with_socket_path(
		"/tmp/test-evict-oldest.sock",
	);

	// Add projects exceeding the limit
	for i in 0..(MAX_CACHED_PROJECTS + 2) {
		let path = PathBuf::from(
			format!("/tmp/project{}", i),
		);
		let project = CachedProject {
			symbols: vec![],
			graph: None,
			hybrid: HybridSearch::new()
				.expect("Failed to create HybridSearch"),
			last_indexed: i as u64 + 1000,
		};
		daemon.project_cache.insert(path, project);
	}

	evict_lru_if_needed(&mut daemon);

	// Should evict down to MAX_CACHED_PROJECTS
	assert_eq!(
		daemon.project_cache.len(),
		MAX_CACHED_PROJECTS,
	);

	// Oldest projects should be evicted
	assert!(!daemon.project_cache.contains_key(
		&PathBuf::from("/tmp/project0")
	));
	assert!(!daemon.project_cache.contains_key(
		&PathBuf::from("/tmp/project1")
	));
}

/// Test evict_lru_if_needed with empty cache
#[test]
fn test_evict_lru_empty_cache() {
	let mut daemon = with_socket_path(
		"/tmp/test-evict-empty.sock",
	);

	// Empty cache should not panic
	evict_lru_if_needed(&mut daemon);

	assert_eq!(daemon.project_cache.len(), 0);
}

/// Test evict_lru_if_needed evicts exactly one
/// project at a time
#[test]
fn test_evict_lru_evicts_incrementally() {
	let mut daemon = with_socket_path(
		"/tmp/test-evict-incremental.sock",
	);

	// Add exactly MAX + 1 projects
	for i in 0..=MAX_CACHED_PROJECTS {
		let path = PathBuf::from(
			format!("/tmp/project{}", i),
		);
		let project = CachedProject {
			symbols: vec![],
			graph: None,
			hybrid: HybridSearch::new()
				.expect("Failed to create HybridSearch"),
			last_indexed: i as u64,
		};
		daemon.project_cache.insert(path, project);
	}

	assert_eq!(
		daemon.project_cache.len(),
		MAX_CACHED_PROJECTS + 1,
	);

	evict_lru_if_needed(&mut daemon);

	// Should evict exactly one project
	assert_eq!(
		daemon.project_cache.len(),
		MAX_CACHED_PROJECTS,
	);

	// Oldest project (timestamp 0) should be evicted
	assert!(!daemon.project_cache.contains_key(
		&PathBuf::from("/tmp/project0")
	));
}
