//! Triple Hybrid Search — Search Operations
//!
//! Main search execution across code, doc, and notes
//! pipelines with parallel keyword and semantic search.
//! Enriched results boost code pipeline with NL-focused
//! embeddings.

use crate::retrieval::daemon::protocol::SearchSpec;
use crate::indexer::triple_search::TripleLimits;
use crate::retrieval::hybrid::triple::{
	PipelineSlice, TripleHybridResults,
	TripleHybridSearch,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Keyword + semantic search result pair
type SearchPair = (
	crate::indexer::triple_search::TripleSearchResults,
	super::triple_vector_store::TripleVectorResults,
);

impl TripleHybridSearch {
	/// Parallel hybrid search across all pipelines
	pub fn search(
		&self,
		query: &str,
		limits: &TripleLimits,
	) -> RetrievalResult<TripleHybridResults> {
		let (kw_results, sem_results) =
			self.run_triple_searches(query)?;
		let code_results = self.fuse_code_pipeline(
			&kw_results, &sem_results, limits.code,
		);
		let doc_results =
			self.fuse_doc_pipeline(&kw_results, &sem_results, limits);
		let notes_results = self
			.fuse_notes_pipeline(&kw_results, &sem_results, limits);
		Ok(TripleHybridResults {
			code_results,
			doc_results,
			notes_results,
		})
	}

	/// Fuse doc pipeline results
	fn fuse_doc_pipeline(
		&self,
		kw_results: &crate::indexer::triple_search::TripleSearchResults,
		sem_results: &super::triple_vector_store::TripleVectorResults,
		limits: &TripleLimits,
	) -> Vec<super::HybridSearchResult> {
		self.fuse_pipeline_results(
			&kw_results.doc_results,
			&sem_results.doc_results,
			&PipelineSlice {
				limit: limits.doc,
				content_type: "doc",
			},
		)
	}
}

impl TripleHybridSearch {
	/// Fuse notes pipeline results
	fn fuse_notes_pipeline(
		&self,
		kw_results: &crate::indexer::triple_search::TripleSearchResults,
		sem_results: &super::triple_vector_store::TripleVectorResults,
		limits: &TripleLimits,
	) -> Vec<super::HybridSearchResult> {
		self.fuse_pipeline_results(
			&kw_results.notes_results,
			&sem_results.notes_results,
			&PipelineSlice {
				limit: limits.notes,
				content_type: "notes",
			},
		)
	}

	/// Run both keyword and semantic searches
	fn run_triple_searches(
		&self,
		query: &str,
	) -> RetrievalResult<SearchPair> {
		let candidates =
			self.config.candidates_per_source;
		let embedding =
			self.embed_query(query)?;
		let cand_limits = TripleLimits {
			code: candidates,
			doc: candidates,
			notes: candidates,
		};
		let keyword_results = self
			.keyword_index
			.search_parallel(query, cand_limits)
			.map_err(|err| {
				RetrievalError::Embedding(
					err.to_string(),
				)
			})?;
		let semantic_results = self
			.vector_store
			.search_parallel(&embedding, &cand_limits);
		Ok((keyword_results, semantic_results))
	}

	/// Embed query text into a vector
	fn embed_query(
		&self,
		query: &str,
	) -> RetrievalResult<Vec<f32>> {
		let query_emb = self
			.daemon_client
			.embed(vec![query.to_string()])?;
		query_emb
			.into_iter()
			.next()
			.ok_or_else(|| {
				RetrievalError::Embedding(
					"no embedding returned".to_string(),
				)
			})
	}
}

impl TripleHybridSearch {
	/// Fuse code pipeline with enriched boost
	fn fuse_code_pipeline(
		&self,
		kw_results: &crate::indexer::triple_search::TripleSearchResults,
		sem_results: &super::triple_vector_store::TripleVectorResults,
		limit: usize,
	) -> Vec<super::HybridSearchResult> {
		let slice = PipelineSlice {
			limit,
			content_type: "code",
		};
		let enriched_input =
			super::triple_fusion_enriched::EnrichedFusionInput {
				keyword_hits: &kw_results.code_results,
				semantic_results: &sem_results
					.code_results,
				enriched_results: &sem_results
					.enriched_results,
			};
		self.fuse_with_enriched(enriched_input, &slice)
	}

	/// Search with SearchSpec (for query expansion)
	pub fn search_with_spec(
		&self,
		spec: &SearchSpec,
		limits: &TripleLimits,
	) -> RetrievalResult<TripleHybridResults> {
		self.search(&spec.original_query, limits)
	}
}
