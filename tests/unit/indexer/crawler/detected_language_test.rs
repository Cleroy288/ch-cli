//! Tests for DetectedLanguage enum.

use std::path::Path;

use ch_cli::indexer::crawler::{DetectedLanguage, Language};

/// Test from_extension detects Rust as supported language
#[test]
fn test_from_extension_rust() {
	// detect Rust from "rs"
	let result = DetectedLanguage::from_extension("rs");

	let expected = Some(DetectedLanguage::Supported(Language::Rust));
	assert_eq!(result, expected);
}

/// Test from_extension detects Markdown as supported language
#[test]
fn test_from_extension_markdown() {
	// detect Markdown from "md"
	let result = DetectedLanguage::from_extension("md");

	let expected = Some(DetectedLanguage::Supported(Language::Markdown));
	assert_eq!(result, expected);
}

/// Test from_extension detects JavaScript (unsupported)
#[test]
fn test_from_extension_javascript() {
	// detect JavaScript from "js"
	let result = DetectedLanguage::from_extension("js");

	assert_eq!(result, Some(DetectedLanguage::JavaScript));
}

/// Test from_extension detects TypeScript (unsupported)
#[test]
fn test_from_extension_typescript() {
	// detect TypeScript from "ts"
	let result = DetectedLanguage::from_extension("ts");

	assert_eq!(result, Some(DetectedLanguage::TypeScript));
}

/// Test from_extension detects Python (unsupported)
#[test]
fn test_from_extension_python() {
	// detect Python from "py"
	let result = DetectedLanguage::from_extension("py");

	assert_eq!(result, Some(DetectedLanguage::Python));
}

/// Test from_extension returns None for unknown extension
#[test]
fn test_from_extension_unknown() {
	// detect unknown extension
	let result = DetectedLanguage::from_extension("xyz");

	assert_eq!(result, None);
}

/// Test from_path detects Rust from file path
#[test]
fn test_from_path_rust() {
	// Rust file path
	let path = Path::new("src/main.rs");
	// detect language from path
	let result = DetectedLanguage::from_path(path);

	let expected = Some(DetectedLanguage::Supported(Language::Rust));
	assert_eq!(result, expected);
}

/// Test from_path detects Python from file path
#[test]
fn test_from_path_python() {
	// Python file path
	let path = Path::new("script.py");
	// detect language from path
	let result = DetectedLanguage::from_path(path);

	assert_eq!(result, Some(DetectedLanguage::Python));
}

/// Test from_path returns None for path without extension
#[test]
fn test_from_path_no_extension() {
	// path without extension
	let path = Path::new("README");
	// detect language from path
	let result = DetectedLanguage::from_path(path);

	assert_eq!(result, None);
}

/// Test display_name for supported Rust language
#[test]
fn test_display_name_rust() {
	// supported Rust
	let lang = DetectedLanguage::Supported(Language::Rust);
	// get display name
	let name = lang.display_name();

	assert_eq!(name, "Rust");
}

/// Test display_name for unsupported JavaScript
#[test]
fn test_display_name_javascript() {
	// unsupported JavaScript
	let lang = DetectedLanguage::JavaScript;
	// get display name
	let name = lang.display_name();

	assert_eq!(name, "JavaScript");
}

/// Test is_supported returns true for supported language
#[test]
fn test_is_supported_rust() {
	// supported Rust
	let lang = DetectedLanguage::Supported(Language::Rust);

	assert!(lang.is_supported());
}

/// Test is_supported returns false for unsupported language
#[test]
fn test_is_supported_python() {
	// unsupported Python
	let lang = DetectedLanguage::Python;

	assert!(!lang.is_supported());
}

/// Test as_supported returns Some for supported language
#[test]
fn test_as_supported_rust() {
	// supported Rust
	let lang = DetectedLanguage::Supported(Language::Rust);
	// extract supported language
	let result = lang.as_supported();

	assert_eq!(result, Some(Language::Rust));
}

/// Test as_supported returns None for unsupported language
#[test]
fn test_as_supported_python() {
	// unsupported Python
	let lang = DetectedLanguage::Python;
	// extract supported language
	let result = lang.as_supported();

	assert_eq!(result, None);
}
