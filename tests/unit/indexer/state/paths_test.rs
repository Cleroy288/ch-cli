//! Tests for IndexState path utilities.
//! Verifies structural relationships between paths.

use std::path::Path;

use rustean::indexer::state::IndexState;

/// index_dir delegates to data_paths::index_dir
#[test]
fn test_index_dir() {
	let root = Path::new("/home/user/project");
	let dir = IndexState::index_dir(root);
	assert!(dir.ends_with("index"));
}

/// state_file is state.json inside index dir
#[test]
fn test_state_file() {
	let root = Path::new("/home/user/project");
	let file = IndexState::state_file(root);
	assert!(file.starts_with(
		IndexState::index_dir(root),
	));
	assert!(file.ends_with("state.json"));
}

/// tantivy_dir is tantivy/ inside index dir
#[test]
fn test_tantivy_dir() {
	let root = Path::new("/home/user/project");
	let dir = IndexState::tantivy_dir(root);
	assert!(dir.starts_with(
		IndexState::index_dir(root),
	));
	assert!(dir.ends_with("tantivy"));
}

/// refs_dir is refs/ inside index dir
#[test]
fn test_refs_dir() {
	let root = Path::new("/home/user/project");
	let dir = IndexState::refs_dir(root);
	assert!(dir.starts_with(
		IndexState::index_dir(root),
	));
	assert!(dir.ends_with("refs"));
}

/// refs_file is refs.json inside index dir
#[test]
fn test_refs_file() {
	let root = Path::new("/home/user/project");
	let file = IndexState::refs_file(root);
	assert!(file.starts_with(
		IndexState::index_dir(root),
	));
	assert!(file.ends_with("refs.json"));
}

/// trigram_file is trigrams.json inside index dir
#[test]
fn test_trigram_file() {
	let root = Path::new("/home/user/project");
	let file = IndexState::trigram_file(root);
	assert!(file.starts_with(
		IndexState::index_dir(root),
	));
	assert!(file.ends_with("trigrams.json"));
}
