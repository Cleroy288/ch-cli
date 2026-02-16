//! Tests for retrieval::docgen::linker_analysis

use rustean::indexer::semantic::ReferenceContext;
use rustean::retrieval::docgen::entry_types::ReferenceKind;
use rustean::retrieval::docgen::linker_analysis::{
	convert_reference_context, is_std_module,
};

#[test]
fn test_is_std_module() {
	assert!(is_std_module("std"));
	assert!(is_std_module("core"));
	assert!(is_std_module("self"));
	assert!(!is_std_module("serde"));
	assert!(!is_std_module("tokio"));
}

#[test]
fn test_convert_reference_context() {
	assert!(matches!(
		convert_reference_context(ReferenceContext::Call),
		ReferenceKind::Call
	));
	assert!(matches!(
		convert_reference_context(ReferenceContext::Type),
		ReferenceKind::TypeUsage
	));
	assert!(matches!(
		convert_reference_context(
			ReferenceContext::Import
		),
		ReferenceKind::Import
	));
}
