//! Tests for memory path helpers.

use std::path::Path;

use rustean::indexer::memory::paths;

/// Test memory_dir returns correct path
#[test]
fn memory_dir_correct() {
	let root = Path::new("/project");
	let dir = paths::memory_dir(root);
	assert_eq!(
		dir, Path::new("/project/.rustean-memory"),
	);
}

/// Test sessions_dir is inside memory_dir
#[test]
fn sessions_dir_inside_memory() {
	let root = Path::new("/project");
	let dir = paths::sessions_dir(root);
	assert!(
		dir.starts_with(paths::memory_dir(root)),
	);
	assert!(dir.ends_with("sessions"));
}

/// Test session_file has .jsonl extension
#[test]
fn session_file_has_jsonl_ext() {
	let root = Path::new("/project");
	let file =
		paths::session_file(root, "s-123");
	assert!(
		file.to_string_lossy().ends_with(
			"s-123.jsonl"
		),
	);
}

/// Test tantivy_dir is inside memory_dir
#[test]
fn tantivy_dir_inside_memory() {
	let root = Path::new("/project");
	let dir = paths::tantivy_dir(root);
	assert!(
		dir.starts_with(paths::memory_dir(root)),
	);
	assert!(dir.ends_with("tantivy"));
}

/// Test meta_file is inside memory_dir
#[test]
fn meta_file_inside_memory() {
	let root = Path::new("/project");
	let file = paths::meta_file(root);
	assert!(
		file.starts_with(paths::memory_dir(root)),
	);
	assert!(file.ends_with("meta.json"));
}
