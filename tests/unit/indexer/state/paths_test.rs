//! Tests for IndexState path utilities.

use std::path::{Path, PathBuf};

use ch_cli::indexer::state::IndexState;

/// Test index_dir returns root joined with index directory name
#[test]
fn test_index_dir() {
	let root = Path::new("/home/user/project"); // root path
	// expected: expected index directory path
	let expected = PathBuf::from("/home/user/project/.ch-index");

	assert_eq!(IndexState::index_dir(root), expected);
}

/// Test state_file returns state.json path in index dir
#[test]
fn test_state_file() {
	let root = Path::new("/home/user/project"); // root path
	// expected: expected state file path
	let expected = PathBuf::from(
		"/home/user/project/.ch-index/state.json"
	);

	assert_eq!(IndexState::state_file(root), expected);
}

/// Test tantivy_dir returns tantivy subdir in index dir
#[test]
fn test_tantivy_dir() {
	let root = Path::new("/home/user/project"); // root path
	// expected: expected tantivy directory path
	let expected = PathBuf::from(
		"/home/user/project/.ch-index/tantivy"
	);

	assert_eq!(IndexState::tantivy_dir(root), expected);
}

/// Test refs_file returns refs.json path in index dir
#[test]
fn test_refs_file() {
	let root = Path::new("/home/user/project"); // root path
	// expected: expected refs file path
	let expected = PathBuf::from(
		"/home/user/project/.ch-index/refs.json"
	);

	assert_eq!(IndexState::refs_file(root), expected);
}

/// Test trigram_file returns trigrams.json in index dir
#[test]
fn test_trigram_file() {
	let root = Path::new("/home/user/project"); // root path
	// expected: expected trigram file path
	let expected = PathBuf::from(
		"/home/user/project/.ch-index/trigrams.json"
	);

	assert_eq!(IndexState::trigram_file(root), expected);
}
