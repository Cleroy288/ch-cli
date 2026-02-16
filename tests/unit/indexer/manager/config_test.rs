//! Tests for IndexManager configuration methods.

use rustean::indexer::manager::IndexManager;

/// Test with_semantic_analysis enables both features
#[test]
fn test_with_semantic_analysis() {
	let manager = IndexManager::new()
		.with_semantic_analysis();

	assert!(manager.flags.semantic_analysis);
	assert!(manager.flags.reference_extraction);
}

/// Test with_reference_extraction enables only references
#[test]
fn test_with_reference_extraction() {
	let manager = IndexManager::new()
		.with_reference_extraction();

	assert!(!manager.flags.semantic_analysis);
	assert!(manager.flags.reference_extraction);
}

/// Test with_persistence enables persistence
#[test]
fn test_with_persistence() {
	let manager = IndexManager::new()
		.with_persistence();

	assert!(manager.flags.persistence);
}

/// Test builder chaining multiple options
#[test]
fn test_builder_chaining() {
	let manager = IndexManager::new()
		.with_semantic_analysis()
		.with_persistence();

	assert!(manager.flags.semantic_analysis);
	assert!(manager.flags.reference_extraction);
	assert!(manager.flags.persistence);
}
