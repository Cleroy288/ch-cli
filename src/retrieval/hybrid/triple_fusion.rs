//! Triple Hybrid Search — Search Operations
//!
//! Main search execution across code, doc, and notes pipelines
//! with parallel keyword and semantic search.

use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::hybrid::triple::{
	TripleHybridResults, TripleHybridSearch,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

impl TripleHybridSearch {
	/// Parallel hybrid search across all three pipelines
	pub fn search(
		&self,
		query: &str,
		code_limit: usize,
		doc_limit: usize,
		notes_limit: usize,
	) -> RetrievalResult<TripleHybridResults> {
		let candidates =
			self.config.candidates_per_source;

		// Get query embedding once (shared)
		let query_embeddings = self
			.daemon_client
			.embed(vec![query.to_string()])?;
		let query_embedding =
			query_embeddings.first().ok_or_else(|| {
				RetrievalError::Embedding(
					"no embedding returned".to_string(),
				)
			})?;

		// Parallel keyword search
		let keyword_results = self
			.keyword_index
			.search_parallel(
				query, candidates, candidates, candidates,
			)
			.map_err(|e| {
				RetrievalError::Embedding(e.to_string())
			})?;

		// Parallel semantic search
		let semantic_results =
			self.vector_store.search_parallel(
				query_embedding,
				candidates,
				candidates,
				candidates,
			);

		// Fuse results for each pipeline
		let code_results = self.fuse_pipeline_results(
			&keyword_results.code_results,
			&semantic_results.code_results,
			code_limit,
			"code",
		);

		let doc_results = self.fuse_pipeline_results(
			&keyword_results.doc_results,
			&semantic_results.doc_results,
			doc_limit,
			"doc",
		);

		let notes_results = self.fuse_pipeline_results(
			&keyword_results.notes_results,
			&semantic_results.notes_results,
			notes_limit,
			"notes",
		);

		Ok(TripleHybridResults {
			code_results,
			doc_results,
			notes_results,
		})
	}

	/// Search with SearchSpec (for query expansion)
	pub fn search_with_spec(
		&self,
		spec: &SearchSpec,
		code_limit: usize,
		doc_limit: usize,
		notes_limit: usize,
	) -> RetrievalResult<TripleHybridResults> {
		self.search(
			&spec.original_query,
			code_limit,
			doc_limit,
			notes_limit,
		)
	}
}
