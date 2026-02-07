//! Tests for retrieval::docgen::store_sync

use std::collections::HashMap;
use std::path::PathBuf;

use tempfile::tempdir;

use ch_cli::indexer::state::{FileState, IndexState};
use ch_cli::indexer::SymbolKind;
use ch_cli::retrieval::docgen::entry_types::DocStatus;
use ch_cli::retrieval::docgen::{DocEntry, DocStore};

#[test]
fn test_sync_with_index_marks_stale() {
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());

	let file_path = PathBuf::from("src/main.rs");
	let mut entry = DocEntry::new(
		"test_fn".to_string(),
		SymbolKind::Function,
		file_path.clone(),
		10,
	);
	entry.source_mtime = 100; // old mtime
	entry.mark_ready("Old doc".to_string());
	store.upsert(entry);

	assert!(store.is_ready());

	// newer mtime in index
	let mut files = HashMap::new();
	files.insert(
		file_path.clone(),
		FileState {
			path: file_path,
			mtime: 200,
			size: 500,
			symbol_count: 5,
		},
	);
	let index_state = IndexState {
		version: 1,
		root: dir.path().to_path_buf(),
		last_updated: 200,
		files,
		symbols: Vec::new(),
		references: Vec::new(),
	};

	store.sync_with_index(&index_state);

	let synced = store.get_by_name("test_fn").unwrap();
	assert_eq!(synced.status, DocStatus::Pending);
	assert!(synced.llm_doc.is_none());
	assert!(!store.is_ready());
}
