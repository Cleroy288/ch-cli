//! Hybrid Search Module
//!
//! Combines keyword search (Tantivy) with semantic search (embeddings)
//! using Reciprocal Rank Fusion for result merging.

pub mod adaptive;
pub mod embedding;
pub mod fusion;
pub mod triple;
pub mod triple_vector_store;
pub mod vector_store;

use std::path::Path;

pub use adaptive::{compute_weights, AdaptiveWeights, QueryType};
pub use embedding::BgeEmbedder;
pub use fusion::{FusedResult, RankedItem};
pub use triple::{TripleHybridResults, TripleHybridSearch, TripleHybridStats};
pub use triple_vector_store::{TripleVectorResults, TripleVectorStats, TripleVectorStore};
pub use vector_store::{SearchResult as VectorSearchResult, VectorPoint, VectorStore};

use crate::indexer::{DocumentType, SearchHit, SearchIndex, Symbol};
use crate::retrieval::daemon::protocol::{QueryIntent, SearchSpec};
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Configuration for hybrid search
#[derive(Debug, Clone)]
pub struct HybridSearchConfig {
	/// weight for keyword search results
	pub keyword_weight: f32,
	/// weight for semantic search results
	pub semantic_weight: f32,
	/// number of candidates to fetch from each source
	pub candidates_per_source: usize,
	/// RRF k constant
	pub rrf_k: f32,
	/// Minimum RRF score threshold - results below this are filtered out
	pub rrf_score_threshold: f32,
	/// Minimum results to keep per content type (code, doc, test) regardless of threshold
	pub min_results_per_type: usize,
}

impl Default for HybridSearchConfig {
	fn default() -> Self {
		Self {
			keyword_weight: 1.0,
			semantic_weight: 1.0,
			candidates_per_source: 50,
			rrf_k: 60.0,
			rrf_score_threshold: 0.015,
			min_results_per_type: 1,
		}
	}
}

/// Result from hybrid search
#[derive(Debug, Clone)]
pub struct HybridSearchResult {
	/// the symbol
	pub symbol: Symbol,
	/// combined RRF score
	pub rrf_score: f32,
	/// keyword rank (None if not in keyword results)
	pub keyword_rank: Option<usize>,
	/// semantic rank (None if not in semantic results)
	pub semantic_rank: Option<usize>,
	/// keyword BM25 score
	pub keyword_score: Option<f32>,
	/// semantic distance (lower is better)
	pub semantic_distance: Option<f32>,
	/// rerank score from cross-encoder (if reranking was applied)
	pub rerank_score: Option<f32>,
}

/// Hybrid search combining keyword and semantic search
pub struct HybridSearch {
	/// Tantivy keyword search index
	keyword_index: SearchIndex,
	/// vector store for semantic search
	vector_store: VectorStore,
	/// daemon client for embeddings
	daemon_client: DaemonClient,
	/// configuration
	config: HybridSearchConfig,
}

impl HybridSearch {
	/// Create a new hybrid search with in-memory indices
	pub fn new() -> RetrievalResult<Self> {
		let keyword_index = SearchIndex::in_memory()
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;
		let vector_store = VectorStore::new();
		let daemon_client = DaemonClient::new();

		Ok(Self {
			keyword_index,
			vector_store,
			daemon_client,
			config: HybridSearchConfig::default(),
		})
	}

	/// Create hybrid search with persistence
	pub fn with_paths(
		tantivy_path: impl AsRef<Path>,
		vector_path: impl AsRef<Path>,
	) -> RetrievalResult<Self> {
		let keyword_index = SearchIndex::open_or_create(tantivy_path.as_ref())
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;
		let vector_store = VectorStore::with_path(vector_path)?;
		let daemon_client = DaemonClient::new();

		Ok(Self {
			keyword_index,
			vector_store,
			daemon_client,
			config: HybridSearchConfig::default(),
		})
	}

	/// Set configuration
	pub fn with_config(mut self, config: HybridSearchConfig) -> Self {
		self.config = config;
		self
	}

