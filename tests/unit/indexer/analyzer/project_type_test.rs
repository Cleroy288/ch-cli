//! Tests for ProjectType expected_language.

use rustean::indexer::analyzer::ProjectType;
use rustean::indexer::crawler::{DetectedLanguage, Language};

/// Test expected_language returns Rust for RustCargo
#[test]
fn test_expected_language_rust() {
	let result = ProjectType::RustCargo.expected_language();

	let expected = DetectedLanguage::Supported(Language::Rust);
	assert_eq!(result, Some(expected));
}

/// Test expected_language returns JavaScript for NodeJs
#[test]
fn test_expected_language_nodejs() {
	let result = ProjectType::NodeJs.expected_language();
	assert_eq!(result, Some(DetectedLanguage::JavaScript));
}

/// Test expected_language returns Python for Python
#[test]
fn test_expected_language_python() {
	let result = ProjectType::Python.expected_language();
	assert_eq!(result, Some(DetectedLanguage::Python));
}

/// Test expected_language returns Go for GoMod
#[test]
fn test_expected_language_go() {
	let result = ProjectType::GoMod.expected_language();
	assert_eq!(result, Some(DetectedLanguage::GoLang));
}

/// Test expected_language returns Java for Gradle
#[test]
fn test_expected_language_gradle() {
	let result = ProjectType::Gradle.expected_language();
	assert_eq!(result, Some(DetectedLanguage::Java));
}
