//! Tests for Claude prompt building from segments.

use rustean::message::MessageSegment;
use rustean::service::claude::build_prompt;

/// Text-only segments produce literal text
#[test]
fn build_prompt_text_only() {
	// Arrange
	let segments = vec![
		MessageSegment::Text(
			"hello world".to_string(),
		),
	];

	// Act
	let prompt = build_prompt(&segments);

	// Assert
	assert_eq!(prompt, "hello world");
}

/// Multiple text segments are concatenated
#[test]
fn build_prompt_multiple_text_segments() {
	// Arrange
	let segments = vec![
		MessageSegment::Text("hello ".to_string()),
		MessageSegment::Text("world".to_string()),
	];

	// Act
	let prompt = build_prompt(&segments);

	// Assert
	assert_eq!(prompt, "hello world");
}

/// Symbol reference wraps source in XML tags
#[test]
fn build_prompt_symbol_reference() {
	// Arrange
	let segments = vec![
		MessageSegment::SymbolReference {
			full_path: "src/main.rs".to_string(),
			display_name: "main".to_string(),
			symbol_path: "main".to_string(),
			source_code: Some(
				"fn main() {}".to_string(),
			),
		},
	];

	// Act
	let prompt = build_prompt(&segments);

	// Assert
	assert!(prompt.contains("<code symbol=\"main\">"));
	assert!(prompt.contains("fn main() {}"));
	assert!(prompt.contains("</code>"));
}

/// Folder reference produces bracketed path
#[test]
fn build_prompt_folder_reference() {
	// Arrange
	let segments = vec![
		MessageSegment::FolderReference {
			full_path: "/src/app".to_string(),
			display_name: "app".to_string(),
		},
	];

	// Act
	let prompt = build_prompt(&segments);

	// Assert
	assert!(prompt.contains("[folder: /src/app]"));
}

/// Empty segments produce empty prompt
#[test]
fn build_prompt_empty_segments() {
	// Arrange
	let segments: Vec<MessageSegment> = vec![];

	// Act
	let prompt = build_prompt(&segments);

	// Assert
	assert_eq!(prompt, "");
}
