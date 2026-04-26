//! Tests for git remote URL parsing.

use rustean::service::tools::git_remote::detect_repo_info;
use std::fs;
use tempfile::TempDir;

/// Detect SSH remote URL with host
#[test]
fn detect_ssh_remote() {
	// Arrange
	let dir = TempDir::new().unwrap();
	let git_dir = dir.path().join(".git");
	fs::create_dir(&git_dir).unwrap();
	let config = "[remote \"origin\"]\n\
		\turl = git@bitbucket.org:myws/myrepo.git\n";
	fs::write(git_dir.join("config"), config)
		.unwrap();

	// Act
	let info =
		detect_repo_info(dir.path()).unwrap();

	// Assert
	assert_eq!(info.host, "bitbucket.org");
	assert_eq!(info.workspace, "myws");
	assert_eq!(info.repo_slug, "myrepo");
}

/// Detect HTTPS remote URL with host
#[test]
fn detect_https_remote() {
	// Arrange
	let dir = TempDir::new().unwrap();
	let git_dir = dir.path().join(".git");
	fs::create_dir(&git_dir).unwrap();
	let config = "[remote \"origin\"]\n\
		\turl = https://bitbucket.org/ws/repo.git\n";
	fs::write(git_dir.join("config"), config)
		.unwrap();

	// Act
	let info =
		detect_repo_info(dir.path()).unwrap();

	// Assert
	assert_eq!(info.host, "bitbucket.org");
	assert_eq!(info.workspace, "ws");
	assert_eq!(info.repo_slug, "repo");
}

/// Returns None when no .git directory
#[test]
fn no_git_dir_returns_none() {
	// Arrange
	let dir = TempDir::new().unwrap();

	// Act
	let result = detect_repo_info(dir.path());

	// Assert
	assert!(result.is_none());
}

/// Returns None when no origin remote
#[test]
fn no_origin_remote_returns_none() {
	// Arrange
	let dir = TempDir::new().unwrap();
	let git_dir = dir.path().join(".git");
	fs::create_dir(&git_dir).unwrap();
	let config = "[remote \"upstream\"]\n\
		\turl = git@github.com:foo/bar.git\n";
	fs::write(git_dir.join("config"), config)
		.unwrap();

	// Act
	let result = detect_repo_info(dir.path());

	// Assert
	assert!(result.is_none());
}

/// GitHub SSH remote returns github.com host
#[test]
fn detect_github_ssh_host() {
	// Arrange
	let dir = TempDir::new().unwrap();
	let git_dir = dir.path().join(".git");
	fs::create_dir(&git_dir).unwrap();
	let config = "[remote \"origin\"]\n\
		\turl = git@github.com:owner/repo.git\n";
	fs::write(git_dir.join("config"), config)
		.unwrap();

	// Act
	let info =
		detect_repo_info(dir.path()).unwrap();

	// Assert
	assert_eq!(info.host, "github.com");
	assert_eq!(info.workspace, "owner");
	assert_eq!(info.repo_slug, "repo");
}
