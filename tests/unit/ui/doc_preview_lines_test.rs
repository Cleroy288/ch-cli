//! Tests for doc preview line rendering

use rustean::retrieval::daemon::protocol::DocEntryResponse;

/// Helper to create a DocEntryResponse for tests
fn make_doc_response(
	name: &str,
	signature: Option<&str>,
) -> DocEntryResponse {
	DocEntryResponse {
		name: name.to_string(),
		kind: "Function".to_string(),
		file_path: "src/calc.rs".to_string(),
		line: 42,
		user_comment: Some(
			"Adds two integers.".to_string(),
		),
		llm_doc: None,
		signature: signature.map(String::from),
		depends_on: vec![],
		depended_by: vec![
			"Calculator::compute".to_string(),
		],
		external_deps: vec![],
		status: "Ready".to_string(),
	}
}

/// Renders non-empty lines for a valid doc entry.
#[test]
fn build_lines_with_doc_has_content() {
	use rustean::ui::components::debug_render
		::build_doc_preview_lines;

	// Arrange
	let doc = make_doc_response(
		"add",
		Some("fn add(a: i32, b: i32) -> i32"),
	);

	// Act
	let lines = build_doc_preview_lines(Some(&doc));

	// Assert — header, location, body, deps
	assert!(lines.len() >= 6);
}

/// Returns placeholder when doc is None.
#[test]
fn build_lines_without_doc_shows_placeholder() {
	use rustean::ui::components::debug_render
		::build_doc_preview_lines;

	// Act
	let lines = build_doc_preview_lines(None);

	// Assert
	assert_eq!(lines.len(), 2);
}

/// Falls back to kind+name when no signature.
#[test]
fn build_lines_no_signature_shows_kind_name() {
	use rustean::ui::components::debug_render
		::build_doc_preview_lines;

	// Arrange
	let doc = make_doc_response("add", None);

	// Act
	let lines = build_doc_preview_lines(Some(&doc));

	// Assert — should still render lines
	assert!(lines.len() >= 6);
}

/// Doc with no user_comment or llm_doc produces
/// fewer body lines.
#[test]
fn build_lines_no_body_text() {
	use rustean::ui::components::debug_render
		::build_doc_preview_lines;

	// Arrange
	let mut doc = make_doc_response("add", None);
	doc.user_comment = None;
	doc.llm_doc = None;

	// Act
	let lines = build_doc_preview_lines(Some(&doc));

	// Assert — header+location+deps, but no body
	assert!(lines.len() >= 5);
}
