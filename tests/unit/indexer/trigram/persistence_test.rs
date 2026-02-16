//! Tests for trigram index persistence.

use std::collections::HashSet;
use std::path::Path;

use tempfile::tempdir;

use rustean::indexer::trigram::{
	Trigram, TrigramIndex, string_to_trigram, trigram_to_string,
};

/// Test save and load roundtrip preserves index data
#[test]
fn test_save_load_roundtrip() {
	let dir = tempdir().unwrap(); // temp directory
	let file_path = dir.path().join("test_index.json"); // index file path
	let mut index = TrigramIndex::new(); // create new index

	// Add some trigrams
	index.index_file(Path::new("file1.rs"), "hello world");
	index.index_file(Path::new("file2.rs"), "world peace");

	index.save(&file_path).unwrap(); // save to disk
	let loaded = TrigramIndex::load(&file_path).unwrap(); // load from disk

	assert_eq!(loaded.file_count, index.file_count);
	assert_eq!(loaded.index.len(), index.index.len());
}

/// Test save creates valid JSON file
#[test]
fn test_save_creates_file() {
	let dir = tempdir().unwrap(); // temp directory
	let file_path = dir.path().join("index.json"); // index file path
	let index = TrigramIndex::new(); // empty index

	index.save(&file_path).unwrap(); // save to disk

	assert!(file_path.exists());
}

/// Test load fails gracefully for non-existent file
#[test]
fn test_load_nonexistent_file() {
	let path = Path::new("/nonexistent/path.json");
	let result = TrigramIndex::load(path);

	assert!(result.is_err());
}

/// Test trigram_to_string converts correctly
#[test]
fn test_trigram_to_string() {
	let trigram: Trigram = [0x48, 0x65, 0x6c]; // "Hel" in hex
	let s = trigram_to_string(&trigram); // convert to string

	assert_eq!(s, "48656c");
}

/// Test string_to_trigram converts correctly
#[test]
fn test_string_to_trigram() {
	let result = string_to_trigram("48656c"); // convert to trigram

	assert_eq!(result, Some([0x48, 0x65, 0x6c]));
}

/// Test string_to_trigram returns None for invalid length
#[test]
fn test_string_to_trigram_invalid_length() {
	let result = string_to_trigram("abc"); // invalid length string

	assert_eq!(result, None);
}

/// Test string_to_trigram returns None for invalid hex
#[test]
fn test_string_to_trigram_invalid_hex() {
	let result = string_to_trigram("gggggg"); // invalid hex characters

	assert_eq!(result, None);
}

/// Test save and load preserves trigram search functionality
#[test]
fn test_save_load_preserves_search() {
	let dir = tempdir().unwrap(); // temp directory
	let file_path = dir.path().join("search_test.json");
	let mut index = TrigramIndex::new(); // create new index

	// Add test data
	index.index_file(Path::new("test.rs"), "function");
	let original: HashSet<_> = index
		.candidate_files("fun")
		.into_iter()
		.collect();

	index.save(&file_path).unwrap(); // save to disk
	let loaded = TrigramIndex::load(&file_path).unwrap();
	let loaded_results: HashSet<_> = loaded
		.candidate_files("fun")
		.into_iter()
		.collect();

	assert_eq!(loaded_results, original);
}
