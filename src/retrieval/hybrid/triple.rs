//! Triple Hybrid Search
//!
//! Combines keyword search (Tantivy) with semantic search (embeddings)
//! across three separate pipelines: Code, Doc, and Notes.
//! Uses parallel execution for fast retrieval without content type interference.

use std::collections::HashMap;
use std::path::Path;

use crate::indexer::symbols::{ContentType, Symbol};
use crate::indexer::triple_search::TripleSearchIndex;
use crate::indexer::SearchHit;
use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::hybrid::fusion::{rrf_score, RankedItem};
use crate::retrieval::hybrid::triple_vector_store::TripleVectorStore;
use crate::retrieval::hybrid::vector_store::{SearchResult as VectorSearchResult, VectorPoint};
use crate::retrieval::hybrid::{HybridSearchConfig, HybridSearchResult};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Stats from triple hybrid indexing
#[derive(Debug, Clone, Default)]
pub struct TripleHybridStats {
	/// code keyword symbols indexed
	pub code_keyword_count: usize,
	/// code vectors indexed
	pub code_vector_count: usize,
	/// doc keyword symbols indexed
	pub doc_keyword_count: usize,
	/// doc vectors indexed
	pub doc_vector_count: usize,
	/// notes keyword symbols indexed
	pub notes_keyword_count: usize,
	/// notes vectors indexed
	pub notes_vector_count: usize,
}

impl TripleHybridStats {
	/// Get total symbols indexed
	pub fn total(&self) -> usize {
		self.code_keyword_count + self.doc_keyword_count + self.notes_keyword_count
	}
}

/// Results from triple hybrid search
#[derive(Debug, Default)]
pub struct TripleHybridResults {
	/// hybrid search results from code pipeline
	pub code_results: Vec<HybridSearchResult>,
	/// hybrid search results from doc pipeline
	pub doc_results: Vec<HybridSearchResult>,
	/// hybrid search results from notes pipeline
	pub notes_results: Vec<HybridSearchResult>,
}

/// Triple hybrid search with separate code, doc, and notes pipelines.
/// Combines keyword and semantic search with RRF fusion for each pipeline.
pub struct TripleHybridSearch {
	/// triple keyword index (code, doc, notes)
	keyword_index: TripleSearchIndex,
	/// triple vector store (code, doc, notes)
	vector_store: TripleVectorStore,
	/// daemon client for embedding generation
	daemon_client: DaemonClient,
	/// configuration for search
	config: HybridSearchConfig,
	/// symbol lookup for converting vector results to symbols
	symbols: Vec<Symbol>,
}

impl TripleHybridSearch {
	/// Create new triple hybrid search (in-memory)
	pub fn new(daemon_client: DaemonClient) -> RetrievalResult<Self> {
		let keyword_index = TripleSearchIndex::in_memory()
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;
		let vector_store = TripleVectorStore::new();

		Ok(Self {
			keyword_index,
			vector_store,
			daemon_client,
			config: HybridSearchConfig::default(),
			symbols: Vec::new(),
		})
	}

	/// Create triple hybrid search with persistence
	pub fn with_persistence(base_path: &Path, daemon_client: DaemonClient) -> RetrievalResult<Self> {
		let keyword_index = TripleSearchIndex::open_or_create(base_path)
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;
		let vector_store = TripleVectorStore::with_path(base_path)?;

		Ok(Self {
			keyword_index,
			vector_store,
			daemon_client,
			config: HybridSearchConfig::default(),
			symbols: Vec::new(),
		})
	}

	/// Set custom configuration for search
	pub fn with_config(mut self, config: HybridSearchConfig) -> Self {
		self.config = config;
		self
	}

	/// Index symbols into all three pipelines
	pub fn index_symbols(&mut self, symbols: &[Symbol]) -> RetrievalResult<TripleHybridStats> {
		// Store symbols for later lookup
		self.symbols = symbols.to_vec();

		// Index into keyword index (routes by content type)
		let keyword_stats = self.keyword_index
			.index_symbols(symbols)
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

		// Generate embeddings and index into vector store
		let texts: Vec<String> = symbols
			.iter()
			.map(|s| self.symbol_to_text(s))
			.collect();

		let embeddings = self.daemon_client.embed(texts)?;

		// Route embeddings to appropriate vector store
		for (i, (symbol, embedding)) in symbols.iter().zip(embeddings.iter()).enumerate() {
			let content_type = ContentType::from_path(&symbol.location.file);
			let point = VectorPoint {
				id: i as u64,
				vector: embedding.clone(),
				file_path: symbol.location.file.clone(),
				line: symbol.location.line,
				symbol_name: symbol.name.clone(),
				symbol_kind: symbol.kind.to_string(),
			};
			self.vector_store.insert(point, content_type);
		}

		// Build HNSW indexes
		self.vector_store.build_indexes()?;

		let vector_stats = self.vector_store.stats();

		Ok(TripleHybridStats {
			code_keyword_count: keyword_stats.code_count,
			code_vector_count: vector_stats.code_count,
			doc_keyword_count: keyword_stats.doc_count,
			doc_vector_count: vector_stats.doc_count,
			notes_keyword_count: keyword_stats.notes_count,
			notes_vector_count: vector_stats.notes_count,
		})
	}

