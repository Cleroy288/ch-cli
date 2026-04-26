//! Tests for memory Tantivy search index.

use rustean::indexer::memory::search_index::MemorySearchIndex;

use crate::helpers::factories::{
	make_interaction_with_text,
};

/// in_memory constructor succeeds
#[test]
fn in_memory_creation_succeeds() {
	let idx = MemorySearchIndex::in_memory();
	assert!(idx.is_ok());
}

/// Indexed interaction is found by query term
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

/// Empty index yields no search results
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

/// count reflects number of indexed documents
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

/// Search matches terms in the response text
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