	/// Index symbols for both keyword and semantic search
	pub fn index_symbols(&mut self, symbols: &[Symbol]) -> RetrievalResult<()> {
		// index in Tantivy
		self.keyword_index
			.index_symbols(symbols)
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

		// generate embeddings via daemon
		let texts: Vec<String> = symbols
			.iter()
			.map(|s| format!("{} {} {}", s.kind, s.name, s.signature.as_deref().unwrap_or("")))
			.collect();

		let embeddings = self.daemon_client.embed(texts)?;

		// add to vector store
		for (i, (symbol, embedding)) in symbols.iter().zip(embeddings.iter()).enumerate() {
			let point = VectorPoint {
				id: i as u64,
				vector: embedding.clone(),
				file_path: symbol.location.file.clone(),
				line: symbol.location.line,
				symbol_name: symbol.name.clone(),
				symbol_kind: symbol.kind.to_string(),
			};
			self.vector_store.insert(point);
		}

		// build HNSW index
		self.vector_store.build_index()?;

		Ok(())
	}

	/// Perform hybrid search with fixed weights
	pub fn search(&self, query: &str, limit: usize) -> RetrievalResult<Vec<HybridSearchResult>> {
		self.search_with_weights(query, limit, self.config.keyword_weight, self.config.semantic_weight)
	}

	/// Perform hybrid search with adaptive weights based on query type.
	/// Favors BM25 for symbol lookups, semantic for conceptual queries.
	pub fn search_adaptive(&self, query: &str, limit: usize) -> RetrievalResult<Vec<HybridSearchResult>> {
		let weights = adaptive::compute_weights(query);
		self.search_with_weights(query, limit, weights.keyword_weight, weights.semantic_weight)
	}

	/// Perform hybrid search using a SearchSpec from query expansion.
	/// Uses intent to adjust symbol kind boosts (e.g., deprioritize fields for Understand).
	pub fn search_with_spec(
		&self,
		spec: &SearchSpec,
		limit: usize,
	) -> RetrievalResult<Vec<HybridSearchResult>> {
		let weights = adaptive::compute_weights(&spec.original_query); // compute adaptive weights
		let candidates = self.config.candidates_per_source; // number of candidates per source
		let query = &spec.original_query; // query string for search

		// keyword search
		let keyword_hits = self
			.keyword_index
			.search(query, candidates)
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

		// semantic search - get query embedding
		let query_embeddings = self.daemon_client.embed(vec![query.to_string()])?;
		let query_embedding = query_embeddings
			.first()
			.ok_or_else(|| RetrievalError::Embedding("no embedding returned".to_string()))?;

		let semantic_hits = self.vector_store.search(query_embedding, candidates);

		// convert to ranked items
		let keyword_ranked = self.to_keyword_ranked(keyword_hits);
		let semantic_ranked = self.to_semantic_ranked(semantic_hits);

		// fuse results with intent-aware weights
		let fused = self.fuse_with_weights_and_intent(
			keyword_ranked,
			semantic_ranked,
			weights.keyword_weight,
			weights.semantic_weight,
			query,
			&spec.intent,
		);

		Ok(fused.into_iter().take(limit).collect())
	}

	/// Convert keyword hits to ranked items
	fn to_keyword_ranked(&self, hits: Vec<SearchHit>) -> Vec<RankedItem<SearchHit>> {
		hits.into_iter()
			.enumerate()
			.map(|(i, hit)| RankedItem {
				item: hit.clone(),
				rank: i + 1,
				score: hit.score,
			})
			.collect()
	}

	/// Convert semantic hits to ranked items
	fn to_semantic_ranked(&self, hits: Vec<VectorSearchResult>) -> Vec<RankedItem<VectorSearchResult>> {
		hits.into_iter()
			.enumerate()
			.map(|(i, result)| RankedItem {
				item: result.clone(),
				rank: i + 1,
				score: result.distance,
			})
			.collect()
	}

