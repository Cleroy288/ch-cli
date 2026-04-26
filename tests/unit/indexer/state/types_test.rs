//! Tests for IndexState and ChangeSet types.

use std::path::PathBuf;

use rustean::indexer::state::{ChangeSet, IndexState, INDEX_VERSION};

/// has_changes is false for an empty changeset
#[test]
fn test_has_changes_empty() {
	// empty changeset with no additions, modifications, or deletions
	let cs = ChangeSet::default();

	assert!(!cs.has_changes());
}

/// has_changes is true when additions exist
#[test]
fn test_has_changes_with_additions() {
	// changeset with one added file
	let cs = ChangeSet {
		added: vec![PathBuf::from("src/main.rs")],
		..Default::default()
	};

	assert!(cs.has_changes());
}

/// files_to_index returns added + modified only
#[test]
fn test_files_to_index_added_and_modified() {
	let added_file = PathBuf::from("src/new.rs"); // a newly added file
	let modified_file = PathBuf::from("src/changed.rs"); // a modified file
	let cs = ChangeSet {
		added: vec![added_file.clone()],
		modified: vec![modified_file.clone()],
		deleted: vec![PathBuf::from("src/old.rs")],
		unchanged: vec![PathBuf::from("src/stable.rs")],
	}; // changeset with mixed changes

	let result = cs.files_to_index(); // files that need re-indexing

	assert_eq!(result.len(), 2);
	assert!(result.contains(&&added_file));
	assert!(result.contains(&&modified_file));
}

/// total_changes sums added + modified + deleted
#[test]
fn test_total_changes_known_counts() {
	// changeset with 2 added, 1 modified, 3 deleted, 1 unchanged
	let cs = ChangeSet {
		added: vec![PathBuf::from("a.rs"), PathBuf::from("b.rs")],
		modified: vec![PathBuf::from("c.rs")],
		deleted: vec![
			PathBuf::from("d.rs"),
			PathBuf::from("e.rs"),
			PathBuf::from("f.rs")
		],
		unchanged: vec![PathBuf::from("g.rs")],
	};

	assert_eq!(cs.total_changes(), 6);
}

/// IndexState::new creates empty state at current version
#[test]
fn test_index_state_new_creates_empty() {
	let root = PathBuf::from("/tmp/test_project"); // project root path

	// state: newly created IndexState
	let state = IndexState::new(root.clone());

	assert_eq!(state.version, INDEX_VERSION);
	assert!(state.files.is_empty());
	assert!(state.symbols.is_empty());
	assert!(state.references.is_empty());
	assert!(state.last_updated > 0);
}

/// IndexState::new canonicalizes the root path
#[test]
fn test_index_state_new_canonicalizes_root() {
	use tempfile::tempdir;

	let temp_dir = tempdir().unwrap(); // temporary directory for test
	let root = temp_dir.path().to_path_buf(); // project root

	// state: IndexState with canonicalized root
	let state = IndexState::new(root.clone());

	// The root should be canonical (absolute)
	assert!(state.root.is_absolute());
}

/// IndexState::default creates a valid empty state
#[test]
fn test_index_state_default() {
	// state: IndexState created via Default trait
	let state = IndexState::default();

	assert_eq!(state.version, INDEX_VERSION);
	assert!(state.files.is_empty());
}
