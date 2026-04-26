//! Tests for Jira URL building.

use rustean::cli::commands::mcp_server
	::mcp_exec_jira_path::build_jira_path;
use serde_json::json;

/// Issue key substitution
#[test]
fn build_path_issue_key() {
	// Arrange
	let tmpl = "/issue/{issue_key}";
	let args =
		json!({"issue_key": "PROJ-123"});

	// Act
	let path = build_jira_path(tmpl, &args);

	// Assert
	assert_eq!(path, "/issue/PROJ-123");
}

/// Project key substitution
#[test]
fn build_path_project_key() {
	// Arrange
	let tmpl = "/project/{project_key}";
	let args =
		json!({"project_key": "MYPROJ"});

	// Act
	let path = build_jira_path(tmpl, &args);

	// Assert
	assert_eq!(path, "/project/MYPROJ");
}

/// maxResults and startAt appended
#[test]
fn build_path_with_pagination() {
	// Arrange
	let tmpl = "/project";
	let args = json!({
		"max_results": 20,
		"start_at": 5,
	});

	// Act
	let path = build_jira_path(tmpl, &args);

	// Assert
	assert!(path.contains("maxResults=20"));
	assert!(path.contains("startAt=5"));
}

/// Fields param appended
#[test]
fn build_path_with_fields() {
	// Arrange
	let tmpl = "/issue/{issue_key}";
	let args = json!({
		"issue_key": "X-1",
		"fields": "summary,status",
	});

	// Act
	let path = build_jira_path(tmpl, &args);

	// Assert
	assert!(
		path.contains("fields=summary,status"),
	);
}

/// Empty args leaves template intact
#[test]
fn build_path_no_args() {
	// Arrange
	let tmpl = "/priority";
	let args = json!({});

	// Act
	let path = build_jira_path(tmpl, &args);

	// Assert
	assert_eq!(path, "/priority");
}
