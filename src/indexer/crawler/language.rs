//! Supported programming languages for indexing.
//!
//! This module defines the Language enum for languages that can be indexed.

use std::path::Path;

/// Supported programming languages for indexing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
	Rust,
	/// Markdown documentation files
	Markdown,
	// Future: JavaScript, TypeScript, Python, Go, etc.
}

impl Language {
	/// Get file extensions for this language
	pub fn extensions(&self) -> &[&str] {
		match self {
			Language::Rust => &["rs"],
			Language::Markdown => &["md", "txt"],
		}
	}

	/// Detect language from file extension
	pub fn from_extension(ext: &str) -> Option<Language> {
		match ext.to_lowercase().as_str() {
			"rs" => Some(Language::Rust),
			"md" | "txt" => Some(Language::Markdown),
			_ => None,
		}
	}

	/// Detect language from file path
	pub fn from_path(path: &Path) -> Option<Language> {
		path.extension()
			.and_then(|ext| ext.to_str())
			.and_then(Language::from_extension)
	}

	/// Get the display name for this language
	pub fn display_name(&self) -> &'static str {
		match self {
			Language::Rust => "Rust",
			Language::Markdown => "Markdown",
		}
	}

	/// Get all supported languages for indexing
	pub fn all_supported() -> &'static [Language] {
		&[Language::Rust, Language::Markdown]
	}
}