	/// Perform hybrid search with custom weights
	pub fn search_with_weights(
		&self,
		query: &str,
		limit: usize,
		keyword_weight: f32,
		semantic_weight: f32,
	) -> RetrievalResult<Vec<HybridSearchResult>> {
		let candidates = self.config.candidates_per_source;

		// keyword search
		let keyword_hits = self
			.keyword_index
			.search(query, candidates)
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

		// semantic search - get query embedding
		let query_embeddings = self.daemon_client.embed(vec![query.to_string()])?;
		let query_embedding = query_embeddings
			.first()
			.ok_or_else(|| RetrievalError::Embedding("no embedding returned".to_string()))?;

		let semantic_hits = self.vector_store.search(query_embedding, candidates);

		// convert to ranked items
		let keyword_ranked: Vec<RankedItem<SearchHit>> = keyword_hits
			.into_iter()
			.enumerate()
			.map(|(i, hit)| RankedItem {
				item: hit.clone(),
				rank: i + 1,
				score: hit.score,
			})
			.collect();

		let semantic_ranked: Vec<RankedItem<VectorSearchResult>> = semantic_hits
			.into_iter()
			.enumerate()
			.map(|(i, result)| RankedItem {
				item: result.clone(),
				rank: i + 1,
				score: result.distance,
			})
			.collect();

		// fuse results with query-aware weights
		let fused = self.fuse_with_weights(keyword_ranked, semantic_ranked, keyword_weight, semantic_weight, query);

		// take top results
		Ok(fused.into_iter().take(limit).collect())
	}

	/// Fuse keyword and semantic results with default config weights
	fn fuse_search_results(
		&self,
		keyword_results: Vec<RankedItem<SearchHit>>,
		semantic_results: Vec<RankedItem<VectorSearchResult>>,
		query: &str,
	) -> Vec<HybridSearchResult> {
		self.fuse_with_weights(
			keyword_results,
			semantic_results,
			self.config.keyword_weight,
			self.config.semantic_weight,
			query,
		)
	}

	/// Fuse keyword and semantic results with custom weights
	/// Uses query-aware boost to reduce documentation priority for code queries
	fn fuse_with_weights(
		&self,
		keyword_results: Vec<RankedItem<SearchHit>>,
		semantic_results: Vec<RankedItem<VectorSearchResult>>,
		keyword_weight: f32,
		semantic_weight: f32,
		query: &str,
	) -> Vec<HybridSearchResult> {
		use std::collections::HashMap;

		let mut results_by_key: HashMap<String, HybridSearchResult> = HashMap::new();

		// process keyword results
		for ranked in keyword_results {
			let key = format!(
				"{}:{}",
				ranked.item.symbol.name, ranked.item.symbol.location.line
			);

			// Calculate combined boost from document type and symbol kind
			let doc_type = DocumentType::from_path(&ranked.item.symbol.location.file); // document classification
			let doc_boost = doc_type.boost_factor_for_query(query); // query-aware boost for source code vs notes
			let kind_boost = ranked.item.symbol.kind.boost_factor(); // boost for functions vs fields
			let combined_boost = doc_boost * kind_boost; // combined multiplier

			// Apply boost to RRF score
			let rrf = fusion::rrf_score(ranked.rank, self.config.rrf_k) * keyword_weight * combined_boost;

			results_by_key
				.entry(key.clone())
				.and_modify(|r| {
					r.rrf_score += rrf;
					r.keyword_rank = Some(ranked.rank);
					r.keyword_score = Some(ranked.score);
				})
				.or_insert(HybridSearchResult {
					symbol: ranked.item.symbol,
					rrf_score: rrf,
					keyword_rank: Some(ranked.rank),
					semantic_rank: None,
					keyword_score: Some(ranked.score),
					semantic_distance: None,
					rerank_score: None,
				});
		}

		// process semantic results
		for ranked in semantic_results {
			let key = format!("{}:{}", ranked.item.point.symbol_name, ranked.item.point.line);

			// Calculate combined boost from document type and symbol kind
			let doc_type = DocumentType::from_path(&ranked.item.point.file_path); // document classification
			let doc_boost = doc_type.boost_factor_for_query(query); // query-aware boost for source code vs notes
			let symbol_kind = parse_symbol_kind(&ranked.item.point.symbol_kind); // parse kind from string
			let kind_boost = symbol_kind.boost_factor(); // boost for functions vs fields
			let combined_boost = doc_boost * kind_boost; // combined multiplier

			// Apply boost to RRF score
			let rrf = fusion::rrf_score(ranked.rank, self.config.rrf_k) * semantic_weight * combined_boost;

			results_by_key
				.entry(key.clone())
				.and_modify(|r| {
					r.rrf_score += rrf;
					r.semantic_rank = Some(ranked.rank);
					r.semantic_distance = Some(ranked.score);
				})
				.or_insert_with(|| {
					// create symbol from vector point
					let point = &ranked.item.point;
					let symbol = Symbol::new(
						point.symbol_name.clone(),
						symbol_kind,
						crate::indexer::CodeLocation::new(
							point.file_path.clone(),
							point.line,
							1,
							0,
							0,
						),
					);

					HybridSearchResult {
						symbol,
						rrf_score: rrf,
						keyword_rank: None,
						semantic_rank: Some(ranked.rank),
						keyword_score: None,
						semantic_distance: Some(ranked.score),
						rerank_score: None,
					}
				});
		}

		// sort by RRF score
		let mut results: Vec<_> = results_by_key.into_values().collect();
		results.sort_by(|a, b| b.rrf_score.partial_cmp(&a.rrf_score).unwrap());

		results
	}

