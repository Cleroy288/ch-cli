//! Tests for memory Tantivy search index.

use rustean::indexer::memory::search_index::MemorySearchIndex;

use crate::helpers::factories::{
	make_interaction_with_text,
};

/// Test in_memory index creation succeeds
#[test]
fn in_memory_creation_succeeds() {
	let idx = MemorySearchIndex::in_memory();
	assert!(idx.is_ok());
}

/// Test index then search finds result
#[test]
fn index_then_search_finds_result() {
	// Arrange
	let idx =
		MemorySearchIndex::in_memory().unwrap();
	let item = make_interaction_with_text(
		"auth", "JWT tokens used",
	);
	idx.index_interaction(&item).unwrap();

	// Act
	let hits = idx.search("auth", 10).unwrap();

	// Assert
	assert!(!hits.is_empty());
}

/// Test search on empty index returns nothing
#[test]
fn search_empty_index_returns_empty() {
	// Arrange
	let idx =
		MemorySearchIndex::in_memory().unwrap();

	// Act
	let hits = idx.search("test", 10).unwrap();

	// Assert
	assert!(hits.is_empty());
}

/// Test count returns correct number
#[test]
fn count_matches_indexed_documents() {
	// Arrange
	let idx =
		MemorySearchIndex::in_memory().unwrap();
	let item = make_interaction_with_text(
		"hello", "world",
	);
	idx.index_interaction(&item).unwrap();

	// Act
	let count = idx.count().unwrap();

	// Assert
	assert_eq!(count, 1);
}

/// Test search matches on response text
#[test]
fn search_matches_response_text() {
	// Arrange
	let idx =
		MemorySearchIndex::in_memory().unwrap();
	let item = make_interaction_with_text(
		"anything",
		"authentication middleware",
	);
	idx.index_interaction(&item).unwrap();

	// Act
	let hits = idx
		.search("middleware", 10)
		.unwrap();

	// Assert
	assert!(!hits.is_empty());
}
