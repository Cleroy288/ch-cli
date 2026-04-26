//! Tests for the markdown section parser.

use rustean::ui::markdown::sections::{
	parse_sections, ContentSection,
};

/// Plain text returns a single Text section
#[test]
fn text_only_returns_single_text_section() {
	// Arrange / Act
	let result = parse_sections("hello world");

	// Assert
	assert_eq!(result.len(), 1);
	assert_eq!(
		result[0],
		ContentSection::Text(
			"hello world".to_string(),
		),
	);
}

/// Code block splits into three sections
#[test]
fn code_block_splits_into_three_sections() {
	// Arrange
	let input =
		"text\n```rust\nfn x(){}\n```\nmore";

	// Act
	let result = parse_sections(input);

	// Assert
	assert_eq!(result.len(), 3);
	assert_eq!(
		result[0],
		ContentSection::Text(
			"text".to_string(),
		),
	);
	assert_eq!(
		result[1],
		ContentSection::Code {
			lang: "rust".to_string(),
			code: "fn x(){}".to_string(),
		},
	);
	assert_eq!(
		result[2],
		ContentSection::Text(
			"more".to_string(),
		),
	);
}

/// Unclosed fence becomes code section (streaming)
#[test]
fn unclosed_fence_becomes_code_section() {
	// Arrange
	let input = "text\n```rust\nfn x(){}";

	// Act
	let result = parse_sections(input);

	// Assert
	assert_eq!(result.len(), 2);
	assert_eq!(
		result[0],
		ContentSection::Text(
			"text".to_string(),
		),
	);
	assert_eq!(
		result[1],
		ContentSection::Code {
			lang: "rust".to_string(),
			code: "fn x(){}".to_string(),
		},
	);
}

/// Bare ``` fence returns empty language string
#[test]
fn empty_lang_tag_returns_empty_string() {
	// Arrange / Act
	let result =
		parse_sections("```\ncode\n```");

	// Assert
	assert_eq!(result.len(), 1);
	assert_eq!(
		result[0],
		ContentSection::Code {
			lang: String::new(),
			code: "code".to_string(),
		},
	);
}

/// Consecutive code blocks are both parsed
#[test]
fn consecutive_code_blocks_parsed() {
	// Arrange
	let input =
		"```go\na\n```\n```py\nb\n```";

	// Act
	let result = parse_sections(input);

	// Assert
	assert_eq!(result.len(), 2);
	assert_eq!(
		result[0],
		ContentSection::Code {
			lang: "go".to_string(),
			code: "a".to_string(),
		},
	);
	assert_eq!(
		result[1],
		ContentSection::Code {
			lang: "py".to_string(),
			code: "b".to_string(),
		},
	);
}

/// No fences returns a single text section
#[test]
fn no_fences_returns_single_text() {
	// Arrange / Act
	let result =
		parse_sections("# Heading\n- item");

	// Assert
	assert_eq!(result.len(), 1);
	assert_eq!(
		result[0],
		ContentSection::Text(
			"# Heading\n- item".to_string(),
		),
	);
}