	/// Fuse results with intent-aware symbol kind boost.
	/// Uses boost_factor_for_intent to adjust symbol priority based on query intent.
	fn fuse_with_weights_and_intent(
		&self,
		keyword_results: Vec<RankedItem<SearchHit>>,
		semantic_results: Vec<RankedItem<VectorSearchResult>>,
		keyword_weight: f32,
		semantic_weight: f32,
		query: &str,
		intent: &QueryIntent,
	) -> Vec<HybridSearchResult> {
		use std::collections::HashMap;

		let mut results_by_key: HashMap<String, HybridSearchResult> = HashMap::new();

		// process keyword results with intent-aware boost
		for ranked in keyword_results {
			let result = self.process_keyword_with_intent(&ranked, keyword_weight, query, intent);
			let key = format!(
				"{}:{}",
				ranked.item.symbol.name, ranked.item.symbol.location.line
			);

			results_by_key
				.entry(key)
				.and_modify(|r| {
					r.rrf_score += result.rrf_score;
					r.keyword_rank = result.keyword_rank;
					r.keyword_score = result.keyword_score;
				})
				.or_insert(result);
		}

		// process semantic results with intent-aware boost
		for ranked in semantic_results {
			let (key, result) = self.process_semantic_with_intent(&ranked, semantic_weight, query, intent);

			results_by_key
				.entry(key)
				.and_modify(|r| {
					r.rrf_score += result.rrf_score;
					r.semantic_rank = result.semantic_rank;
					r.semantic_distance = result.semantic_distance;
				})
				.or_insert(result);
		}

		// sort by RRF score descending
		let mut results: Vec<_> = results_by_key.into_values().collect();
		results.sort_by(|a, b| b.rrf_score.partial_cmp(&a.rrf_score).unwrap());

		results
	}

	/// Process a single keyword result with intent-aware boost
	fn process_keyword_with_intent(
		&self,
		ranked: &RankedItem<SearchHit>,
		keyword_weight: f32,
		query: &str,
		intent: &QueryIntent,
	) -> HybridSearchResult {
		let doc_type = DocumentType::from_path(&ranked.item.symbol.location.file);
		let doc_query_boost = doc_type.boost_factor_for_query(query); // query-aware doc boost
		let doc_intent_boost = doc_type.boost_factor_for_intent(intent); // intent-aware doc boost
		let kind_boost = ranked.item.symbol.kind.boost_factor_for_intent(intent); // intent-aware kind boost
		let combined_boost = doc_query_boost * doc_intent_boost * kind_boost;

		let rrf = fusion::rrf_score(ranked.rank, self.config.rrf_k) * keyword_weight * combined_boost;

		HybridSearchResult {
			symbol: ranked.item.symbol.clone(),
			rrf_score: rrf,
			keyword_rank: Some(ranked.rank),
			semantic_rank: None,
			keyword_score: Some(ranked.score),
			semantic_distance: None,
			rerank_score: None,
		}
	}

