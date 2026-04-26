//! Tests for Language enum.

use std::path::Path;

use rustean::indexer::crawler::Language;

/// from_extension maps known extensions to languages
#[test]
fn from_extension_table() {
	let cases: &[(&str, Option<Language>)] = &[
		("rs", Some(Language::Rust)),
		("md", Some(Language::Markdown)),
		("py", None),
		("xyz", None),
	];

	for (ext, expected) in cases {
		assert_eq!(
			Language::from_extension(ext),
			*expected,
			"from_extension({ext:?})",
		);
	}
}

/// from_path maps file paths to languages
#[test]
fn from_path_table() {
	let cases: &[(&str, Option<Language>)] = &[
		("src/main.rs", Some(Language::Rust)),
		("README.md", Some(Language::Markdown)),
		("script.py", None),
		("Makefile", None),
	];

	for (path, expected) in cases {
		assert_eq!(
			Language::from_path(Path::new(path)),
			*expected,
			"from_path({path:?})",
		);
	}
}

/// extensions returns the right set per language
#[test]
fn extensions_per_language() {
	assert_eq!(
		Language::Rust.extensions(),
		&["rs"],
	);
	assert_eq!(
		Language::Markdown.extensions(),
		&["md", "txt"],
	);
}

/// display_name returns the human-readable name
#[test]
fn display_name_per_language() {
	assert_eq!(Language::Rust.display_name(), "Rust");
	assert_eq!(
		Language::Markdown.display_name(),
		"Markdown",
	);
}

/// all_supported lists every supported variant
#[test]
fn all_supported_returns_all() {
	let langs = Language::all_supported();

	assert_eq!(
		langs,
		&[Language::Rust, Language::Markdown],
	);
}
