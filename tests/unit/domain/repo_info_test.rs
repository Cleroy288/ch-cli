//! Tests for RepoEntry and RepoCache serde.

use rustean::domain::repo_info::{
	RepoCache, RepoEntry,
};

/// Build a test RepoEntry
fn make_entry() -> RepoEntry {
	RepoEntry {
		folder: "backend".into(),
		path: "./backend".into(),
		host: "bitbucket.org".into(),
		workspace: "myteam".into(),
		repo_slug: "backend".into(),
	}
}

/// RepoEntry serializes to JSON and back
#[test]
fn repo_entry_serde_roundtrip() {
	// Arrange
	let entry = make_entry();

	// Act
	let json =
		serde_json::to_string(&entry).unwrap();
	let back: RepoEntry =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert_eq!(back.folder, "backend");
	assert_eq!(back.workspace, "myteam");
	assert_eq!(back.repo_slug, "backend");
}

/// RepoCache serializes to JSON and back
#[test]
fn repo_cache_serde_roundtrip() {
	// Arrange
	let cache = RepoCache {
		scan_depth: 2,
		discovered_at: "12345".into(),
		repos: vec![make_entry()],
	};

	// Act
	let json =
		serde_json::to_string(&cache).unwrap();
	let back: RepoCache =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert_eq!(back.scan_depth, 2);
	assert_eq!(back.discovered_at, "12345");
	assert_eq!(back.repos.len(), 1);
}

/// Empty repos list roundtrips correctly
#[test]
fn repo_cache_empty_repos_roundtrip() {
	// Arrange
	let cache = RepoCache {
		scan_depth: 3,
		discovered_at: "0".into(),
		repos: vec![],
	};

	// Act
	let json =
		serde_json::to_string(&cache).unwrap();
	let back: RepoCache =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert_eq!(back.repos.len(), 0);
	assert_eq!(back.scan_depth, 3);
}
