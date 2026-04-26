//! Tests for Bitbucket URL building.

use rustean::cli::commands::mcp_server
	::mcp_exec_bb::build_bb_path;
use serde_json::json;

/// Simple workspace substitution
#[test]
fn build_path_workspace_only() {
	// Arrange
	let tmpl = "/workspaces/{workspace}/members";
	let args = json!({"workspace": "my-ws"});

	// Act
	let path = build_bb_path(tmpl, &args);

	// Assert
	assert_eq!(
		path,
		"/workspaces/my-ws/members?pagelen=100",
	);
}

/// Workspace + repo substitution
#[test]
fn build_path_workspace_and_repo() {
	// Arrange
	let tmpl = "/repositories/{workspace}\
		/{repo_slug}";
	let args = json!({
		"workspace": "ws",
		"repo_slug": "my-repo",
	});

	// Act
	let path = build_bb_path(tmpl, &args);

	// Assert
	assert_eq!(
		path,
		"/repositories/ws/my-repo?pagelen=100",
	);
}

/// Page param appended after pagelen
#[test]
fn build_path_with_page() {
	// Arrange
	let tmpl = "/workspaces";
	let args = json!({"page": 2});

	// Act
	let path = build_bb_path(tmpl, &args);

	// Assert
	assert_eq!(
		path,
		"/workspaces?pagelen=100&page=2",
	);
}

/// PR ID substitution
#[test]
fn build_path_with_pr_id() {
	// Arrange
	let tmpl = "/repositories/{workspace}\
		/{repo_slug}/pullrequests\
		/{pull_request_id}";
	let args = json!({
		"workspace": "ws",
		"repo_slug": "repo",
		"pull_request_id": "42",
	});

	// Act
	let path = build_bb_path(tmpl, &args);

	// Assert
	assert!(path.contains("/pullrequests/42"));
	assert!(path.contains("pagelen=100"));
}

/// No params leaves template placeholders
#[test]
fn build_path_missing_params() {
	// Arrange
	let tmpl = "/repositories/{workspace}";
	let args = json!({});

	// Act
	let path = build_bb_path(tmpl, &args);

	// Assert — placeholder remains, pagelen added
	assert_eq!(
		path,
		"/repositories/{workspace}?pagelen=100",
	);
}
