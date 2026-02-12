//! Tests for per-file reference persistence.

use std::path::PathBuf;

use tempfile::tempdir;

use rustean::indexer::semantic::{
	ReferenceContext, SymbolReference,
};
use rustean::indexer::state::ref_persistence::merge_and_save_refs;
use rustean::indexer::state::{ChangeSet, IndexState};
use rustean::indexer::symbols::CodeLocation;

/// Helper: create a SymbolReference for testing.
fn make_ref(
	name: &str,
	file: PathBuf,
	line: usize,
) -> SymbolReference {
	SymbolReference {
		name: name.to_string(),
		location: CodeLocation {
			file,
			line,
			column: 1,
			byte_offset: 0,
			byte_length: name.len(),
		},
		context: ReferenceContext::Call,
	}
}

/// Collect ref names sorted for order-independent checks.
fn sorted_names(
	refs: &[SymbolReference],
) -> Vec<String> {
	let mut names: Vec<String> =
		refs.iter().map(|r| r.name.clone()).collect();
	names.sort();
	names
}

/// Save refs via merge, reload, verify roundtrip.
#[test]
fn merge_saves_and_loads_roundtrip() {
	// Arrange
	let tmp = tempdir().unwrap();
	let root = tmp.path().to_path_buf();
	let refs = vec![
		make_ref("foo", root.join("a.rs"), 1),
		make_ref("bar", root.join("b.rs"), 5),
	];

	// Act — full reindex (changes = None)
	let result =
		merge_and_save_refs(&root, &refs, &None).unwrap();
	let loaded =
		IndexState::load_references(&root).unwrap();

	// Assert
	assert_eq!(result.len(), 2);
	assert_eq!(loaded.len(), 2);
	assert_eq!(
		sorted_names(&loaded),
		vec!["bar", "foo"],
	);
}

/// Modified file's refs replaced, other file's kept.
#[test]
fn merge_removes_stale_file_refs() {
	// Arrange — initial refs from two files
	let tmp = tempdir().unwrap();
	let root = tmp.path().to_path_buf();
	let initial = vec![
		make_ref("old_a", root.join("a.rs"), 1),
		make_ref("keep_b", root.join("b.rs"), 3),
	];
	merge_and_save_refs(&root, &initial, &None).unwrap();

	// Act — a.rs modified, new ref replaces old
	let changes = ChangeSet {
		modified: vec![root.join("a.rs")],
		..Default::default()
	};
	let new_refs =
		vec![make_ref("new_a", root.join("a.rs"), 2)];
	let result = merge_and_save_refs(
		&root,
		&new_refs,
		&Some(changes),
	)
	.unwrap();

	// Assert — keep_b preserved, old_a replaced by new_a
	assert_eq!(result.len(), 2);
	let names = sorted_names(&result);
	assert!(names.contains(&"keep_b".to_string()));
	assert!(names.contains(&"new_a".to_string()));
}

/// Zero-change merge preserves all cached refs.
#[test]
fn merge_zero_changes_preserves_all() {
	// Arrange — save initial refs
	let tmp = tempdir().unwrap();
	let root = tmp.path().to_path_buf();
	let initial = vec![
		make_ref("alpha", root.join("x.rs"), 1),
		make_ref("beta", root.join("y.rs"), 2),
	];
	merge_and_save_refs(&root, &initial, &None).unwrap();

	// Act — zero changes, empty new refs
	let changes = ChangeSet::default();
	let result = merge_and_save_refs(
		&root,
		&[],
		&Some(changes),
	)
	.unwrap();

	// Assert — all preserved
	assert_eq!(result.len(), 2);
	assert_eq!(
		sorted_names(&result),
		vec!["alpha", "beta"],
	);
}

/// Each source file gets its own JSON in refs/.
#[test]
fn merge_creates_per_file_json() {
	// Arrange
	let tmp = tempdir().unwrap();
	let root = tmp.path().to_path_buf();
	let refs = vec![
		make_ref("foo", root.join("src/a.rs"), 1),
		make_ref("bar", root.join("src/b.rs"), 5),
	];

	// Act
	merge_and_save_refs(&root, &refs, &None).unwrap();

	// Assert — two separate files in refs/
	let refs_dir = IndexState::refs_dir(&root);
	let count =
		std::fs::read_dir(&refs_dir).unwrap().count();
	assert_eq!(count, 2);
}

/// Legacy refs.json migrated to refs/ directory.
#[test]
fn merge_migrates_legacy_refs_json() {
	// Arrange — write a legacy refs.json
	let tmp = tempdir().unwrap();
	let root = tmp.path().to_path_buf();
	let index_dir = IndexState::index_dir(&root);
	std::fs::create_dir_all(&index_dir).unwrap();

	let legacy =
		vec![make_ref("old", root.join("a.rs"), 1)];
	let json = serde_json::to_string(&legacy).unwrap();
	let refs_file = IndexState::refs_file(&root);
	std::fs::write(&refs_file, json).unwrap();

	// Act — merge triggers migration + adds new
	let new =
		vec![make_ref("new", root.join("b.rs"), 2)];
	let changes = ChangeSet::default();
	let result = merge_and_save_refs(
		&root,
		&new,
		&Some(changes),
	)
	.unwrap();

	// Assert — legacy file gone, refs/ exists, both refs
	assert!(!refs_file.exists());
	assert!(IndexState::refs_dir(&root).exists());
	assert_eq!(result.len(), 2);
	assert_eq!(
		sorted_names(&result),
		vec!["new", "old"],
	);
}