	/// Process a single semantic result with intent-aware boost
	fn process_semantic_with_intent(
		&self,
		ranked: &RankedItem<VectorSearchResult>,
		semantic_weight: f32,
		query: &str,
		intent: &QueryIntent,
	) -> (String, HybridSearchResult) {
		let doc_type = DocumentType::from_path(&ranked.item.point.file_path);
		let doc_query_boost = doc_type.boost_factor_for_query(query); // query-aware doc boost
		let doc_intent_boost = doc_type.boost_factor_for_intent(intent); // intent-aware doc boost
		let symbol_kind = parse_symbol_kind(&ranked.item.point.symbol_kind);
		let kind_boost = symbol_kind.boost_factor_for_intent(intent); // intent-aware kind boost
		let combined_boost = doc_query_boost * doc_intent_boost * kind_boost;

		let rrf = fusion::rrf_score(ranked.rank, self.config.rrf_k) * semantic_weight * combined_boost;

		let key = format!("{}:{}", ranked.item.point.symbol_name, ranked.item.point.line);

		let point = &ranked.item.point;
		let symbol = Symbol::new(
			point.symbol_name.clone(),
			symbol_kind,
			crate::indexer::CodeLocation::new(point.file_path.clone(), point.line, 1, 0, 0),
		);

		let result = HybridSearchResult {
			symbol,
			rrf_score: rrf,
			keyword_rank: None,
			semantic_rank: Some(ranked.rank),
			keyword_score: None,
			semantic_distance: Some(ranked.score),
			rerank_score: None,
		};

		(key, result)
	}

	/// Get reference to keyword index
	pub fn keyword_index(&self) -> &SearchIndex {
		&self.keyword_index
	}

	/// Get reference to vector store
	pub fn vector_store(&self) -> &VectorStore {
		&self.vector_store
	}

	/// Persist indices to disk
	pub fn persist(&self) -> RetrievalResult<()> {
		self.vector_store.persist()
	}
}

/// Parse symbol kind from string
fn parse_symbol_kind(kind: &str) -> crate::indexer::SymbolKind {
	match kind.to_lowercase().as_str() {
		"fn" | "function" => crate::indexer::SymbolKind::Function,
		"struct" => crate::indexer::SymbolKind::Struct,
		"enum" => crate::indexer::SymbolKind::Enum,
		"trait" => crate::indexer::SymbolKind::Trait,
		"impl" => crate::indexer::SymbolKind::Impl,
		"method" => crate::indexer::SymbolKind::Method,
		"const" | "constant" => crate::indexer::SymbolKind::Constant,
		"static" => crate::indexer::SymbolKind::Static,
		"type" => crate::indexer::SymbolKind::TypeAlias,
		"mod" | "module" => crate::indexer::SymbolKind::Module,
		"macro" => crate::indexer::SymbolKind::Macro,
		"field" => crate::indexer::SymbolKind::Field,
		"variant" => crate::indexer::SymbolKind::EnumVariant,
		_ => crate::indexer::SymbolKind::Function,
	}
}

