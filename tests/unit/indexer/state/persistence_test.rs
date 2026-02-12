//! Tests for IndexState persistence (save/load).

use std::path::PathBuf;

use tempfile::tempdir;

use rustean::indexer::semantic::{ReferenceContext, SymbolReference};
use rustean::indexer::state::IndexState;
use rustean::indexer::symbols::CodeLocation;

/// Test save and load roundtrip preserves state
#[test]
fn test_save_load_roundtrip() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test
	let root = temp_dir.path().to_path_buf(); // project root

	// original_state: IndexState to be saved
	let mut original_state = IndexState::new(root.clone());
	let test_file = root.join("test.rs");
	std::fs::write(&test_file, b"fn main() {}").unwrap();
	original_state.update_file(&test_file, 5).unwrap();
	original_state.touch();

	// Save the state
	original_state.save().unwrap();

	// loaded_state: IndexState loaded from disk
	let loaded_state = IndexState::load(&root).unwrap();

	assert_eq!(loaded_state.version, original_state.version);
	assert_eq!(loaded_state.files.len(), original_state.files.len());
	assert_eq!(loaded_state.last_updated, original_state.last_updated);
}

/// Test load returns error for non-existent state
#[test]
fn test_load_nonexistent_returns_error() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test
	let root = temp_dir.path(); // project root with no index

	// result: attempt to load non-existent state
	let result = IndexState::load(root);

	assert!(result.is_err());
}

/// Test load rejects incompatible version
#[test]
fn test_load_rejects_incompatible_version() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test
	let root = temp_dir.path().to_path_buf(); // project root

	// state: IndexState with modified version
	let mut state = IndexState::new(root.clone());
	state.version = 999; // incompatible version number

	state.save().unwrap();

	// result: attempt to load with incompatible version
	let result = IndexState::load(&root);

	assert!(result.is_err());
	let err_msg = result.unwrap_err().to_string();
	assert!(err_msg.contains("version mismatch"));
}

/// Test save creates index directory
#[test]
fn test_save_creates_index_dir() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test
	let root = temp_dir.path().to_path_buf(); // project root

	// state: new IndexState
	let state = IndexState::new(root.clone());

	state.save().unwrap();

	// index_dir: path to the created index directory
	let index_dir = IndexState::index_dir(&root);

	assert!(index_dir.exists());
	assert!(index_dir.is_dir());
}

/// Test save_references and load_references roundtrip
#[test]
fn test_save_load_references_roundtrip() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test
	let root = temp_dir.path().to_path_buf(); // project root

	// state: IndexState with some references
	let mut state = IndexState::new(root.clone());
	state.references = vec![
		SymbolReference {
			name: "test_func".to_string(),
			location: CodeLocation {
				file: PathBuf::from("src/main.rs"),
				line: 10,
				column: 5,
				byte_offset: 100,
				byte_length: 9,
			},
			context: ReferenceContext::Call,
		},
		SymbolReference {
			name: "MyType".to_string(),
			location: CodeLocation {
				file: PathBuf::from("src/lib.rs"),
				line: 20,
				column: 10,
				byte_offset: 200,
				byte_length: 6,
			},
			context: ReferenceContext::Type,
		},
	];

	// Save references
	state.save_references().unwrap();

	// loaded_refs: references loaded from disk
	let loaded_refs = IndexState::load_references(&root).unwrap();

	assert_eq!(loaded_refs.len(), 2);
	// Order-independent: per-file storage may reorder
	let mut names: Vec<&str> =
		loaded_refs.iter().map(|r| r.name.as_str()).collect();
	names.sort();
	assert_eq!(names, vec!["MyType", "test_func"]);
}

/// Test load_references returns empty vec for non-existent file
#[test]
fn test_load_references_nonexistent_returns_empty() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test
	let root = temp_dir.path(); // project root with no refs file

	// refs: loaded references (should be empty)
	let refs = IndexState::load_references(root).unwrap();

	assert!(refs.is_empty());
}

/// Test exists returns true when state file exists
#[test]
fn test_exists_returns_true_when_present() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test
	let root = temp_dir.path().to_path_buf(); // project root

	// state: IndexState to create the state file
	let state = IndexState::new(root.clone());
	state.save().unwrap();

	assert!(IndexState::exists(&root));
}

/// Test exists returns false when state file does not exist
#[test]
fn test_exists_returns_false_when_absent() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test
	let root = temp_dir.path(); // project root with no index

	assert!(!IndexState::exists(root));
}
