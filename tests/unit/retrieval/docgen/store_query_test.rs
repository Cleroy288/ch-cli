//! Tests for retrieval::docgen::store_query

use std::path::{Path, PathBuf};

use tempfile::tempdir;

use rustean::indexer::SymbolKind;
use rustean::retrieval::docgen::{DocEntry, DocStore};

#[test]
fn test_store_pending() {
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

	let pending = store.get_pending();
	assert_eq!(pending.len(), 1);
	assert_eq!(pending[0].name, "fn1");
}

/// Verify get_by_file returns matching entries.
#[test]
fn test_store_get_by_file() {
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());

	let path_a = PathBuf::from("src/alpha.rs");
	let path_b = PathBuf::from("src/beta.rs");

	let entry1 = DocEntry::new(
		"fn_one".to_string(),
		SymbolKind::Function,
		path_a.clone(),
		1,
	);
	let entry2 = DocEntry::new(
		"fn_two".to_string(),
		SymbolKind::Function,
		path_a.clone(),
		20,
	);
	let entry3 = DocEntry::new(
		"fn_three".to_string(),
		SymbolKind::Function,
		path_b.clone(),
		1,
	);

	store.upsert(entry1);
	store.upsert(entry2);
	store.upsert(entry3);

	let alpha = store.get_by_file(&path_a);
	let beta = store.get_by_file(&path_b);
	let none = store.get_by_file(Path::new("src/none.rs"));

	assert_eq!(alpha.len(), 2);
	assert_eq!(beta.len(), 1);
	assert_eq!(beta[0].name, "fn_three");
	assert!(none.is_empty());
}

/// Verify search_by_name is case-insensitive.
#[test]
fn test_store_search_by_name() {
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());

	let entry1 = DocEntry::new(
		"process_data".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/main.rs"),
		1,
	);
	let entry2 = DocEntry::new(
		"ProcessResult".to_string(),
		SymbolKind::Struct,
		PathBuf::from("src/main.rs"),
		10,
	);
	let entry3 = DocEntry::new(
		"unrelated".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/main.rs"),
		20,
	);

	store.upsert(entry1);
	store.upsert(entry2);
	store.upsert(entry3);

	let results = store.search_by_name("process");
	assert_eq!(results.len(), 2);

	let no_results = store.search_by_name("zzz_missing");
	assert!(no_results.is_empty());
}

/// Verify get_pending_ids returns only Pending IDs.
#[test]
fn test_store_get_pending_ids() {
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());

	let entry_pending = DocEntry::new(
		"pending_fn".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/main.rs"),
		1,
	);
	let pending_id = entry_pending.id.clone();

	let mut entry_ready = DocEntry::new(
		"ready_fn".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/main.rs"),
		10,
	);
	entry_ready.mark_ready("doc".to_string());

	let mut entry_failed = DocEntry::new(
		"failed_fn".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/main.rs"),
		20,
	);
	entry_failed.mark_failed();

	store.upsert(entry_pending);
	store.upsert(entry_ready);
	store.upsert(entry_failed);

	let pending_ids = store.get_pending_ids();

	assert_eq!(pending_ids.len(), 1);
	assert_eq!(pending_ids[0], pending_id);
}
