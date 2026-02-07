//! Tests for IndexManager builder.

use ch_cli::indexer::crawler::CrawlerConfig;
use ch_cli::indexer::manager::IndexManager;

/// Test IndexManager::new creates manager with default config
#[test]
fn test_new() {
	let manager = IndexManager::new();

	assert!(!manager.enable_semantic_analysis);
	assert!(!manager.enable_reference_extraction);
	assert!(!manager.enable_persistence);
	assert!(manager.progress_callback.is_none());
}

/// Test IndexManager::with_config uses custom crawler config
#[test]
fn test_with_config() {
	let config = CrawlerConfig::default();
	let manager = IndexManager::with_config(config);

	assert!(!manager.enable_semantic_analysis);
	assert!(!manager.enable_reference_extraction);
	assert!(!manager.enable_persistence);
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
		default_manager.enable_semantic_analysis,
		new_manager.enable_semantic_analysis
	);
	assert_eq!(
		default_manager.enable_reference_extraction,
		new_manager.enable_reference_extraction
	);
	assert_eq!(
		default_manager.enable_persistence,
		new_manager.enable_persistence
	);
}
