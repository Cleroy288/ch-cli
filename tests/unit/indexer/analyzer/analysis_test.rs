//! Tests for CodebaseAnalysis.

use std::collections::HashMap;

use rustean::indexer::analyzer::CodebaseAnalysis;
use rustean::indexer::crawler::{DetectedLanguage, Language};

/// Helper to create an empty CodebaseAnalysis
fn make_empty_analysis() -> CodebaseAnalysis {
	CodebaseAnalysis {
		primary_language: None,
		language_counts: HashMap::new(),
		total_source_files: 0,
		is_primary_supported: false,
		project_type: None,
	}
}

/// Test has_supported_files returns false when no files exist
#[test]
fn test_has_supported_files_empty() {
	let analysis = make_empty_analysis();
	assert!(!analysis.has_supported_files());
}

/// Test has_supported_files returns true when supported files exist
#[test]
fn test_has_supported_files_with_rust() {
	let mut analysis = make_empty_analysis();
	let rust = DetectedLanguage::Supported(Language::Rust);
	analysis.language_counts.insert(rust, 5);

	assert!(analysis.has_supported_files());
}

/// Test has_supported_files false when only unsupported exist
#[test]
fn test_has_supported_files_only_unsupported() {
	let mut analysis = make_empty_analysis();
	analysis
		.language_counts
		.insert(DetectedLanguage::Python, 10);

	assert!(!analysis.has_supported_files());
}

/// Test supported_file_count returns 0 for empty analysis
#[test]
fn test_supported_file_count_empty() {
	let analysis = make_empty_analysis();
	assert_eq!(analysis.supported_file_count(), 0);
}

/// Test supported_file_count sums only supported languages
#[test]
fn test_supported_file_count_mixed() {
	let mut analysis = make_empty_analysis();
	let rust = DetectedLanguage::Supported(Language::Rust);
	let md = DetectedLanguage::Supported(Language::Markdown);
	analysis.language_counts.insert(rust, 10);
	analysis.language_counts.insert(md, 3);
	analysis
		.language_counts
		.insert(DetectedLanguage::Python, 20);

	let count = analysis.supported_file_count();
	assert_eq!(count, 13);
}
