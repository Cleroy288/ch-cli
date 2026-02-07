//! Additional tests for ProjectType.

use ch_cli::indexer::analyzer::ProjectType;
use ch_cli::indexer::crawler::DetectedLanguage;

/// Test expected_language returns Java for Maven
#[test]
fn test_expected_language_maven() {
	let result = ProjectType::Maven.expected_language();
	assert_eq!(result, Some(DetectedLanguage::Java));
}

/// Test expected_language returns CSharp for DotNet
#[test]
fn test_expected_language_dotnet() {
	let result = ProjectType::DotNet.expected_language();
	assert_eq!(result, Some(DetectedLanguage::CSharp));
}

/// Test expected_language returns None for Unknown
#[test]
fn test_expected_language_unknown() {
	let result = ProjectType::Unknown.expected_language();
	assert_eq!(result, None);
}

/// Test display_name returns correct string for RustCargo
#[test]
fn test_display_name_rust() {
	let name = ProjectType::RustCargo.display_name();
	assert_eq!(name, "Rust (Cargo)");
}

/// Test display_name returns correct string for NodeJs
#[test]
fn test_display_name_nodejs() {
	let name = ProjectType::NodeJs.display_name();
	assert_eq!(name, "Node.js");
}
