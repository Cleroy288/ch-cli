//! Tests for DocStore::get_by_file_and_name

use std::path::{Path, PathBuf};

use tempfile::tempdir;

use rustean::indexer::SymbolKind;
use rustean::retrieval::docgen::{DocEntry, DocStore};

/// Returns matching entry when file and name match.
#[test]
fn get_by_file_and_name_exact_match() {
	// Arrange
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());
	let path = PathBuf::from("src/calc.rs");
	let entry = DocEntry::new(
		"add".to_string(),
		SymbolKind::Function,
		path.clone(),
		10,
	);
	store.upsert(entry);

	// Act
	let result = store.get_by_file_and_name(
		&path, "add",
	);

	// Assert
	assert!(result.is_some());
	assert_eq!(result.unwrap().name, "add");
}

/// Returns None when name exists in a different file.
#[test]
fn get_by_file_and_name_wrong_file() {
	// Arrange
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());
	let entry = DocEntry::new(
		"new".to_string(),
		SymbolKind::Function,
		PathBuf::from("src/alpha.rs"),
		1,
	);
	store.upsert(entry);

	// Act
	let result = store.get_by_file_and_name(
		Path::new("src/beta.rs"),
		"new",
	);

	// Assert
	assert!(result.is_none());
}

/// Distinguishes same-named symbols in different files.
#[test]
fn get_by_file_and_name_disambiguates() {
	// Arrange
	let dir = tempdir().unwrap();
	let mut store = DocStore::new(dir.path());
	let path_a = PathBuf::from("src/alpha.rs");
	let path_b = PathBuf::from("src/beta.rs");

	let entry_a = DocEntry::new(
		"new".to_string(),
		SymbolKind::Function,
		path_a.clone(),
		5,
	);
	let entry_b = DocEntry::new(
		"new".to_string(),
		SymbolKind::Function,
		path_b.clone(),
		10,
	);
	store.upsert(entry_a);
	store.upsert(entry_b);

	// Act
	let result_a = store.get_by_file_and_name(
		&path_a, "new",
	);
	let result_b = store.get_by_file_and_name(
		&path_b, "new",
	);

	// Assert
	assert_eq!(result_a.unwrap().line, 5);
	assert_eq!(result_b.unwrap().line, 10);
}

/// Returns None when store is empty.
#[test]
fn get_by_file_and_name_empty_store() {
	// Arrange
	let dir = tempdir().unwrap();
	let store = DocStore::new(dir.path());

	// Act
	let result = store.get_by_file_and_name(
		Path::new("src/main.rs"),
		"foo",
	);

	// Assert
	assert!(result.is_none());
}
