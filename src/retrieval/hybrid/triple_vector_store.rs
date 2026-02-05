//! Triple Vector Store
//!
//! Provides separate HNSW vector stores for Code, Doc, and Notes content types.
//! Enables parallel semantic search across all three stores without interference.

use std::path::{Path, PathBuf};

use crate::indexer::symbols::ContentType;
use crate::retrieval::hybrid::vector_store::{SearchResult, VectorPoint, VectorStore};
use crate::retrieval::RetrievalResult;

/// Stats from triple vector indexing
#[derive(Debug, Clone, Default)]
pub struct TripleVectorStats {
	/// number of code vectors indexed
	pub code_count: usize,
	/// number of doc vectors indexed
	pub doc_count: usize,
	/// number of notes vectors indexed
	pub notes_count: usize,
}

impl TripleVectorStats {
	/// Get total vectors indexed
	pub fn total(&self) -> usize {
		self.code_count + self.doc_count + self.notes_count
	}
}

/// Results from triple vector search
#[derive(Debug, Default)]
pub struct TripleVectorResults {
	/// search results from code store
	pub code_results: Vec<SearchResult>,
	/// search results from doc store
	pub doc_results: Vec<SearchResult>,
	/// search results from notes store
	pub notes_results: Vec<SearchResult>,
}

/// Triple vector store with separate code, doc, and notes HNSW indexes.
/// Enables parallel semantic search without content type interference.
pub struct TripleVectorStore {
	/// vector store for code embeddings
	code_store: VectorStore,
	/// vector store for doc embeddings
	doc_store: VectorStore,
	/// vector store for notes embeddings
	notes_store: VectorStore,
	/// base path for persistence (if enabled)
	base_path: Option<PathBuf>,
}

impl TripleVectorStore {
	/// Create new in-memory triple vector store
	pub fn new() -> Self {
		Self {
			code_store: VectorStore::new(),
			doc_store: VectorStore::new(),
			notes_store: VectorStore::new(),
			base_path: None,
		}
	}

	/// Create triple vector store with persistence
	pub fn with_path(base_path: &Path) -> RetrievalResult<Self> {
		let code_path = base_path.join("code").join("vectors.json");
		let doc_path = base_path.join("docs").join("vectors.json");
		let notes_path = base_path.join("notes").join("vectors.json");

		// Ensure directories exist
		if let Some(parent) = code_path.parent() {
			std::fs::create_dir_all(parent)?;
		}
		if let Some(parent) = doc_path.parent() {
			std::fs::create_dir_all(parent)?;
		}
		if let Some(parent) = notes_path.parent() {
			std::fs::create_dir_all(parent)?;
		}

		let code_store = VectorStore::with_path(&code_path)?;
		let doc_store = VectorStore::with_path(&doc_path)?;
		let notes_store = VectorStore::with_path(&notes_path)?;

		Ok(Self {
			code_store,
			doc_store,
			notes_store,
			base_path: Some(base_path.to_path_buf()),
		})
	}

	/// Insert a vector point, routing to appropriate store
	pub fn insert(&mut self, point: VectorPoint, content_type: ContentType) {
		match content_type {
			ContentType::Code => self.code_store.insert(point),
			ContentType::Doc => self.doc_store.insert(point),
			ContentType::Notes => self.notes_store.insert(point),
		}
	}

	/// Insert a batch of points with their content types
	pub fn insert_batch(&mut self, points: Vec<(VectorPoint, ContentType)>) {
		for (point, content_type) in points {
			self.insert(point, content_type);
		}
	}

	/// Build all three HNSW indexes
	pub fn build_indexes(&mut self) -> RetrievalResult<()> {
		self.code_store.build_index()?;
		self.doc_store.build_index()?;
		self.notes_store.build_index()?;
		Ok(())
	}

	/// Search code vectors only
	pub fn search_code(&self, query: &[f32], k: usize) -> Vec<SearchResult> {
		self.code_store.search(query, k)
	}

	/// Search doc vectors only
	pub fn search_docs(&self, query: &[f32], k: usize) -> Vec<SearchResult> {
		self.doc_store.search(query, k)
	}

	/// Search notes vectors only
	pub fn search_notes(&self, query: &[f32], k: usize) -> Vec<SearchResult> {
		self.notes_store.search(query, k)
	}

	/// Parallel search all three stores using rayon
	pub fn search_parallel(
		&self,
		query: &[f32],
		code_k: usize,
		doc_k: usize,
		notes_k: usize,
	) -> TripleVectorResults {
		// Use rayon::join for parallel execution
		let ((code_results, doc_results), notes_results) = rayon::join(
			|| {
				rayon::join(
					|| self.search_code(query, code_k),
					|| self.search_docs(query, doc_k),
				)
			},
			|| self.search_notes(query, notes_k),
		);

		TripleVectorResults {
			code_results,
			doc_results,
			notes_results,
		}
	}

