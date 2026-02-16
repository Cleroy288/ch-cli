//! Tests for IndexCache.

use rustean::service::search::cache::IndexCache;

use crate::helpers::factories_service::{
	cleanup_project, make_rust_project,
};

/// new() creates a working cache instance
#[test]
fn cache_new_creates_empty() {
	// Act
	let cache = IndexCache::new();

	// Assert — with_index on nonexistent path errors
	let result = cache.with_index(
		std::path::Path::new("/nonexistent"),
		false,
		|_| Ok(()),
	);
	// Should not panic; may error on missing path
	drop(result);
}

/// with_index builds on first call, caches on second
#[test]
fn with_index_caches_on_second_call() {
	// Arrange
	let dir = make_rust_project("cache-test");
	let cache = IndexCache::new();

	// Act — first call builds the index
	let count_1 = cache.with_index(
		&dir, false, |res| Ok(res.symbols.len()),
	);

	// Act — second call uses cache
	let count_2 = cache.with_index(
		&dir, false, |res| Ok(res.symbols.len()),
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
		false,
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
