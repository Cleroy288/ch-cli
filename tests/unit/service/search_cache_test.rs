//! Tests for IndexCache.

use rustean::service::search::cache::IndexCache;

use crate::helpers::factories_service::{
	cleanup_project, make_rust_project,
};

/// Empty directory returns empty symbols
#[test]
fn with_index_empty_dir_returns_no_symbols() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let cache = IndexCache::new();

	// Act
	let result = cache.with_index(
		dir.path(),
		|res| Ok(res.symbols.len()),
	);

	// Assert
	assert_eq!(result.unwrap(), 0);
}

/// with_index builds on first call, caches on second
#[test]
fn with_index_caches_on_second_call() {
	// Arrange
	let dir = make_rust_project("cache-test");
	let cache = IndexCache::new();

	// Act — first call builds the index
	let count_1 = cache.with_index(
		&dir, |res| Ok(res.symbols.len()),
	);

	// Act — second call uses cache
	let count_2 = cache.with_index(
		&dir, |res| Ok(res.symbols.len()),
	);

	// Assert — both succeed with same count
	let n1 = count_1.expect("first call");
	let n2 = count_2.expect("second call");
	assert_eq!(n1, n2);
	assert!(n1 > 0, "should find symbols");

	// Cleanup
	cleanup_project(&dir);
}

/// with_index closure receives valid IndexResult
#[test]
fn with_index_passes_valid_result() {
	// Arrange
	let dir = make_rust_project("cache-valid");
	let cache = IndexCache::new();

	// Act
	let has_graph = cache.with_index(
		&dir,
		|res| Ok(!res.symbols.is_empty()),
	);

	// Assert
	assert!(
		has_graph.expect("should succeed"),
		"result should have symbols",
	);

	// Cleanup
	cleanup_project(&dir);
}