impl Default for HybridSearch {
	fn default() -> Self {
		Self::new().expect("failed to create hybrid search")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::indexer::SymbolKind;
	use std::path::PathBuf;

	/// Test that source code files get higher boost than benchmark files
	#[test]
	fn test_source_code_boosted_over_benchmark() {
		let source_path = PathBuf::from("src/main.rs"); // source code file
		let benchmark_path = PathBuf::from("notes/benchmarks/perf.md"); // benchmark file

		let source_doc_type = DocumentType::from_path(&source_path);
		let benchmark_doc_type = DocumentType::from_path(&benchmark_path);

		let source_boost = source_doc_type.boost_factor();
		let benchmark_boost = benchmark_doc_type.boost_factor();

		// Source code should have higher boost than benchmark
		assert!(
			source_boost > benchmark_boost,
			"Source boost ({}) should be > benchmark boost ({})",
			source_boost,
			benchmark_boost
		);
	}

	/// Test that functions get higher boost than fields
	#[test]
	fn test_function_boosted_over_field() {
		let function_boost = SymbolKind::Function.boost_factor(); // function boost
		let field_boost = SymbolKind::Field.boost_factor(); // field boost

		// Functions should have higher boost than fields
		assert!(
			function_boost > field_boost,
			"Function boost ({}) should be > field boost ({})",
			function_boost,
			field_boost
		);
	}

	/// Test combined boost calculation
	#[test]
	fn test_combined_boost_calculation() {
		let source_path = PathBuf::from("src/lib.rs"); // source code
		let notes_path = PathBuf::from("notes/design.md"); // notes file

		// Source code function should beat notes field
		let source_func_boost = DocumentType::from_path(&source_path).boost_factor()
			* SymbolKind::Function.boost_factor();
		let notes_field_boost = DocumentType::from_path(&notes_path).boost_factor()
			* SymbolKind::Field.boost_factor();

		assert!(
			source_func_boost > notes_field_boost,
			"Source function ({}) should beat notes field ({})",
			source_func_boost,
			notes_field_boost
		);
	}

	/// Test parse_symbol_kind function
	#[test]
	fn test_parse_symbol_kind() {
		assert_eq!(parse_symbol_kind("fn"), SymbolKind::Function);
		assert_eq!(parse_symbol_kind("function"), SymbolKind::Function);
		assert_eq!(parse_symbol_kind("struct"), SymbolKind::Struct);
		assert_eq!(parse_symbol_kind("field"), SymbolKind::Field);
		assert_eq!(parse_symbol_kind("method"), SymbolKind::Method);
		assert_eq!(parse_symbol_kind("const"), SymbolKind::Constant);
	}

	/// Test query-aware boost reduces documentation boost for code queries
	#[test]
	fn test_query_aware_boost_reduces_doc() {
		let doc_path = PathBuf::from("doc/api.md"); // documentation file
		let query_impl = "show me the implementation"; // implementation keyword
		let query_where = "where is AuthService"; // no implementation keyword

		let doc_type = DocumentType::from_path(&doc_path);

		// Implementation query should reduce doc boost by 0.5x
		let impl_boost = doc_type.boost_factor_for_query(query_impl);
		let where_boost = doc_type.boost_factor_for_query(query_where);

		assert_eq!(impl_boost, 0.5); // 1.0 * 0.5
		assert_eq!(where_boost, 1.0); // unchanged
		assert!(where_boost > impl_boost, "Where query should not reduce boost");
	}

	/// Test query-aware boost keeps source code unchanged
	#[test]
	fn test_query_aware_boost_source_unchanged() {
		let source_path = PathBuf::from("src/main.rs"); // source code file
		let query_impl = "show me the implementation"; // implementation keyword

		let source_type = DocumentType::from_path(&source_path);
		let base_boost = source_type.boost_factor();
		let impl_boost = source_type.boost_factor_for_query(query_impl);

		// Source code boost should remain unchanged
		assert_eq!(impl_boost, base_boost);
		assert_eq!(impl_boost, 1.5);
	}

	/// Test combined boost uses query-aware document boost
	#[test]
	fn test_combined_boost_query_aware() {
		let source_path = PathBuf::from("src/lib.rs"); // source code
		let notes_path = PathBuf::from("/project/notes/design.md"); // notes file (absolute path)
		let query = "show me the implementation"; // implementation keyword

		// Source code function boost: 1.5 * 1.4 = 2.1 (unchanged by query)
		let source_boost = DocumentType::from_path(&source_path).boost_factor_for_query(query)
			* SymbolKind::Function.boost_factor();

		// Notes function boost: 0.7 * 0.5 * 1.4 = 0.49 (reduced by query)
		let notes_boost = DocumentType::from_path(&notes_path).boost_factor_for_query(query)
			* SymbolKind::Function.boost_factor();

		// Source should beat notes by large margin with implementation query
		// 2.1 / 0.49 = 4.28x
		assert!(
			source_boost > notes_boost * 4.0,
			"Source ({}) should beat notes ({}) by 4x+ for impl query",
			source_boost,
			notes_boost
		);
	}

	/// Test intent-aware boost: Understand intent boosts functions over fields
	#[test]
	fn test_intent_boost_understand_functions() {
		let func_base = SymbolKind::Function.boost_factor(); // 1.4
		let field_base = SymbolKind::Field.boost_factor(); // 0.7

		let func_understand = SymbolKind::Function.boost_factor_for_intent(&QueryIntent::Understand);
		let field_understand = SymbolKind::Field.boost_factor_for_intent(&QueryIntent::Understand);

		// Function should be boosted 1.5x for Understand (1.4 * 1.5 = 2.1)
		assert!((func_understand - func_base * 1.5).abs() < 0.001);

		// Field should be reduced to 0.3x for Understand (0.7 * 0.3 = 0.21)
		assert!((field_understand - field_base * 0.3).abs() < 0.001);

		// Function should be much higher than field for Understand
		assert!(
			func_understand > field_understand * 4.0,
			"Function ({}) should be 4x+ field ({}) for Understand",
			func_understand,
			field_understand
		);
	}

	/// Test intent-aware boost: FindDefinition intent boosts structs/enums/traits
	#[test]
	fn test_intent_boost_find_definition_types() {
		let struct_base = SymbolKind::Struct.boost_factor(); // 1.3
		let enum_base = SymbolKind::Enum.boost_factor(); // 1.3
		let trait_base = SymbolKind::Trait.boost_factor(); // 1.3

		let struct_def = SymbolKind::Struct.boost_factor_for_intent(&QueryIntent::FindDefinition);
		let enum_def = SymbolKind::Enum.boost_factor_for_intent(&QueryIntent::FindDefinition);
		let trait_def = SymbolKind::Trait.boost_factor_for_intent(&QueryIntent::FindDefinition);

		// All type definitions should be boosted 1.1x for FindDefinition
		assert!((struct_def - struct_base * 1.1).abs() < 0.001);
		assert!((enum_def - enum_base * 1.1).abs() < 0.001);
		assert!((trait_def - trait_base * 1.1).abs() < 0.001);
	}

	/// Test intent-aware boost: Search intent uses base boost
	#[test]
	fn test_intent_boost_search_uses_base() {
		let func_base = SymbolKind::Function.boost_factor();
		let field_base = SymbolKind::Field.boost_factor();
		let struct_base = SymbolKind::Struct.boost_factor();

		let func_search = SymbolKind::Function.boost_factor_for_intent(&QueryIntent::Search);
		let field_search = SymbolKind::Field.boost_factor_for_intent(&QueryIntent::Search);
		let struct_search = SymbolKind::Struct.boost_factor_for_intent(&QueryIntent::Search);

		// All should be unchanged for Search intent
		assert!((func_search - func_base).abs() < 0.001);
		assert!((field_search - field_base).abs() < 0.001);
		assert!((struct_search - struct_base).abs() < 0.001);
	}

	/// Test combined doc + intent boost for Understand query
	#[test]
	fn test_combined_doc_and_intent_boost() {
		let source_path = PathBuf::from("src/lib.rs");
		let query = "show me the implementation";

		// Combined boost: doc_boost (query-aware) * kind_boost (intent-aware)
		let func_boost = DocumentType::from_path(&source_path).boost_factor_for_query(query)
			* SymbolKind::Function.boost_factor_for_intent(&QueryIntent::Understand);
		let field_boost = DocumentType::from_path(&source_path).boost_factor_for_query(query)
			* SymbolKind::Field.boost_factor_for_intent(&QueryIntent::Understand);

		// Function should dominate field with both boosts applied
		// func: 1.5 * (1.4 * 1.2) = 1.5 * 1.68 = 2.52
		// field: 1.5 * (0.7 * 0.5) = 1.5 * 0.35 = 0.525
		assert!(
			func_boost > field_boost * 4.0,
			"Function ({}) should be 4x+ field ({}) with combined boosts",
			func_boost,
			field_boost
		);
	}
}
