//! Triple Vector Store — Query Operations
//!
//! Read-only state queries for the triple vector store:
//! stats, emptiness checks, and index readiness.

use std::path::Path;

use crate::retrieval::hybrid::triple_vector_store::{
	TripleVectorStats, TripleVectorStore,
};

impl TripleVectorStore {
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
		self.code_store.is_empty()
			&& self.doc_store.is_empty()
			&& self.notes_store.is_empty()
	}

	/// Check if all indexes are built
	pub fn is_indexed(&self) -> bool {
		self.code_store.is_indexed()
			&& self.doc_store.is_indexed()
			&& self.notes_store.is_indexed()
	}

	/// Get base path (if persistent)
	pub fn path(&self) -> Option<&Path> {
		self.base_path.as_deref()
	}
}
