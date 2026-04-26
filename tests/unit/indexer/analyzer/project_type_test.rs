//! Tests for ProjectType expected_language.

use rustean::indexer::analyzer::ProjectType;
use rustean::indexer::crawler::{
	DetectedLanguage, Language,
};

/// expected_language maps project types correctly
#[test]
fn expected_language_table() {
	let cases: &[(
		ProjectType,
		Option<DetectedLanguage>,
	)] = &[
		(
			ProjectType::RustCargo,
			Some(DetectedLanguage::Supported(
				Language::Rust,
			)),
		),
		(
			ProjectType::NodeJs,
			Some(DetectedLanguage::JavaScript),
		),
		(
			ProjectType::Python,
			Some(DetectedLanguage::Python),
		),
		(
			ProjectType::GoMod,
			Some(DetectedLanguage::GoLang),
		),
		(
			ProjectType::Gradle,
			Some(DetectedLanguage::Java),
		),
	];

	for (proj, expected) in cases {
		assert_eq!(
			proj.expected_language(),
			*expected,
			"expected_language for {proj:?}",
		);
	}
}
