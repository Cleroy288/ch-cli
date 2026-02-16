//! Tests for IndexState change detection.

use tempfile::tempdir;

use rustean::indexer::state::IndexState;

/// Test update_file adds a new file to the index
#[test]
fn test_update_file_adds_new_file() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path().to_path_buf(); // project root
	let file_path = root.join("new.rs"); // new file path

	std::fs::write(&file_path, b"fn test() {}").unwrap();

	// state: empty IndexState for the project
	let mut state = IndexState::new(root.clone());
	let symbol_count = 3; // number of symbols extracted from the file

	state.update_file(&file_path, symbol_count).unwrap();

	assert_eq!(state.files.len(), 1);
	let stored = state.files.values().next().unwrap();
	assert_eq!(stored.symbol_count, symbol_count);
}

/// Test update_file updates existing file
#[test]
fn test_update_file_replaces_existing() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path().to_path_buf(); // project root
	let file_path = root.join("existing.rs"); // existing file path

	std::fs::write(&file_path, b"fn old() {}").unwrap();

	// state: IndexState with one file already indexed
	let mut state = IndexState::new(root.clone());
	state.update_file(&file_path, 1).unwrap();

	// Update the file content and re-index
	std::thread::sleep(std::time::Duration::from_millis(100));
	std::fs::write(&file_path, b"fn new() { let x = 1; }").unwrap();

	state.update_file(&file_path, 5).unwrap();

	assert_eq!(state.files.len(), 1);
	let stored = state.files.values().next().unwrap();
	assert_eq!(stored.symbol_count, 5);
}

/// Test remove_file deletes a file from the index
#[test]
fn test_remove_file_deletes_entry() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path().to_path_buf(); // project root
	let file_path = root.join("to_remove.rs"); // file to be removed

	std::fs::write(&file_path, b"fn remove_me() {}").unwrap();

	// state: IndexState with one file indexed
	let mut state = IndexState::new(root.clone());
	state.update_file(&file_path, 2).unwrap();
	assert_eq!(state.files.len(), 1);

	state.remove_file(&file_path);

	assert_eq!(state.files.len(), 0);
}

/// Test detect_changes identifies added files
#[test]
fn test_detect_changes_finds_added_files() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path().to_path_buf(); // project root

	// state: empty IndexState
	let state = IndexState::new(root.clone());

	// new_file: a newly created file on disk
	let new_file = root.join("added.rs");
	std::fs::write(&new_file, b"// new file").unwrap();

	// current_files: list of files currently on disk
	let current_files = vec![new_file.clone()];

	// changes: detected changes between index and disk
	let changes = state.detect_changes(&current_files);

	assert_eq!(changes.added.len(), 1);
	assert_eq!(changes.modified.len(), 0);
	assert_eq!(changes.deleted.len(), 0);
	assert!(changes.added.contains(&new_file));
}

/// Test detect_changes identifies modified files
#[test]
fn test_detect_changes_finds_modified_files() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path().to_path_buf(); // project root
	let file_path = root.join("modified.rs"); // file that will be modified

	std::fs::write(&file_path, b"// original").unwrap();

	// state: IndexState with the file indexed at original state
	let mut state = IndexState::new(root.clone());
	state.update_file(&file_path, 1).unwrap();

	// Modify the file
	std::thread::sleep(std::time::Duration::from_millis(100));
	std::fs::write(&file_path, b"// modified content").unwrap();

	// current_files: list of files currently on disk
	let current_files = vec![file_path.clone()];

	// changes: detected changes between index and disk
	let changes = state.detect_changes(&current_files);

	assert_eq!(changes.modified.len(), 1);
	assert_eq!(changes.added.len(), 0);
	assert!(changes.modified.contains(&file_path));
}

/// Test detect_changes identifies deleted files
#[test]
fn test_detect_changes_finds_deleted_files() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path().to_path_buf(); // project root
	let file_path = root.join("deleted.rs"); // file that will be deleted

	std::fs::write(&file_path, b"// will be deleted").unwrap();

	// state: IndexState with the file indexed before deletion
	let mut state = IndexState::new(root.clone());
	state.update_file(&file_path, 1).unwrap();

	// Delete the file from disk
	std::fs::remove_file(&file_path).unwrap();

	// current_files: empty list (no files on disk)
	let current_files = vec![];

	// changes: detected changes between index and disk
	let changes = state.detect_changes(&current_files);

	assert_eq!(changes.deleted.len(), 1);
	assert_eq!(changes.added.len(), 0);
	assert_eq!(changes.modified.len(), 0);
}

/// Test detect_changes identifies unchanged files
#[test]
fn test_detect_changes_finds_unchanged_files() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path().to_path_buf(); // project root
	let file_path = root.join("unchanged.rs"); // file that remains unchanged

	std::fs::write(&file_path, b"const X: i32 = 42;").unwrap();

	// state: IndexState with the file indexed
	let mut state = IndexState::new(root.clone());
	state.update_file(&file_path, 1).unwrap();

	// current_files: same file still exists, unmodified
	let current_files = vec![file_path.clone()];

	// changes: detected changes between index and disk
	let changes = state.detect_changes(&current_files);

	assert_eq!(changes.unchanged.len(), 1);
	assert_eq!(changes.added.len(), 0);
	assert_eq!(changes.modified.len(), 0);
	assert_eq!(changes.deleted.len(), 0);
	assert!(changes.unchanged.contains(&file_path));
}

/// Test touch updates the last_updated timestamp
#[test]
fn test_touch_updates_timestamp() {
	let temp_dir = tempdir().unwrap();
	let root = temp_dir.path().to_path_buf();

	let mut state = IndexState::new(root);
	// Force to 0 to avoid timing flakiness
	state.last_updated = 0;

	state.touch();

	assert!(state.last_updated > 0);
}
