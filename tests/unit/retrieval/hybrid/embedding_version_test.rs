use std::fs;

use rustean::retrieval::hybrid::embedding_version::{
	invalidate_vectors, is_cache_current,
	write_version, EMBEDDING_FORMAT_VERSION,
};

#[test]
fn is_cache_current_returns_false_when_no_file() {
	let dir = tempfile::tempdir().unwrap();
	assert!(!is_cache_current(dir.path()));
}

#[test]
fn write_version_then_is_cache_current() {
	let dir = tempfile::tempdir().unwrap();

	write_version(dir.path());
	assert!(is_cache_current(dir.path()));
}

#[test]
fn is_cache_current_returns_false_on_old_version() {
	let dir = tempfile::tempdir().unwrap();
	let path = dir.path().join("embedding_version");
	fs::write(&path, "1").unwrap();

	// Current version is 7 (bincode), so old = stale
	assert_eq!(EMBEDDING_FORMAT_VERSION, 7);
	assert!(!is_cache_current(dir.path()));
}

#[test]
fn invalidate_vectors_removes_files() {
	let dir = tempfile::tempdir().unwrap();
	let json = dir.path().join("vectors.json");
	let bin = dir.path().join("vectors.bin");
	fs::write(&json, "{}").unwrap();
	fs::write(&bin, [0u8]).unwrap();

	invalidate_vectors(dir.path());

	assert!(!json.exists());
	assert!(!bin.exists());
}

#[test]
fn invalidate_vectors_noop_when_no_files() {
	let dir = tempfile::tempdir().unwrap();
	// Should not panic
	invalidate_vectors(dir.path());
}
