//! Triple Vector Store — Mutation and Persistence
//!
//! Batch insert, index building, persistence, and clear
//! operations for the triple vector store.

use crate::retrieval::hybrid::triple_vector_store::{
	TripleVectorStore,
};
use crate::retrieval::RetrievalResult;

impl TripleVectorStore {
	/// Insert a batch of points with their content types
	pub fn insert_batch(
		&mut self,
		points: Vec<(
			crate::retrieval::hybrid::vector_store::VectorPoint,
			crate::indexer::symbols::ContentType,
		)>,
	) {
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

	/// Persist all three stores to disk
	pub fn persist(&self) -> RetrievalResult<()> {
		self.code_store.persist()?;
		self.doc_store.persist()?;
		self.notes_store.persist()?;
		Ok(())
	}

	/// Clear all stores
	pub fn clear(&mut self) {
		self.code_store.clear();
		self.doc_store.clear();
		self.notes_store.clear();
	}
}
