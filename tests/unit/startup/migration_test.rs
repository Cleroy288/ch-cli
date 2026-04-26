//! Tests for the data layout migration.

use std::fs;

use rustean::domain::data_paths;
use rustean::startup::migration
	::migrate_data_layout;

/// Migrates .rustean-data/ to centralized dir
#[test]
fn migrates_local_to_centralized() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let local = root.join(".rustean-data");
	fs::create_dir_all(local.join("index"))
		.unwrap();
	fs::write(
		local.join("index/state.json"), "{}",
	)
	.unwrap();

	// Act
	migrate_data_layout(root);

	// Assert — local removed
	assert!(!local.exists());
	// Assert — centralized created
	let target = data_paths::data_dir(root);
	assert!(target.join("index/state.json").exists());
}

/// Skips when no old layout found
#[test]
fn skip_when_no_old_layout() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();

	// Act
	migrate_data_layout(root);

	// Assert — nothing created locally
	assert!(!root.join(".rustean-data").exists());
}

/// Moves old index dir to centralized
#[test]
fn migrates_old_index_dir() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let idx = root.join(".rustean-index");
	fs::create_dir(&idx).unwrap();
	fs::write(idx.join("state.json"), "{}")
		.unwrap();

	// Act
	migrate_data_layout(root);

	// Assert — old dir gone
	assert!(!idx.exists());
	assert!(!root.join(".rustean-data").exists());
	// Assert — in centralized
	let target = data_paths::data_dir(root);
	assert!(target.join("index/state.json").exists());
}

/// Moves old credentials to centralized
#[test]
fn migrates_old_credentials() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let creds = root
		.join(".rustean-credentials.json");
	fs::write(&creds, "{}").unwrap();

	// Act
	migrate_data_layout(root);

	// Assert
	assert!(!creds.exists());
	assert!(!root.join(".rustean-data").exists());
}
