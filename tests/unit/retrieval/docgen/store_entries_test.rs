//! Tests for retrieval::docgen::store_entries

use std::path::PathBuf;

use tempfile::tempdir;

use rustean::indexer::SymbolKind;
use rustean::retrieval::docgen::{DocEntry, DocStore};

#[test]
fn test_store_upsert_and_get() {
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());

	let entry = DocEntry::new(
		"test_fn".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/main.rs"),
		10,
	);
	let id = entry.id.clone();

	store.upsert(entry);

	assert_eq!(store.len(), 1);
	assert!(store.get(&id).is_some());
	assert_eq!(
		store.get(&id).unwrap().name,
		"test_fn"
	);
}

/// Verify clear removes all entries and resets status.
#[test]
fn test_store_clear() {
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());

	let entry1 = DocEntry::new(
		"fn_a".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/a.rs"),
		1,
	);
	let entry2 = DocEntry::new(
		"fn_b".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/b.rs"),
		1,
	);

	store.upsert(entry1);
	store.upsert(entry2);
	assert_eq!(store.len(), 2);

	store.clear();

	assert!(store.is_empty());
	assert_eq!(store.len(), 0);
	assert!(!store.is_ready());
}

/// Verify remove deletes entry by ID and returns it.
#[test]
fn test_store_remove() {
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());

	let entry = DocEntry::new(
		"fn_remove".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/main.rs"),
		5,
	);
	let id = entry.id.clone();

	store.upsert(entry);
	assert_eq!(store.len(), 1);

	let removed = store.remove(&id);

	assert!(removed.is_some());
	assert_eq!(removed.unwrap().name, "fn_remove");
	assert!(store.is_empty());
	assert!(store.get(&id).is_none());
}

/// Verify all_entries_mut allows modifying entries.
#[test]
fn test_store_all_entries_mut() {
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());

	let entry1 = DocEntry::new(
		"fn_x".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/main.rs"),
		1,
	);
	let entry2 = DocEntry::new(
		"fn_y".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/main.rs"),
		10,
	);

	store.upsert(entry1);
	store.upsert(entry2);

	for entry in store.all_entries_mut() {
		entry.mark_ready("batch doc".to_string());
	}

	let stats = store.stats();
	assert_eq!(stats.ready, 2);
	assert_eq!(stats.pending, 0);
}
