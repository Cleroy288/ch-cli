//! Tests for IndexManager builder.

use rustean::indexer::crawler::CrawlerConfig;
use rustean::indexer::manager::IndexManager;

/// Test IndexManager::new creates manager with default config
#[test]
fn test_new_default_flags_all_false() {
	// Arrange + Act
	let manager = IndexManager::new();

	// Assert
	assert!(!manager.flags().semantic_analysis);
	assert!(!manager.flags().reference_extraction);
	assert!(!manager.flags().persistence);
	assert!(!manager.has_progress_callback());
}

/// Test IndexManager::with_config uses custom crawler config
#[test]
fn test_with_config_default_flags_all_false() {
	// Arrange
	let config = CrawlerConfig::default();

	// Act
	let manager = IndexManager::with_config(config);

	// Assert
	assert!(!manager.flags().semantic_analysis);
	assert!(!manager.flags().reference_extraction);
	assert!(!manager.flags().persistence);
}

/// Test on_progress sets progress callback
#[test]
fn test_on_progress_sets_callback() {
	// Arrange + Act
	let manager = IndexManager::new()
		.on_progress(|_, _, _| {});

	// Assert
	assert!(manager.has_progress_callback());
}

/// Test default implementation matches new
#[test]
fn test_default_matches_new() {
	// Arrange
	let default_mgr = IndexManager::default();
	let new_mgr = IndexManager::new();

	// Assert
	assert_eq!(
		default_mgr.flags().semantic_analysis,
		new_mgr.flags().semantic_analysis
	);
	assert_eq!(
		default_mgr.flags().reference_extraction,
		new_mgr.flags().reference_extraction
	);
	assert_eq!(
		default_mgr.flags().persistence,
		new_mgr.flags().persistence
	);
}
