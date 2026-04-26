//! Tests for repo discovery scanner.

use std::fs;

use rustean::service::tools::repo_discovery;

/// Returns empty when no subdirectories
#[test]
fn empty_dir_returns_empty() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();

	// Act
	let repos =
		repo_discovery::discover_repos(
			dir.path(), 2,
		);

	// Assert
	assert!(repos.is_empty());
}

/// Skips directories without .git folder
#[test]
fn skips_non_git_dirs() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	fs::create_dir(root.join("some-project"))
		.unwrap();

	// Act
	let repos =
		repo_discovery::discover_repos(root, 2);

	// Assert
	assert!(repos.is_empty());
}

/// Skips node_modules directories
#[test]
fn skips_node_modules() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let nm = root.join("node_modules/pkg");
	fs::create_dir_all(&nm).unwrap();
	fs::create_dir(nm.join(".git")).unwrap();

	// Act
	let repos =
		repo_discovery::discover_repos(root, 3);

	// Assert
	assert!(repos.is_empty());
}

/// Respects max_depth limit
#[test]
fn respects_max_depth() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	// Create deeply nested dir (depth 3)
	let deep = root.join("a/b/c");
	fs::create_dir_all(&deep).unwrap();

	// Act — depth 0 scans only root children
	let repos =
		repo_discovery::discover_repos(root, 0);

	// Assert
	assert!(repos.is_empty());
}
