//! Tests for Language enum.

use std::path::Path;

use rustean::indexer::crawler::Language;

/// Test extensions returns "rs" for Rust
#[test]
fn test_extensions_rust() {
	// extensions for Rust language
	let exts = Language::Rust.extensions();

	assert_eq!(exts, &["rs"]);
}

/// Test extensions returns "md" and "txt" for Markdown
#[test]
fn test_extensions_markdown() {
	// extensions for Markdown language
	let exts = Language::Markdown.extensions();

	assert_eq!(exts, &["md", "txt"]);
}

/// Test from_extension detects Rust from "rs"
#[test]
fn test_from_extension_rs() {
	// detect language from "rs"
	let result = Language::from_extension("rs");

	assert_eq!(result, Some(Language::Rust));
}

/// Test from_extension detects Markdown from "md"
#[test]
fn test_from_extension_md() {
	// detect language from "md"
	let result = Language::from_extension("md");

	assert_eq!(result, Some(Language::Markdown));
}

/// Test from_extension detects Python as None (unsupported)
#[test]
fn test_from_extension_py() {
	// detect language from "py"
	let result = Language::from_extension("py");

	assert_eq!(result, None);
}

/// Test from_extension returns None for unknown extension
#[test]
fn test_from_extension_unknown() {
	// detect language from unknown ext
	let result = Language::from_extension("xyz");

	assert_eq!(result, None);
}

/// Test display_name returns "Rust" for Rust
#[test]
fn test_display_name_rust() {
	// display name for Rust
	let name = Language::Rust.display_name();

	assert_eq!(name, "Rust");
}

/// Test display_name returns "Markdown" for Markdown
#[test]
fn test_display_name_markdown() {
	// display name for Markdown
	let name = Language::Markdown.display_name();

	assert_eq!(name, "Markdown");
}

/// Test from_path detects Rust from .rs file
#[test]
fn test_from_path_rust() {
	// Rust source file
	let path = Path::new("src/main.rs");
	// detect language from path
	let result = Language::from_path(path);

	assert_eq!(result, Some(Language::Rust));
}

/// Test from_path detects Markdown from .md file
#[test]
fn test_from_path_markdown() {
	// Markdown file
	let path = Path::new("README.md");
	// detect language from path
	let result = Language::from_path(path);

	assert_eq!(result, Some(Language::Markdown));
}

/// Test from_path returns None for unsupported extension
#[test]
fn test_from_path_unsupported() {
	// Python file (unsupported)
	let path = Path::new("script.py");
	// detect language from path
	let result = Language::from_path(path);

	assert_eq!(result, None);
}

/// Test from_path returns None for path without extension
#[test]
fn test_from_path_no_extension() {
	// file without extension
	let path = Path::new("Makefile");
	// detect language from path
	let result = Language::from_path(path);

	assert_eq!(result, None);
}

/// Test all_supported returns all supported languages
#[test]
fn test_all_supported() {
	// get all supported languages
	let langs = Language::all_supported();

	assert_eq!(langs, &[Language::Rust, Language::Markdown]);
	assert_eq!(langs.len(), 2);
}
