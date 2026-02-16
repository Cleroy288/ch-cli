use std::path::PathBuf;

use rustean::retrieval::hybrid::vector_store::{
	VectorPoint, VectorStore,
};

fn create_point(
	id: u64,
	vector: Vec<f32>,
) -> VectorPoint {
	VectorPoint {
		id,
		vector,
		file_path: PathBuf::from("test.rs"),
		line: 1,
		symbol_name: format!("symbol_{}", id),
		symbol_kind: "function".to_string(),
	}
}

#[test]
fn test_insert_and_search() {
	let mut store = VectorStore::new();

	// insert points
	store.insert(create_point(0, vec![1.0, 0.0, 0.0]));
	store.insert(create_point(1, vec![0.0, 1.0, 0.0]));
	store.insert(create_point(2, vec![0.0, 0.0, 1.0]));

	// build index
	store.build_index().unwrap();

	// search
	let results = store.search(&[1.0, 0.0, 0.0], 2);
	assert_eq!(results.len(), 2);
	assert_eq!(results[0].point.id, 0);
}

#[test]
fn test_empty_store() {
	let store = VectorStore::new();
	let results = store.search(&[1.0, 0.0, 0.0], 5);
	assert!(results.is_empty());
}

/// Test len returns correct count after inserts
#[test]
fn test_len() {
	let mut store = VectorStore::new();

	assert_eq!(store.len(), 0);

	store.insert(create_point(0, vec![1.0, 0.0, 0.0]));
	store.insert(create_point(1, vec![0.0, 1.0, 0.0]));

	assert_eq!(store.len(), 2);
}

/// Test is_indexed: false before build, true after
#[test]
fn test_is_indexed() {
	let mut store = VectorStore::new();

	assert!(!store.is_indexed());

	store.insert(create_point(0, vec![1.0, 0.0, 0.0]));
	assert!(!store.is_indexed());

	store.build_index().unwrap();
	assert!(store.is_indexed());
}

/// Test clear removes all points and invalidates index
#[test]
fn test_clear() {
	let mut store = VectorStore::new();

	store.insert(create_point(0, vec![1.0, 0.0, 0.0]));
	store.insert(create_point(1, vec![0.0, 1.0, 0.0]));
	store.build_index().unwrap();

	assert_eq!(store.len(), 2);
	assert!(store.is_indexed());

	store.clear();

	assert_eq!(store.len(), 0);
	assert!(store.is_empty());
	assert!(!store.is_indexed());
}
