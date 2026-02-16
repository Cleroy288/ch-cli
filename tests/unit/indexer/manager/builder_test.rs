//! Tests for IndexManager builder.

use rustean::indexer::crawler::CrawlerConfig;
use rustean::indexer::manager::IndexManager;

/// Test IndexManager::new creates manager with default config
#[test]
fn test_new() {
	let manager = IndexManager::new();

	assert!(!manager.flags.semantic_analysis);
	assert!(!manager.flags.reference_extraction);
	assert!(!manager.flags.persistence);
	assert!(manager.progress_callback.is_none());
}

/// Test IndexManager::with_config uses custom crawler config
#[test]
fn test_with_config() {
	let config = CrawlerConfig::default();
	let manager = IndexManager::with_config(config);

	assert!(!manager.flags.semantic_analysis);
	assert!(!manager.flags.reference_extraction);
	assert!(!manager.flags.persistence);
}

/// Test on_progress sets progress callback
#[test]
fn test_on_progress() {
	let manager = IndexManager::new()
		.on_progress(|_, _, _| {});

	assert!(manager.progress_callback.is_some());
}

/// Test default implementation matches new
#[test]
fn test_default() {
	let default_manager = IndexManager::default(); // default manager
	let new_manager = IndexManager::new(); // new manager

	assert_eq!(
		default_manager.flags.semantic_analysis,
		new_manager.flags.semantic_analysis
	);
	assert_eq!(
		default_manager.flags.reference_extraction,
		new_manager.flags.reference_extraction
	);
	assert_eq!(
		default_manager.flags.persistence,
		new_manager.flags.persistence
	);
}
