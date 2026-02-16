//! Tests for FileState.

use std::io::Write;
use std::path::PathBuf;

use tempfile::tempdir;

use rustean::indexer::state::FileState;

/// Test from_path creates FileState with correct metadata
#[test]
fn test_from_path_creates_valid_state() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path(); // project root
	let file_path = root.join("test.rs"); // test file path

	// content: some data to write to the file
	let content = b"fn main() {}";
	std::fs::write(&file_path, content).unwrap();

	// state: FileState created from the file
	let state = FileState::from_path(&file_path, root).unwrap();

	assert_eq!(state.path, PathBuf::from("test.rs"));
	assert!(state.mtime > 0);
	assert_eq!(state.size, content.len() as u64);
	assert_eq!(state.symbol_count, 0);
}

/// Test from_path with nested path
#[test]
fn test_from_path_nested_directory() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path(); // project root
	let nested_dir = root.join("src").join("lib"); // nested directory path
	std::fs::create_dir_all(&nested_dir).unwrap();

	let file_path = nested_dir.join("mod.rs"); // nested file path
	std::fs::write(&file_path, b"pub mod test;").unwrap();

	// state: FileState created from the nested file
	let state = FileState::from_path(&file_path, root).unwrap();

	assert_eq!(state.path, PathBuf::from("src/lib/mod.rs"));
	assert!(state.mtime > 0);
}

/// Test has_changed returns false for unchanged file
#[test]
fn test_has_changed_unchanged_file() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path(); // project root
	let file_path = root.join("unchanged.rs"); // test file path

	std::fs::write(&file_path, b"const X: i32 = 42;").unwrap();

	// state: FileState created from the file
	let state = FileState::from_path(&file_path, root).unwrap();

	// Wait a bit to ensure mtime would change if we modified
	std::thread::sleep(std::time::Duration::from_millis(10));

	assert!(!state.has_changed(root));
}

/// Test has_changed returns true for modified file
#[test]
fn test_has_changed_modified_file() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path(); // project root
	let file_path = root.join("modified.rs"); // test file path

	std::fs::write(&file_path, b"let x = 1;").unwrap();

	// state: FileState created from the file before modification
	let state = FileState::from_path(&file_path, root).unwrap();

	// Wait to ensure mtime changes
	std::thread::sleep(std::time::Duration::from_millis(100));

	// Modify the file
	let mut file = std::fs::OpenOptions::new()
		.append(true)
		.open(&file_path)
		.unwrap();
	file.write_all(b"\nlet y = 2;").unwrap();
	drop(file);

	assert!(state.has_changed(root));
}

/// Test has_changed returns true for deleted file
#[test]
fn test_has_changed_deleted_file() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path(); // project root
	let file_path = root.join("deleted.rs"); // test file path

	std::fs::write(&file_path, b"// will be deleted").unwrap();

	// state: FileState created from the file before deletion
	let state = FileState::from_path(&file_path, root).unwrap();

	// Delete the file
	std::fs::remove_file(&file_path).unwrap();

	assert!(state.has_changed(root));
}

/// Test has_changed detects size change without mtime change
#[test]
fn test_has_changed_size_only() {
	let temp_dir = tempdir().unwrap(); // temporary directory for test files
	let root = temp_dir.path(); // project root
	let file_path = root.join("size_change.rs"); // test file path

	std::fs::write(&file_path, b"short").unwrap();

	// state: FileState created from the file
	let mut state = FileState::from_path(&file_path, root).unwrap();

	// Manually change the stored size to simulate detection
	state.size = 999;

	assert!(state.has_changed(root));
}
