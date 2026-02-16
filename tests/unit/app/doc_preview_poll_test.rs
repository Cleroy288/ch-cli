//! Tests for strip_project_prefix utility.

use rustean::app::handlers::doc_preview_fetch
	::strip_project_prefix;

/// Strips matching project prefix from absolute path
#[test]
fn strip_prefix_removes_project_path() {
	// Arrange
	let abs = "/home/user/project/src/main.rs";
	let project = "/home/user/project";

	// Act
	let result = strip_project_prefix(abs, project);

	// Assert
	assert_eq!(result, "src/main.rs");
}

/// Returns original when prefix does not match
#[test]
fn strip_prefix_no_match_returns_original() {
	let abs = "/other/path/src/main.rs";
	let project = "/home/user/project";

	let result = strip_project_prefix(abs, project);

	assert_eq!(result, "/other/path/src/main.rs");
}

/// Handles trailing slash on project path
#[test]
fn strip_prefix_trailing_slash() {
	let abs = "/home/user/project/src/main.rs";
	let project = "/home/user/project/";

	let result = strip_project_prefix(abs, project);

	assert_eq!(result, "src/main.rs");
}