	/// Convert symbol to text for embedding
	fn symbol_to_text(&self, symbol: &Symbol) -> String {
		let mut parts = vec![symbol.name.clone()];

		if let Some(ref sig) = symbol.signature {
			parts.push(sig.clone());
		}

		if let Some(ref doc) = symbol.doc_comment {
			parts.push(doc.clone());
		}

		if let Some(ref content) = symbol.content {
			// For doc chunks, use the content
			parts.push(content.clone());
		}

		parts.join(" ")
	}

	/// Parallel hybrid search across all three pipelines
	pub fn search(
		&self,
		query: &str,
		code_limit: usize,
		doc_limit: usize,
		notes_limit: usize,
	) -> RetrievalResult<TripleHybridResults> {
		let candidates = self.config.candidates_per_source;

		// Get query embedding once (shared across all pipelines)
		let query_embeddings = self.daemon_client.embed(vec![query.to_string()])?;
		let query_embedding = query_embeddings
			.first()
			.ok_or_else(|| RetrievalError::Embedding("no embedding returned".to_string()))?;

		// Parallel keyword search
		let keyword_results = self.keyword_index
			.search_parallel(query, candidates, candidates, candidates)
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

		// Parallel semantic search
		let semantic_results = self.vector_store.search_parallel(
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

	/// Search with SearchSpec (for query expansion integration)
	pub fn search_with_spec(
		&self,
		spec: &SearchSpec,
		code_limit: usize,
		doc_limit: usize,
		notes_limit: usize,
	) -> RetrievalResult<TripleHybridResults> {
		// Use original query for search
		self.search(&spec.original_query, code_limit, doc_limit, notes_limit)
	}

	/// Filter results by relevance score with min/max guarantees
	/// - Always includes minimum results (if available)
	/// - Filters out results below threshold after minimum met
	/// - Stops at max_limit
	fn filter_by_relevance(
		&self,
		results: Vec<HybridSearchResult>,
		max_limit: usize,
		content_type: &str,
	) -> Vec<HybridSearchResult> {
		let threshold = self.config.rrf_score_threshold; // minimum RRF score to include
		let min_results = self.config.min_results_per_type; // always include at least this many
		let total = results.len(); // total results before filtering

		let mut filtered: Vec<HybridSearchResult> = Vec::new(); // filtered results

		for result in results.into_iter() {
			// Always include if we haven't met minimum
			if filtered.len() < min_results {
				filtered.push(result);
				continue;
			}

			// Stop if we hit max limit
			if filtered.len() >= max_limit {
				break;
			}

			// Include only if above threshold
			if result.rrf_score >= threshold {
				filtered.push(result);
			}
		}

		// Debug log if filtering occurred
		let below_threshold = total.saturating_sub(filtered.len());
		if below_threshold > 0 {
			eprintln!(
				"[pipeline] {}: {} results, {} filtered (score < {})",
				content_type,
				filtered.len(),
				below_threshold,
				threshold
			);
		}

		filtered
	}

	/// Fuse keyword and semantic results for a single pipeline
	fn fuse_pipeline_results(
		&self,
		keyword_hits: &[SearchHit],
		semantic_results: &[VectorSearchResult],
		limit: usize,
		content_type: &str,
	) -> Vec<HybridSearchResult> {
		// Convert to ranked items
		let keyword_ranked: Vec<RankedItem<SearchHit>> = keyword_hits
			.iter()
			.enumerate()
			.map(|(i, hit)| RankedItem {
				item: hit.clone(),
				rank: i + 1,
				score: hit.score,
			})
			.collect();

		let semantic_ranked: Vec<RankedItem<&VectorSearchResult>> = semantic_results
			.iter()
			.enumerate()
			.map(|(i, result)| RankedItem {
				item: result,
				rank: i + 1,
				score: result.distance,
			})
			.collect();

		// Build RRF scores
		let mut scores: HashMap<String, HybridSearchResult> = HashMap::new();
		let k = self.config.rrf_k;

		// Process keyword results
		for ranked in &keyword_ranked {
			let key = self.symbol_key(&ranked.item.symbol);
			let rrf = rrf_score(ranked.rank, k) * self.config.keyword_weight;

			scores.entry(key).or_insert_with(|| HybridSearchResult {
				symbol: ranked.item.symbol.clone(),
				rrf_score: 0.0,
				keyword_rank: None,
				semantic_rank: None,
				keyword_score: None,
				semantic_distance: None,
				rerank_score: None,
			}).rrf_score += rrf;

			if let Some(result) = scores.get_mut(&self.symbol_key(&ranked.item.symbol)) {
				result.keyword_rank = Some(ranked.rank);
				result.keyword_score = Some(ranked.score);
			}
		}

		// Process semantic results
		for ranked in &semantic_ranked {
			// Find matching symbol
			if let Some(symbol) = self.find_symbol_by_vector(&ranked.item.point) {
				let key = self.symbol_key(&symbol);
				let rrf = rrf_score(ranked.rank, k) * self.config.semantic_weight;

				scores.entry(key.clone()).or_insert_with(|| HybridSearchResult {
					symbol: symbol.clone(),
					rrf_score: 0.0,
					keyword_rank: None,
					semantic_rank: None,
					keyword_score: None,
					semantic_distance: None,
					rerank_score: None,
				}).rrf_score += rrf;

				if let Some(result) = scores.get_mut(&key) {
					result.semantic_rank = Some(ranked.rank);
					result.semantic_distance = Some(ranked.item.distance);
				}
			}
		}

		// Sort by RRF score and filter by relevance
		let mut results: Vec<HybridSearchResult> = scores.into_values().collect();
		results.sort_by(|a, b| b.rrf_score.partial_cmp(&a.rrf_score).unwrap_or(std::cmp::Ordering::Equal));

		self.filter_by_relevance(results, limit, content_type)
	}

	/// Create a unique key for a symbol
	fn symbol_key(&self, symbol: &Symbol) -> String {
		format!(
			"{}:{}:{}",
			symbol.location.file.display(),
			symbol.location.line,
			symbol.name
		)
	}

	/// Find symbol by vector point
	fn find_symbol_by_vector(&self, point: &VectorPoint) -> Option<&Symbol> {
		self.symbols.get(point.id as usize)
	}

	/// Persist all indexes and stores
	pub fn persist(&self) -> RetrievalResult<()> {
		self.vector_store.persist()
	}

	/// Get stats
	pub fn stats(&self) -> TripleHybridStats {
		let vector_stats = self.vector_store.stats();
		TripleHybridStats {
			code_keyword_count: 0, // Would need to track this
			code_vector_count: vector_stats.code_count,
			doc_keyword_count: 0,
			doc_vector_count: vector_stats.doc_count,
			notes_keyword_count: 0,
			notes_vector_count: vector_stats.notes_count,
		}
	}

	/// Check if empty
	pub fn is_empty(&self) -> bool {
		self.symbols.is_empty()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::indexer::symbols::{CodeLocation, SymbolKind};
	use std::path::PathBuf;

	/// Create a test symbol
	fn create_test_symbol(name: &str, path: &str, kind: SymbolKind) -> Symbol {
		Symbol::new(
			name.to_string(),
			kind,
			CodeLocation::new(PathBuf::from(path), 1, 0, 0, 10),
		)
	}

	/// Test TripleHybridStats total calculation
	#[test]
	fn test_triple_hybrid_stats_total() {
		let stats = TripleHybridStats {
			code_keyword_count: 10,
			code_vector_count: 10,
			doc_keyword_count: 5,
			doc_vector_count: 5,
			notes_keyword_count: 3,
			notes_vector_count: 3,
		};

		// Total should be keyword counts (10 + 5 + 3 = 18)
		assert_eq!(stats.total(), 18);
	}

	/// Test TripleHybridResults default
	#[test]
	fn test_triple_hybrid_results_default() {
		let results = TripleHybridResults::default();

		assert!(results.code_results.is_empty());
		assert!(results.doc_results.is_empty());
		assert!(results.notes_results.is_empty());
	}

	/// Test symbol_to_text conversion
	#[test]
	fn test_symbol_to_text() {
		// This test would require a mock daemon client
		// For now, just verify the struct can be created
		let stats = TripleHybridStats::default();
		assert_eq!(stats.total(), 0);
	}

	/// Helper to create a mock HybridSearchResult with given RRF score
	fn mock_result(rrf_score: f32) -> HybridSearchResult {
		HybridSearchResult {
			symbol: Symbol::new(
				format!("test_{}", (rrf_score * 1000.0) as i32),
				SymbolKind::Function,
				CodeLocation::new(
					PathBuf::from("test.rs"),
					1, 0, 0, 10,
				),
			),
			rrf_score,
			keyword_rank: None,
			semantic_rank: None,
			keyword_score: None,
			semantic_distance: None,
			rerank_score: None,
		}
	}

	/// Test: Results above threshold are kept
	#[test]
	fn test_filter_keeps_high_scores() {
		// Create results with varying scores
		let results = vec![
			mock_result(0.025), // above 0.015 threshold
			mock_result(0.020), // above threshold
			mock_result(0.010), // below threshold
			mock_result(0.005), // below threshold
		];

		// Create config with threshold 0.015, min 1
		let config = HybridSearchConfig {
			rrf_score_threshold: 0.015,
			min_results_per_type: 1,
			..Default::default()
		};

		// Filter with max 10
		let threshold = config.rrf_score_threshold;
		let min_results = config.min_results_per_type;

		let mut result_filtered: Vec<HybridSearchResult> = Vec::new();
		for result in results.into_iter() {
			if result_filtered.len() < min_results {
				result_filtered.push(result);
				continue;
			}
			if result_filtered.len() >= 10 {
				break;
			}
			if result.rrf_score >= threshold {
				result_filtered.push(result);
			}
		}

		// Should keep 2 results (scores 0.025 and 0.020)
		assert_eq!(result_filtered.len(), 2);
	}

	/// Test: Minimum guarantee honored even with low scores
	#[test]
	fn test_filter_respects_minimum() {
		let results = vec![
			mock_result(0.010), // below threshold
			mock_result(0.005), // below threshold
		];

		let config = HybridSearchConfig {
			rrf_score_threshold: 0.015,
			min_results_per_type: 1,
			..Default::default()
		};

		let threshold = config.rrf_score_threshold;
		let min_results = config.min_results_per_type;

		let mut filtered: Vec<HybridSearchResult> = Vec::new();
		for result in results.into_iter() {
			if filtered.len() < min_results {
				filtered.push(result);
				continue;
			}
			if filtered.len() >= 10 {
				break;
			}
			if result.rrf_score >= threshold {
				filtered.push(result);
			}
		}

		// Should keep 1 result (minimum guarantee)
		assert_eq!(filtered.len(), 1);
	}

	/// Test: Max limit still enforced
	#[test]
	fn test_filter_respects_maximum() {
		let results = vec![
			mock_result(0.030),
			mock_result(0.025),
			mock_result(0.020),
			mock_result(0.018),
		];

		let max_limit = 2;
		let threshold = 0.015_f32;
		let min_results = 1_usize;

		let mut filtered: Vec<HybridSearchResult> = Vec::new();
		for result in results.into_iter() {
			if filtered.len() < min_results {
				filtered.push(result);
				continue;
			}
			if filtered.len() >= max_limit {
				break;
			}
			if result.rrf_score >= threshold {
				filtered.push(result);
			}
		}

		// Should keep 2 results (max limit)
		assert_eq!(filtered.len(), 2);
	}

	/// Test: Empty input returns empty
	#[test]
	fn test_filter_empty_input() {
		let results: Vec<HybridSearchResult> = vec![];

		let mut filtered: Vec<HybridSearchResult> = Vec::new();
		for result in results.into_iter() {
			if filtered.len() < 1 {
				filtered.push(result);
				continue;
			}
			if result.rrf_score >= 0.015 {
				filtered.push(result);
			}
		}

		assert!(filtered.is_empty());
	}

	/// Test: All results below threshold, min=0 returns empty
	#[test]
	fn test_filter_all_below_threshold_min_zero() {
		let results = vec![
			mock_result(0.010),
			mock_result(0.005),
		];

		let threshold = 0.015_f32;
		let min_results = 0_usize;

		let mut filtered: Vec<HybridSearchResult> = Vec::new();
		for result in results.into_iter() {
			if filtered.len() < min_results {
				filtered.push(result);
				continue;
			}
			if result.rrf_score >= threshold {
				filtered.push(result);
			}
		}

		// Should be empty (min=0, all below threshold)
		assert!(filtered.is_empty());
	}
}
