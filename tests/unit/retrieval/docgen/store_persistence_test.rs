//! Tests for retrieval::docgen::store_persistence

use std::path::PathBuf;

use tempfile::tempdir;

use rustean::indexer::SymbolKind;
use rustean::retrieval::docgen::{DocEntry, DocStore};

#[test]
fn test_store_persistence() {
	let dir = tempdir().unwrap();

	{
		let mut store = DocStore::new(dir.path());
		let mut entry = DocEntry::new(
			"test".to_string(),
			SymbolKind::Function,
			PathBuf::from("src/main.rs"),
			10,
		);
		entry.mark_ready("Generated doc".to_string());
		store.upsert(entry);
		store.save().unwrap();
	}

	{
		let store =
			DocStore::load(dir.path()).unwrap();
		assert_eq!(store.len(), 1);
		let entry =
			store.get_by_name("test").unwrap();
		assert_eq!(
			entry.llm_doc,
			Some("Generated doc".to_string())
		);
		assert!(store.is_ready());
	}
}

#[test]
fn test_store_stats() {
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());

	let entry1 = DocEntry::new(
		"fn1".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/main.rs"),
		10,
	);
	let mut entry2 = DocEntry::new(
		"fn2".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/main.rs"),
		20,
	);
	entry2.mark_ready("doc".to_string());

	store.upsert(entry1);
	store.upsert(entry2);

	let stats = store.stats();
	assert_eq!(stats.total, 2);
	assert_eq!(stats.ready, 1);
	assert_eq!(stats.pending, 1);
	assert!(!stats.is_complete);
}
