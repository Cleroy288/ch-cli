//! Tests for repo cache persistence.

use std::fs;

use rustean::domain::repo_info::{
	RepoCache, RepoEntry,
};
use rustean::service::tools::repo_cache;

/// Build a test cache with one entry
fn make_cache() -> RepoCache {
	RepoCache {
		scan_depth: 2,
		discovered_at: "99999".into(),
		repos: vec![RepoEntry {
			folder: "api".into(),
			path: "./api".into(),
			host: "bitbucket.org".into(),
			workspace: "ws".into(),
			repo_slug: "api".into(),
		}],
	}
}

/// Save and load returns same data
#[test]
fn save_and_load_roundtrip() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let cache = make_cache();

	// Act
	repo_cache::save_repos(root, &cache).unwrap();
	let loaded =
		repo_cache::load_repos(root).unwrap();

	// Assert
	assert_eq!(loaded.scan_depth, 2);
	assert_eq!(loaded.repos.len(), 1);
	assert_eq!(loaded.repos[0].folder, "api");
}

/// has_repos_cache returns false when no file
#[test]
fn has_cache_false_when_empty() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();

	// Act + Assert
	assert!(!repo_cache::has_repos_cache(
		dir.path(),
	));
}

/// has_repos_cache returns true after save
#[test]
fn has_cache_true_after_save() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let cache = make_cache();

	// Act
	repo_cache::save_repos(root, &cache).unwrap();

	// Assert
	assert!(repo_cache::has_repos_cache(root));
}

/// Load returns None when file does not exist
#[test]
fn load_returns_none_when_missing() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();

	// Act + Assert
	assert!(
		repo_cache::load_repos(dir.path()).is_none()
	);
}

/// Load returns None for invalid config JSON
#[test]
fn load_returns_none_for_bad_json() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let path =
		rustean::domain::data_paths_dirs::config_file(root);
	fs::create_dir_all(path.parent().unwrap())
		.unwrap();
	fs::write(&path, "not json").unwrap();

	// Act + Assert
	assert!(repo_cache::load_repos(root).is_none());
}
