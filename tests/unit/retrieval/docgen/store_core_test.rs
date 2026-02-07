//! Tests for retrieval::docgen::store_core

use tempfile::tempdir;

use ch_cli::retrieval::docgen::{DocStore, DocStoreStats};

#[test]
fn test_store_new() {
	let dir = tempdir().unwrap();
	let store = DocStore::new(dir.path());

	assert!(store.is_empty());
	assert!(!store.is_ready());
}

/// Verify completion_percent for 0%, 50%, and 100%.
#[test]
fn test_completion_percent() {
	let stats_zero = DocStoreStats {
		total: 4,
		ready: 0,
		pending: 4,
		generating: 0,
		failed: 0,
		is_complete: false,
	};
	assert!(
		(stats_zero.completion_percent() - 0.0).abs()
			< f64::EPSILON
	);

	let stats_half = DocStoreStats {
		total: 4,
		ready: 2,
		pending: 2,
		generating: 0,
		failed: 0,
		is_complete: false,
	};
	assert!(
		(stats_half.completion_percent() - 50.0).abs()
			< f64::EPSILON
	);

	let stats_full = DocStoreStats {
		total: 4,
		ready: 4,
		pending: 0,
		generating: 0,
		failed: 0,
		is_complete: true,
	};
	assert!(
		(stats_full.completion_percent() - 100.0).abs()
			< f64::EPSILON
	);

	let stats_empty = DocStoreStats {
		total: 0,
		ready: 0,
		pending: 0,
		generating: 0,
		failed: 0,
		is_complete: false,
	};
	assert!(
		(stats_empty.completion_percent() - 100.0).abs()
			< f64::EPSILON
	);
}
