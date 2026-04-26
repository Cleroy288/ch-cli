//! Tests for IndexManager configuration methods.

use rustean::indexer::manager::IndexManager;

/// Test with_semantic_analysis enables both features
#[test]
fn test_with_semantic_analysis_enables_both() {
	// Arrange + Act
	let manager = IndexManager::new()
		.with_semantic_analysis();

	// Assert
	assert!(manager.flags().semantic_analysis);
	assert!(manager.flags().reference_extraction);
}

/// Test with_reference_extraction enables only refs
#[test]
fn test_with_reference_extraction_only_refs() {
	// Arrange + Act
	let manager = IndexManager::new()
		.with_reference_extraction();

	// Assert
	assert!(!manager.flags().semantic_analysis);
	assert!(manager.flags().reference_extraction);
}

/// Test with_persistence transitions and enables flag
#[test]
fn test_with_persistence_enables_flag() {
	// Arrange + Act
	let manager = IndexManager::new()
		.with_persistence();

	// Assert
	assert!(manager.flags().persistence);
}

/// Test builder chaining multiple options
#[test]
fn test_builder_chaining_all_flags() {
	// Arrange + Act
	let manager = IndexManager::new()
		.with_semantic_analysis()
		.with_persistence();

	// Assert
	assert!(manager.flags().semantic_analysis);
	assert!(manager.flags().reference_extraction);
	assert!(manager.flags().persistence);
}
