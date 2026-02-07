//! Triple Vector Store
//!
//! Provides separate HNSW vector stores for Code, Doc, and
//! Notes content types. Enables parallel semantic search
//! across all three stores without interference.

use std::path::{Path, PathBuf};

use crate::indexer::symbols::ContentType;
use crate::retrieval::hybrid::triple_vector_helpers::create_store_dirs;
use crate::retrieval::hybrid::vector_store::{
	SearchResult, VectorPoint, VectorStore,
};
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

/// Triple vector store with separate code, doc, and notes
/// HNSW indexes. Enables parallel semantic search without
/// content type interference.
pub struct TripleVectorStore {
	/// vector store for code embeddings
	pub(crate) code_store: VectorStore,
	/// vector store for doc embeddings
	pub(crate) doc_store: VectorStore,
	/// vector store for notes embeddings
	pub(crate) notes_store: VectorStore,
	/// base path for persistence (if enabled)
	pub(crate) base_path: Option<PathBuf>,
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
	pub fn with_path(
		base_path: &Path,
	) -> RetrievalResult<Self> {
		let code_path =
			base_path.join("code").join("vectors.json");
		let doc_path =
			base_path.join("docs").join("vectors.json");
		let notes_path =
			base_path.join("notes").join("vectors.json");

		// Ensure directories exist
		create_store_dirs(
			&code_path, &doc_path, &notes_path,
		)?;

		let code_store =
			VectorStore::with_path(&code_path)?;
		let doc_store =
			VectorStore::with_path(&doc_path)?;
		let notes_store =
			VectorStore::with_path(&notes_path)?;

		Ok(Self {
			code_store,
			doc_store,
			notes_store,
			base_path: Some(base_path.to_path_buf()),
		})
	}

	/// Insert a vector point, routing to appropriate store
	pub fn insert(
		&mut self,
		point: VectorPoint,
		content_type: ContentType,
	) {
		match content_type {
			ContentType::Code => {
				self.code_store.insert(point)
			}
			ContentType::Doc => {
				self.doc_store.insert(point)
			}
			ContentType::Notes => {
				self.notes_store.insert(point)
			}
		}
	}
}

impl Default for TripleVectorStore {
	fn default() -> Self {
		Self::new()
	}
}

