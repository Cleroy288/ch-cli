//! Additional tests for ProjectType.

use rustean::indexer::analyzer::ProjectType;
use rustean::indexer::crawler::DetectedLanguage;

/// expected_language for remaining project types
#[test]
fn expected_language_extra_table() {
	use DetectedLanguage::*;
	use ProjectType::*;
	let cases: &[(
		ProjectType,
		Option<DetectedLanguage>,
	)] = &[
		(Maven, Some(Java)),
		(DotNet, Some(CSharp)),
		(Unknown, None),
	];

	for (proj, expected) in cases {
		assert_eq!(
			proj.expected_language(),
			*expected,
			"expected_language for {proj:?}",
		);
	}
}

/// display_name returns human-readable labels
#[test]
fn display_name_table() {
	let cases: &[(ProjectType, &str)] = &[
		(ProjectType::RustCargo, "Rust (Cargo)"),
		(ProjectType::NodeJs, "Node.js"),
	];

	for (proj, expected) in cases {
		assert_eq!(
			proj.display_name(),
			*expected,
			"display_name for {proj:?}",
		);
	}
}
