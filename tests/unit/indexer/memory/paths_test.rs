//! Tests for memory path helpers.
//! Verifies structural relationships between paths.

use std::path::Path;

use rustean::indexer::memory::paths;

/// memory_dir delegates to data_paths::memory_dir
#[test]
fn memory_dir_ends_with_memory() {
	let root = Path::new("/project");
	let dir = paths::memory_dir(root);
	assert!(dir.ends_with("memory"));
}

/// sessions_dir is a subdirectory of memory_dir
#[test]
fn sessions_dir_inside_memory() {
	let root = Path::new("/project");
	let dir = paths::sessions_dir(root);
	assert!(
		dir.starts_with(paths::memory_dir(root)),
	);
	assert!(dir.ends_with("sessions"));
}

/// session_file uses .jsonl extension
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

/// tantivy_dir is inside memory_dir
#[test]
fn tantivy_dir_inside_memory() {
	let root = Path::new("/project");
	let dir = paths::tantivy_dir(root);
	assert!(
		dir.starts_with(paths::memory_dir(root)),
	);
	assert!(dir.ends_with("tantivy"));
}

/// meta_file is inside memory_dir
#[test]
fn meta_file_inside_memory() {
	let root = Path::new("/project");
	let file = paths::meta_file(root);
	assert!(
		file.starts_with(paths::memory_dir(root)),
	);
	assert!(file.ends_with("meta.json"));
}
