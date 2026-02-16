//! Triple Vector Store
//!
//! Provides separate HNSW vector stores for Code, Doc,
//! Notes, and Enriched (LLM doc) content types. Enables
//! parallel semantic search without interference.

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
	/// number of enriched (LLM doc) vectors indexed
	pub enriched_count: usize,
}

impl TripleVectorStats {
	/// Get total vectors indexed
	pub fn total(&self) -> usize {
		self.code_count
			+ self.doc_count
			+ self.notes_count
			+ self.enriched_count
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
	/// search results from enriched store
	pub enriched_results: Vec<SearchResult>,
}

/// Triple vector store with separate code, doc, notes,
/// and enriched HNSW indexes.
pub struct TripleVectorStore {
	/// vector store for code embeddings
	pub(crate) code_store: VectorStore,
	/// vector store for doc embeddings
	pub(crate) doc_store: VectorStore,
	/// vector store for notes embeddings
	pub(crate) notes_store: VectorStore,
	/// vector store for LLM doc enriched embeddings
	pub(crate) enriched_store: VectorStore,
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
			enriched_store: VectorStore::new(),
			base_path: None,
		}
	}

	/// Create triple vector store with persistence
	pub fn with_path(
		base_path: &Path,
	) -> RetrievalResult<Self> {
		let paths = build_store_paths(base_path);

		create_store_dirs(
			&paths.0, &paths.1,
			&paths.2, &paths.3,
		)?;

		let stores =
			open_all_stores(&paths)?;

		Ok(Self {
			code_store: stores.0,
			doc_store: stores.1,
			notes_store: stores.2,
			enriched_store: stores.3,
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

	/// Insert into the enriched (LLM doc) store
	pub fn insert_enriched(&mut self, point: VectorPoint) {
		self.enriched_store.insert(point);
	}
}

/// Build the four vector store paths from base
fn build_store_paths(base: &Path) -> FourPaths {
	let vectors = "vectors.bin";
	(
		base.join("code").join(vectors),
		base.join("docs").join(vectors),
		base.join("notes").join(vectors),
		base.join("enriched").join(vectors),
	)
}

/// Four vector store paths (code, doc, notes, enriched)
type FourPaths = (PathBuf, PathBuf, PathBuf, PathBuf);

/// Four opened vector stores
type FourStores =
	(VectorStore, VectorStore, VectorStore, VectorStore);

/// Open all four vector stores from paths
fn open_all_stores(
	paths: &FourPaths,
) -> RetrievalResult<FourStores> {
	Ok((
		VectorStore::with_path(&paths.0)?,
		VectorStore::with_path(&paths.1)?,
		VectorStore::with_path(&paths.2)?,
		VectorStore::with_path(&paths.3)?,
	))
}

impl Default for TripleVectorStore {
	fn default() -> Self {
		Self::new()
	}
}
