//! Tests for DetectedLanguage enum.

use std::path::Path;

use rustean::indexer::crawler::{
	DetectedLanguage, Language,
};

/// from_extension maps extensions to detected langs
#[test]
fn from_extension_table() {
	use DetectedLanguage::*;
	let cases: &[(
		&str,
		Option<DetectedLanguage>,
	)] = &[
		("rs", Some(Supported(Language::Rust))),
		("md", Some(Supported(Language::Markdown))),
		("js", Some(JavaScript)),
		("ts", Some(TypeScript)),
		("py", Some(Python)),
		("xyz", None),
	];

	for (ext, expected) in cases {
		assert_eq!(
			DetectedLanguage::from_extension(ext),
			*expected,
			"from_extension({ext:?})",
		);
	}
}

/// from_path maps file paths to detected langs
#[test]
fn from_path_table() {
	use DetectedLanguage::*;
	let cases: &[(
		&str,
		Option<DetectedLanguage>,
	)] = &[
		(
			"src/main.rs",
			Some(Supported(Language::Rust)),
		),
		("script.py", Some(Python)),
		("README", None),
	];

	for (path, expected) in cases {
		assert_eq!(
			DetectedLanguage::from_path(
				Path::new(path),
			),
			*expected,
			"from_path({path:?})",
		);
	}
}

/// display_name returns human-readable names
#[test]
fn display_name_table() {
	use DetectedLanguage::*;
	let cases: &[(DetectedLanguage, &str)] = &[
		(Supported(Language::Rust), "Rust"),
		(JavaScript, "JavaScript"),
	];

	for (lang, expected) in cases {
		assert_eq!(
			lang.display_name(),
			*expected,
			"display_name for {lang:?}",
		);
	}
}

/// is_supported distinguishes supported from not
#[test]
fn is_supported_table() {
	use DetectedLanguage::*;
	let cases: &[(DetectedLanguage, bool)] = &[
		(Supported(Language::Rust), true),
		(Python, false),
	];

	for (lang, expected) in cases {
		assert_eq!(
			lang.is_supported(),
			*expected,
			"is_supported for {lang:?}",
		);
	}
}

/// as_supported extracts the inner Language
#[test]
fn as_supported_table() {
	use DetectedLanguage::*;
	let cases: &[(
		DetectedLanguage,
		Option<Language>,
	)] = &[
		(
			Supported(Language::Rust),
			Some(Language::Rust),
		),
		(Python, None),
	];

	for (lang, expected) in cases {
		assert_eq!(
			lang.as_supported(),
			*expected,
			"as_supported for {lang:?}",
		);
	}
}