	/// Sequential search all three stores (fallback)
	pub fn search_sequential(
		&self,
		query: &[f32],
		code_k: usize,
		doc_k: usize,
		notes_k: usize,
	) -> TripleVectorResults {
		let code_results = self.search_code(query, code_k);
		let doc_results = self.search_docs(query, doc_k);
		let notes_results = self.search_notes(query, notes_k);

		TripleVectorResults {
			code_results,
			doc_results,
			notes_results,
		}
	}

	/// Persist all three stores to disk
	pub fn persist(&self) -> RetrievalResult<()> {
		self.code_store.persist()?;
		self.doc_store.persist()?;
		self.notes_store.persist()?;
		Ok(())
	}

	/// Get stats for all stores
	pub fn stats(&self) -> TripleVectorStats {
		TripleVectorStats {
			code_count: self.code_store.len(),
			doc_count: self.doc_store.len(),
			notes_count: self.notes_store.len(),
		}
	}

	/// Check if all stores are empty
	pub fn is_empty(&self) -> bool {
		self.code_store.is_empty() && self.doc_store.is_empty() && self.notes_store.is_empty()
	}

	/// Check if all indexes are built
	pub fn is_indexed(&self) -> bool {
		self.code_store.is_indexed() && self.doc_store.is_indexed() && self.notes_store.is_indexed()
	}

	/// Clear all stores
	pub fn clear(&mut self) {
		self.code_store.clear();
		self.doc_store.clear();
		self.notes_store.clear();
	}

	/// Get base path (if persistent)
	pub fn path(&self) -> Option<&Path> {
		self.base_path.as_deref()
	}
}

impl Default for TripleVectorStore {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::path::PathBuf;

	/// Create a test vector point
	fn create_test_point(id: u64, file_path: &str) -> VectorPoint {
		VectorPoint {
			id,
			vector: vec![1.0, 0.0, 0.0], // simple unit vector
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

		// Insert points for each content type
		store.insert(create_test_point(0, "/src/main.rs"), ContentType::Code);
		store.insert(create_test_point(1, "/tests/test.rs"), ContentType::Code);
		store.insert(create_test_point(2, "/doc/api.md"), ContentType::Doc);
		store.insert(create_test_point(3, "/notes/impl.md"), ContentType::Notes);
		store.insert(create_test_point(4, "/notes/bench.md"), ContentType::Notes);

		let stats = store.stats();

		assert_eq!(stats.code_count, 2);
		assert_eq!(stats.doc_count, 1);
		assert_eq!(stats.notes_count, 2);
		assert_eq!(stats.total(), 5);
	}

	/// Test TripleVectorStore search returns from correct store
	#[test]
	fn test_triple_vector_store_search_code() {
		let mut store = TripleVectorStore::new();

		store.insert(create_test_point(0, "/src/main.rs"), ContentType::Code);
		store.insert(create_test_point(1, "/doc/api.md"), ContentType::Doc);

		store.build_indexes().unwrap();

		// Search code - should find 1
		let results = store.search_code(&[1.0, 0.0, 0.0], 10);
		assert_eq!(results.len(), 1);
		assert_eq!(results[0].point.id, 0);
	}

	/// Test TripleVectorStore parallel search
	#[test]
	fn test_triple_vector_store_parallel_search() {
		let mut store = TripleVectorStore::new();

		store.insert(create_test_point(0, "/src/main.rs"), ContentType::Code);
		store.insert(create_test_point(1, "/doc/api.md"), ContentType::Doc);
		store.insert(create_test_point(2, "/notes/impl.md"), ContentType::Notes);

		store.build_indexes().unwrap();

		// Parallel search all
		let results = store.search_parallel(&[1.0, 0.0, 0.0], 10, 10, 10);

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

		let results = store.search_code(&[1.0, 0.0, 0.0], 10);
		assert!(results.is_empty());
	}

	/// Test TripleVectorStore clear
	#[test]
	fn test_triple_vector_store_clear() {
		let mut store = TripleVectorStore::new();

		store.insert(create_test_point(0, "/src/main.rs"), ContentType::Code);
		store.insert(create_test_point(1, "/doc/api.md"), ContentType::Doc);

		assert_eq!(store.stats().total(), 2);

		store.clear();

		assert!(store.is_empty());
		assert_eq!(store.stats().total(), 0);
	}
}
