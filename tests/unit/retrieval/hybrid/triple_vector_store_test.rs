use std::path::PathBuf;

use ch_cli::indexer::ContentType;
use ch_cli::retrieval::hybrid::triple_vector_store::TripleVectorStore;
use ch_cli::retrieval::hybrid::vector_store::VectorPoint;

/// Create a test vector point
fn create_test_point(
	id: u64,
	file_path: &str,
) -> VectorPoint {
	VectorPoint {
		id,
		vector: vec![1.0, 0.0, 0.0],
		file_path: PathBuf::from(file_path),
		line: 1,
		symbol_name: format!("symbol_{}", id),
		symbol_kind: "function".to_string(),
	}
}

/// Test TripleVectorStore routes points correctly
#[test]
fn test_triple_vector_store_routes_points() {
	let mut store = TripleVectorStore::new();

	store.insert(
		create_test_point(0, "/src/main.rs"),
		ContentType::Code,
	);
	store.insert(
		create_test_point(1, "/tests/test.rs"),
		ContentType::Code,
	);
	store.insert(
		create_test_point(2, "/doc/api.md"),
		ContentType::Doc,
	);
	store.insert(
		create_test_point(3, "/notes/impl.md"),
		ContentType::Notes,
	);
	store.insert(
		create_test_point(4, "/notes/bench.md"),
		ContentType::Notes,
	);

	let stats = store.stats();

	assert_eq!(stats.code_count, 2);
	assert_eq!(stats.doc_count, 1);
	assert_eq!(stats.notes_count, 2);
	assert_eq!(stats.total(), 5);
}

/// Test TripleVectorStore search returns correct store
#[test]
fn test_triple_vector_store_search_code() {
	let mut store = TripleVectorStore::new();

	store.insert(
		create_test_point(0, "/src/main.rs"),
		ContentType::Code,
	);
	store.insert(
		create_test_point(1, "/doc/api.md"),
		ContentType::Doc,
	);

	store.build_indexes().unwrap();

	let results =
		store.search_code(&[1.0, 0.0, 0.0], 10);
	assert_eq!(results.len(), 1);
	assert_eq!(results[0].point.id, 0);
}

/// Test TripleVectorStore parallel search
#[test]
fn test_triple_vector_store_parallel_search() {
	let mut store = TripleVectorStore::new();

	store.insert(
		create_test_point(0, "/src/main.rs"),
		ContentType::Code,
	);
	store.insert(
		create_test_point(1, "/doc/api.md"),
		ContentType::Doc,
	);
	store.insert(
		create_test_point(2, "/notes/impl.md"),
		ContentType::Notes,
	);

	store.build_indexes().unwrap();

	let results = store
		.search_parallel(&[1.0, 0.0, 0.0], 10, 10, 10);

	assert_eq!(results.code_results.len(), 1);
	assert_eq!(results.doc_results.len(), 1);
	assert_eq!(results.notes_results.len(), 1);
}

/// Test TripleVectorStore empty search
#[test]
fn test_triple_vector_store_empty() {
	let store = TripleVectorStore::new();

	assert!(store.is_empty());
	assert_eq!(store.stats().total(), 0);

	let results =
		store.search_code(&[1.0, 0.0, 0.0], 10);
	assert!(results.is_empty());
}

/// Test TripleVectorStore clear
#[test]
fn test_triple_vector_store_clear() {
	let mut store = TripleVectorStore::new();

	store.insert(
		create_test_point(0, "/src/main.rs"),
		ContentType::Code,
	);
	store.insert(
		create_test_point(1, "/doc/api.md"),
		ContentType::Doc,
	);

	assert_eq!(store.stats().total(), 2);

	store.clear();

	assert!(store.is_empty());
	assert_eq!(store.stats().total(), 0);
}
