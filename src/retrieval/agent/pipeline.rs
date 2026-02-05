//! Retrieval Pipeline
//!
//! Orchestrates the full code retrieval workflow from natural language
//! query to formatted context output.

use std::sync::mpsc;
use std::time::Duration;

use crate::indexer::{DocumentType, IndexManager, IndexState, SemanticGraph, Symbol, SymbolReference, TrigramIndex};
use crate::retrieval::context::{ContextConfig, ContextExpander};
use crate::retrieval::daemon::protocol::SearchSpec;
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::docgen::DocStore;
use crate::retrieval::hybrid::{HybridSearch, HybridSearchConfig, HybridSearchResult, TripleHybridSearch};
use crate::retrieval::query::{TieredConfig, TieredQueryExpander, fallback_parse};
use crate::retrieval::{RetrievalError, RetrievalResult};

use super::output::{CodeResult, DocResult, NotesResult, StructuredOutput};
use super::RetrievalOutput;

/// Configuration for the retrieval pipeline
#[derive(Debug, Clone)]
pub struct PipelineConfig {
	/// max results to return
	pub max_results: usize,
	/// max tokens in output
	pub max_tokens: usize,
	/// enable query expansion via LLM
	pub expand_query: bool,
	/// enable tiered expansion (fast-path for explicit queries)
	pub tiered_expansion: bool,
	/// confidence threshold for fast-path (0.0 - 1.0)
	pub fast_path_threshold: f32,
	/// importance threshold for validated symbols (0.0 - 1.0)
	pub importance_threshold: f32,
	/// enable semantic search
	pub semantic_search: bool,
	/// enable reranking
	pub rerank: bool,
	/// enable context expansion
	pub expand_context: bool,
	/// path to index (defaults to current directory)
	pub project_path: String,
	/// enable persistent index caching (reduces warm start from ~4.5s to <500ms)
	pub enable_persistence: bool,
	/// use daemon-side project caching (faster for repeated queries)
	pub use_daemon_cache: bool,
	/// include usage count for each symbol in output
	pub include_usage_counts: bool,
	/// use generated documentation for enhanced context
	pub use_doc_context: bool,
	/// use structured pipeline (separate code, doc, notes) instead of unified
	pub use_triple_pipeline: bool,
	/// max code results for structured pipeline
	pub max_code_results: usize,
	/// max doc results for structured pipeline
	pub max_doc_results: usize,
	/// max notes results for structured pipeline
	pub max_notes_results: usize,
	/// max lines to include per file (full content)
	pub max_lines_per_file: usize,
	/// RRF score threshold for filtering low-relevance results
	pub rrf_score_threshold: f32,
	/// minimum results to keep per content type regardless of threshold
	pub min_results_per_type: usize,
}

impl Default for PipelineConfig {
	fn default() -> Self {
		Self {
			max_results: 20,
			max_tokens: 8000,
			expand_query: true,
			tiered_expansion: true,
			fast_path_threshold: 0.7,
			importance_threshold: 0.5,
			semantic_search: true,
			rerank: true,
			expand_context: true,
			project_path: ".".to_string(),
			enable_persistence: true,
			use_daemon_cache: true,
			include_usage_counts: true,
			use_doc_context: true,
			use_triple_pipeline: true,
			max_code_results: 10,
			max_doc_results: 5,
			max_notes_results: 3,
			max_lines_per_file: 500,
			rrf_score_threshold: 0.015,
			min_results_per_type: 1,
		}
	}
}

/// Result from a pipeline execution
#[derive(Debug)]
pub struct PipelineResult {
	/// the search specification used
	pub search_spec: SearchSpec,
	/// raw search results
	pub search_results: Vec<HybridSearchResult>,
	/// formatted XML output
	pub xml_output: String,
	/// token count
	pub token_count: usize,
}

/// The main retrieval pipeline orchestrator
pub struct RetrievalPipeline {
	/// configuration
	config: PipelineConfig,
	/// daemon client for ML operations
	daemon: DaemonClient,
	/// indexed symbols (cached after first run)
	symbols: Option<Vec<Symbol>>,
	/// semantic graph (cached after first run)
	graph: Option<SemanticGraph>,
	/// hybrid search instance (unified pipeline)
	hybrid: Option<HybridSearch>,
	/// triple hybrid search (code, doc, notes pipelines)
	triple_hybrid: Option<TripleHybridSearch>,
	/// trigram index for fast pre-filtering
	trigram_index: Option<TrigramIndex>,
	/// documentation store for enhanced context
	doc_store: Option<DocStore>,
}

impl RetrievalPipeline {
	/// Create a new pipeline with default config
	pub fn new() -> Self {
		Self {
			config: PipelineConfig::default(),
			daemon: DaemonClient::new(),
			symbols: None,
			graph: None,
			hybrid: None,
			triple_hybrid: None,
			trigram_index: None,
			doc_store: None,
		}
	}

	/// Create pipeline with custom config
	pub fn with_config(config: PipelineConfig) -> Self {
		Self {
			config,
			daemon: DaemonClient::new(),
			symbols: None,
			graph: None,
			hybrid: None,
			triple_hybrid: None,
			trigram_index: None,
			doc_store: None,
		}
	}

	/// Initialize the pipeline (index project, build search indices)
	/// Uses persistent caching when enabled for faster warm starts
	pub fn initialize(&mut self) -> RetrievalResult<()> {
		// Use local indexing directly - daemon caching adds complexity
		// without significant benefit for single-query CLI usage
		self.initialize_local()
	}

	/// Initialize via daemon-side caching (fastest path)
	fn initialize_via_daemon(&mut self) -> RetrievalResult<()> {
		let project_path = std::fs::canonicalize(&self.config.project_path)
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;
		let project_str = project_path.display().to_string();

		// Check if project is already cached in daemon
		let (cached, symbol_count, _last_indexed) = self.daemon.project_status(&project_str)?;

		if cached {
			eprintln!("[pipeline] Using daemon cache ({} symbols)", symbol_count);
		} else {
			eprintln!("[pipeline] Indexing project via daemon: {}", project_str);
			let (count, was_cached, time_ms) = self.daemon.index_project(&project_str, false)?;
			if was_cached {
				eprintln!("[pipeline] Daemon cache hit ({} symbols)", count);
			} else {
				eprintln!("[pipeline] Daemon indexed {} symbols in {}ms", count, time_ms);
			}
		}

		// We still need local graph for context expansion and hybrid search
		let manager = IndexManager::new()
			.with_semantic_analysis()
			.with_reference_extraction();
		let manager = if self.config.enable_persistence {
			manager.with_persistence()
		} else {
			manager
		};

		let result = manager
			.index_project(&self.config.project_path)
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

		self.symbols = Some(result.symbols.clone());
		self.graph = result.semantic_graph;

		// Initialize hybrid search (needed for local search)
		if self.config.semantic_search {
			self.initialize_hybrid_search(&result.symbols)?;
		}

		Ok(())
	}

	/// Initialize with local indexing (fallback path)
	fn initialize_local(&mut self) -> RetrievalResult<()> {
		eprintln!("[pipeline] Indexing project: {}", self.config.project_path);

		// Build IndexManager with optional persistence
		// Enable reference extraction for cross-file usage tracking
		let manager = IndexManager::new()
			.with_semantic_analysis()
			.with_reference_extraction();
		let manager = if self.config.enable_persistence {
			eprintln!("[pipeline] Persistence enabled (incremental indexing)");
			manager.with_persistence()
		} else {
			manager
		};

		let result = manager
			.index_project(&self.config.project_path)
			.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

		// Log incremental vs full index info
		if result.incremental {
			let changes = result.changes.as_ref().map(|c| c.total_changes()).unwrap_or(0);
			eprintln!("[pipeline] Incremental index: {} changes", changes);
		}
		eprintln!("[pipeline] Found {} symbols", result.symbols.len());

		// Store symbols
		self.symbols = Some(result.symbols.clone());

		// Clone project path to avoid borrow issues
		let project_path_owned = std::path::PathBuf::from(&self.config.project_path);
		let project_path = project_path_owned.as_path();

		// Handle semantic graph with reference persistence
		self.graph = self.build_graph_with_persistence(
			result.semantic_graph,
			&result.references,
			&result.symbols,
			project_path,
			result.incremental,
		);

		// Build trigram index for fast pre-filtering
		self.trigram_index = self.build_trigram_index(
			&result.file_results,
			project_path,
			result.incremental,
		);

		// Build hybrid search index if semantic search enabled
		if self.config.semantic_search {
			if self.config.use_triple_pipeline {
				self.initialize_triple_hybrid(&result.symbols)?;
			} else {
				self.initialize_hybrid_search(&result.symbols)?;
			}
		}

		// Load doc store if doc context is enabled
		if self.config.use_doc_context {
			self.load_doc_store(project_path);
		}

		Ok(())
	}

	/// Load documentation store for enhanced context
	fn load_doc_store(&mut self, project_path: &std::path::Path) {
		match DocStore::load(project_path) {
			Ok(store) => {
				if store.is_ready() {
					let stats = store.stats();
					eprintln!(
						"[pipeline] Loaded doc store ({} entries, {}% ready)",
						stats.total,
						(stats.completion_percent() as u32)
					);
					self.doc_store = Some(store);
				} else {
					eprintln!("[pipeline] Doc store not ready (run 'ch-cli docs generate')");
				}
			}
			Err(_) => {
				eprintln!("[pipeline] No doc store found (run 'ch-cli docs generate')");
			}
		}
	}

	/// Get documentation for a symbol if available
	pub fn get_doc_for_symbol(&self, symbol_name: &str) -> Option<String> {
		let store = self.doc_store.as_ref()?;
		let entry = store.get_by_name(symbol_name)?;

		Some(entry.combined_doc())
	}

	/// Check if doc store is available and ready
	pub fn has_doc_context(&self) -> bool {
		self.doc_store.as_ref().map(|s| s.is_ready()).unwrap_or(false)
	}

	/// Build or load trigram index for fast text pre-filtering
	fn build_trigram_index(
		&self,
		file_results: &[crate::indexer::FileResult],
		project_path: &std::path::Path,
		incremental: bool,
	) -> Option<TrigramIndex> {
		if !self.config.enable_persistence {
			return None;
		}

		let trigram_file = IndexState::trigram_file(project_path);

		// If incremental with no changes, try to load from cache
		if incremental && file_results.is_empty() {
			match TrigramIndex::load(&trigram_file) {
				Ok(index) => {
					let stats = index.stats();
					eprintln!(
						"[pipeline] Loaded trigram index ({} trigrams, {} files)",
						stats.trigram_count, stats.file_count
					);
					return Some(index);
				}
				Err(e) => {
					eprintln!("[pipeline] Warning: failed to load trigram index: {}", e);
				}
			}
		}

		// Build fresh trigram index from file results
		if !file_results.is_empty() {
			let mut index = TrigramIndex::new();

			for file_result in file_results {
				if file_result.error.is_none() {
					if let Ok(content) = std::fs::read_to_string(&file_result.path) {
						index.index_file(&file_result.path, &content);
					}
				}
			}

			let stats = index.stats();
			eprintln!(
				"[pipeline] Built trigram index ({} trigrams, {} files)",
				stats.trigram_count, stats.file_count
			);

			// Save to disk
			if let Err(e) = index.save(&trigram_file) {
				eprintln!("[pipeline] Warning: failed to save trigram index: {}", e);
			}

			return Some(index);
		}

		None
	}

	/// Build semantic graph with reference persistence
	fn build_graph_with_persistence(
		&self,
		graph: Option<SemanticGraph>,
		new_refs: &[crate::indexer::parser::ExtractedReference],
		symbols: &[Symbol],
		project_path: &std::path::Path,
		incremental: bool,
	) -> Option<SemanticGraph> {
		if !self.config.enable_persistence {
			return graph;
		}

		// If we have new references, save them
		if !new_refs.is_empty() {
			let refs: Vec<SymbolReference> = new_refs
				.iter()
				.map(|r| SymbolReference {
					name: r.name.clone(),
					location: r.location.clone(),
					context: r.context.clone(),
				})
				.collect();

			if let Err(e) = self.save_references(&refs, project_path) {
				eprintln!("[pipeline] Warning: failed to save references: {}", e);
			} else {
				eprintln!("[pipeline] Saved {} references to cache", refs.len());
			}
		}

		// If incremental with 0 changes, load references from cache
		if incremental && new_refs.is_empty() {
			match IndexState::load_references(project_path) {
				Ok(cached_refs) if !cached_refs.is_empty() => {
					eprintln!("[pipeline] Loaded {} references from cache", cached_refs.len());

					// Rebuild graph with cached references
					let mut new_graph = SemanticGraph::new();
					new_graph.add_symbols(symbols);
					for ref_item in cached_refs {
						new_graph.add_reference(ref_item);
					}
					return Some(new_graph);
				}
				Ok(_) => {
					eprintln!("[pipeline] No cached references found");
				}
				Err(e) => {
					eprintln!("[pipeline] Warning: failed to load cached references: {}", e);
				}
			}
		}

		graph
	}

	/// Save references to cache
	fn save_references(
		&self,
		refs: &[SymbolReference],
		project_path: &std::path::Path,
	) -> std::io::Result<()> {
		let index_dir = IndexState::index_dir(project_path);
		std::fs::create_dir_all(&index_dir)?;

		let refs_file = IndexState::refs_file(project_path);
		let content = serde_json::to_string(refs)
			.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
		std::fs::write(&refs_file, content)?;

		Ok(())
	}

	/// Initialize hybrid search with optional persistence
	fn initialize_hybrid_search(&mut self, symbols: &[Symbol]) -> RetrievalResult<()> {
		let project_path = std::path::Path::new(&self.config.project_path);
		let index_dir = IndexState::index_dir(project_path); // .ch-index directory
		let tantivy_path = index_dir.join("tantivy");
		let vector_path = index_dir.join("vectors.json");

		// Try to load from persistent paths if enabled and they exist
		if self.config.enable_persistence && tantivy_path.exists() && vector_path.exists() {
			eprintln!("[pipeline] Loading cached hybrid search index...");
			match HybridSearch::with_paths(&tantivy_path, &vector_path) {
				Ok(hybrid) => {
					eprintln!("[pipeline] Hybrid index loaded from cache");
					self.hybrid = Some(hybrid);
					return Ok(());
				}
				Err(e) => {
					eprintln!("[pipeline] Cache load failed, rebuilding: {}", e);
				}
			}
		}

		// Build fresh index
		eprintln!("[pipeline] Building hybrid search index...");
		let mut hybrid = if self.config.enable_persistence {
			// Create with paths for persistence
			HybridSearch::with_paths(&tantivy_path, &vector_path)?
		} else {
			HybridSearch::new()?
		};

		hybrid.index_symbols(symbols)?;

		// Persist vectors if enabled
		if self.config.enable_persistence {
			if let Err(e) = hybrid.persist() {
				eprintln!("[pipeline] Warning: failed to persist vectors: {}", e);
			}
		}

		self.hybrid = Some(hybrid);
		Ok(())
	}

	/// Initialize structured hybrid search (code, doc, notes pipelines)
	fn initialize_triple_hybrid(&mut self, symbols: &[Symbol]) -> RetrievalResult<()> {
		let project_path = std::path::Path::new(&self.config.project_path);
		let index_dir = IndexState::index_dir(project_path);

		eprintln!("[pipeline] Building hybrid search index...");

		// Build HybridSearchConfig from PipelineConfig threshold values
		let hybrid_config = HybridSearchConfig {
			rrf_score_threshold: self.config.rrf_score_threshold,
			min_results_per_type: self.config.min_results_per_type,
			..HybridSearchConfig::default()
		};

		let mut hybrid = if self.config.enable_persistence {
			TripleHybridSearch::with_persistence(&index_dir, self.daemon.clone())?
				.with_config(hybrid_config)
		} else {
			TripleHybridSearch::new(self.daemon.clone())?
				.with_config(hybrid_config)
		};

		let stats = hybrid.index_symbols(symbols)?;
		eprintln!(
			"[pipeline] Indexed: {} code, {} doc, {} notes",
			stats.code_keyword_count, stats.doc_keyword_count, stats.notes_keyword_count
		);

		if self.config.enable_persistence {
			if let Err(e) = hybrid.persist() {
				eprintln!("[pipeline] Warning: failed to persist index: {}", e);
			}
		}

		self.triple_hybrid = Some(hybrid);
		Ok(())
	}

	/// Run the full retrieval pipeline
	pub fn retrieve(&mut self, query: &str) -> RetrievalResult<RetrievalOutput> {
		// Ensure initialized
		if self.symbols.is_none() {
			self.initialize()?;
		}

		// Step 1: Query Expansion (tiered or direct LLM with graceful degradation)
		let search_spec = if self.config.expand_query {
			if self.config.tiered_expansion {
				// Use tiered expansion with fast-path optimization
				let tiered_config = TieredConfig {
					confidence_threshold: self.config.fast_path_threshold,
					importance_threshold: self.config.importance_threshold,
					existence_threshold: 0.5,
				};
				let expander = TieredQueryExpander::with_config(
					&self.daemon,
					self.graph.as_ref(),
					tiered_config,
				);
				match expander.expand(query) {
					Ok(result) => {
						eprintln!(
							"[pipeline] Tiered expansion: {:?} in {}ms",
							result.tier_used, result.time_ms
						);
						result.spec
					}
					Err(e) => {
						eprintln!("[pipeline] Tiered expansion failed: {}, using fallback", e);
						self.fallback_query_expansion(query)
					}
				}
			} else {
				// Direct LLM expansion with safe timeout
				eprintln!("[pipeline] Expanding query via LLM...");
				self.expand_query_safe(query)
			}
		} else {
			// Simple spec without expansion
			SearchSpec {
				original_query: query.to_string(),
				symbol_names: query.split_whitespace().map(String::from).collect(),
				intent: crate::retrieval::daemon::protocol::QueryIntent::Search,
				file_filters: Vec::new(),
				context_hints: Vec::new(),
			}
		};

		eprintln!(
			"[pipeline] Search spec: {:?} symbols, intent: {:?}",
			search_spec.symbol_names.len(),
			search_spec.intent
		);

		// Step 2: Hybrid Search
		let fetch_limit = if self.config.rerank {
			self.config.max_results * 3
		} else {
			self.config.max_results
		};

		let mut results = self.search(&search_spec, fetch_limit)?;
		eprintln!("[pipeline] Found {} candidates", results.len());

		// For Understand intent, filter out documentation to favor source code
		if matches!(search_spec.intent, crate::retrieval::daemon::protocol::QueryIntent::Understand) {
			let before_count = results.len();
			results.retain(|r| {
				// Keep everything except DocumentChunk from doc files
				!(r.symbol.kind == crate::indexer::SymbolKind::DocumentChunk
					&& r.symbol.location.file.to_string_lossy().contains("/doc/"))
			});
			if results.len() < before_count {
				eprintln!("[pipeline] Filtered {} doc chunks for Understand intent", before_count - results.len());
			}
		}

		// Step 3: Reranking (with graceful degradation)
		if self.config.rerank && !results.is_empty() {
			eprintln!("[pipeline] Reranking...");
			results = self.rerank_safe(query, &search_spec, results);
			results.truncate(self.config.max_results);
		}

		// Step 4: Context Expansion
		let xml_output = if self.config.expand_context {
			eprintln!("[pipeline] Expanding context...");
			self.expand_context(&results)?
		} else {
			self.format_simple(&results)
		};

		let token_count = xml_output.len() / 4; // rough estimate

		Ok(RetrievalOutput {
			query: query.to_string(),
			search_spec,
			xml_output,
			result_count: results.len(),
			token_count,
			has_more: results.len() >= self.config.max_results,
		})
	}

	/// Run structured pipeline retrieval (code, doc, notes separate)
	/// Returns StructuredOutput with full file content
	pub fn retrieve_structured(&mut self, query: &str) -> RetrievalResult<StructuredOutput> {
		if self.symbols.is_none() {
			self.initialize()?;
		}

		let search_spec = self.expand_query_for_structured(query);
		let intent_str = format!("{:?}", search_spec.intent);

		let hybrid = self.triple_hybrid.as_ref().ok_or_else(|| {
			RetrievalError::Embedding("Structured pipeline not initialized".to_string())
		})?;

		let results = hybrid.search(
			&search_spec.original_query,
			self.config.max_code_results,
			self.config.max_doc_results,
			self.config.max_notes_results,
		)?;

		let mut output = StructuredOutput::new(query.to_string(), intent_str);

		// Process code results with full file content
		for result in &results.code_results {
			if let Some(code_result) = self.build_code_result(&result.symbol) {
				output.add_code(code_result);
			}
		}

		// Process doc results
		for result in &results.doc_results {
			output.add_doc(self.build_doc_result(&result.symbol));
		}

		// Process notes results
		for result in &results.notes_results {
			output.add_notes(self.build_notes_result(&result.symbol));
		}

		eprintln!(
			"[pipeline] Results: {} code, {} doc, {} notes",
			output.code_context.len(),
			output.doc_context.len(),
			output.notes_context.len()
		);

		Ok(output)
	}

	/// Expand query for structured pipeline (simplified, no reranking)
	fn expand_query_for_structured(&self, query: &str) -> SearchSpec {
		if self.config.expand_query {
			self.expand_query_safe(query)
		} else {
			fallback_parse(query)
		}
	}

	/// Build CodeResult with full file content
	fn build_code_result(&self, symbol: &Symbol) -> Option<CodeResult> {
		let file_path = &symbol.location.file;
		let content = std::fs::read_to_string(file_path).ok()?;
		let lines: Vec<&str> = content.lines().collect();
		let line_count = lines.len();
		let truncated = line_count > self.config.max_lines_per_file;
		let full_content = if truncated {
			lines[..self.config.max_lines_per_file].join("\n")
		} else {
			content
		};

		Some(CodeResult {
			file: file_path.display().to_string(),
			symbol: symbol.name.clone(),
			kind: symbol.kind.to_string(),
			line: symbol.location.line,
			signature: symbol.signature.clone(),
			full_content,
			line_count,
			truncated,
			relevance_score: 1.0,
		})
	}

	/// Build DocResult from symbol
	fn build_doc_result(&self, symbol: &Symbol) -> DocResult {
		let content = symbol.content.clone().unwrap_or_default();
		DocResult {
			file: symbol.location.file.display().to_string(),
			section: symbol.name.clone(),
			content,
			relevance_score: 1.0,
		}
	}

	/// Build NotesResult from symbol
	fn build_notes_result(&self, symbol: &Symbol) -> NotesResult {
		let content = symbol.content.clone().unwrap_or_default();
		NotesResult {
			file: symbol.location.file.display().to_string(),
			section: symbol.name.clone(),
			content,
			relevance_score: 1.0,
		}
	}

	/// Perform hybrid search using SearchSpec with intent-aware boosting
	fn search(
		&self,
		spec: &SearchSpec,
		limit: usize,
	) -> RetrievalResult<Vec<HybridSearchResult>> {
		if let Some(ref hybrid) = self.hybrid {
			// use search_with_spec for intent-aware boost
			hybrid.search_with_spec(spec, limit)
		} else {
			// Fallback: no results if hybrid not initialized
			Ok(Vec::new())
		}
	}

	/// Safe query expansion with timeout
	/// Returns fallback SearchSpec if daemon fails or times out
	fn expand_query_safe(&self, query: &str) -> SearchSpec {
		let (tx, rx) = mpsc::channel(); // channel for result communication
		let query_clone = query.to_string(); // owned copy for thread

		std::thread::spawn(move || {
			// Create new daemon client in thread (avoids Clone requirement)
			let daemon = DaemonClient::new();
			let result = daemon.expand(query_clone);
			let _ = tx.send(result);
		});

		// Wait with 10 second timeout
		match rx.recv_timeout(Duration::from_secs(10)) {
			Ok(Ok(spec)) => spec,
			Ok(Err(e)) => {
				eprintln!("[pipeline] Query expansion failed: {}, using fallback", e);
				self.fallback_query_expansion(query)
			}
			Err(_) => {
				eprintln!("[pipeline] Query expansion timed out, using fallback");
				self.fallback_query_expansion(query)
			}
		}
	}

	/// Safe reranking with timeout
	/// Returns original results (sorted by RRF) if daemon fails or times out
	fn rerank_safe(
		&self,
		query: &str,
		spec: &SearchSpec,
		results: Vec<HybridSearchResult>,
	) -> Vec<HybridSearchResult> {
		if results.is_empty() {
			return results;
		}

		let (tx, rx) = mpsc::channel(); // channel for result communication
		let query_clone = query.to_string(); // owned copy for thread

		// Create documents for reranking (before spawning thread)
		let documents: Vec<String> = results
			.iter()
			.map(|r| {
				format!(
					"{} {} {}",
					r.symbol.kind,
					r.symbol.name,
					r.symbol.signature.as_deref().unwrap_or("")
				)
			})
			.collect();

		std::thread::spawn(move || {
			// Create new daemon client in thread (avoids Clone requirement)
			let daemon = DaemonClient::new();
			let rerank_result = daemon.rerank(query_clone, documents);
			let _ = tx.send(rerank_result);
		});

		// Wait with 15 second timeout (reranking can be slower)
		match rx.recv_timeout(Duration::from_secs(15)) {
			Ok(Ok(scores)) => {
				// Apply scores and sort
				let mut reranked = results;
				for (result, score) in reranked.iter_mut().zip(scores.iter()) {
					// Apply query-aware and intent-aware boosts
					let doc_type = DocumentType::from_path(&result.symbol.location.file);
					let doc_query_boost = doc_type.boost_factor_for_query(query);
					let doc_intent_boost = doc_type.boost_factor_for_intent(&spec.intent);
					let kind_boost = result.symbol.kind.boost_factor_for_intent(&spec.intent);
					let combined_boost = doc_query_boost * doc_intent_boost * kind_boost;
					result.rerank_score = Some(*score * combined_boost);
				}
				reranked.sort_by(|a, b| {
					b.rerank_score
						.partial_cmp(&a.rerank_score)
						.unwrap_or(std::cmp::Ordering::Equal)
				});
				reranked
			}
			Ok(Err(e)) => {
				eprintln!("[pipeline] Reranking failed: {}, using RRF scores", e);
				results // Already sorted by RRF
			}
			Err(_) => {
				eprintln!("[pipeline] Reranking timed out, using RRF scores");
				results // Already sorted by RRF
			}
		}
	}

	/// Fallback query expansion when daemon is unavailable
	/// Uses fast-path parser for basic symbol extraction
	fn fallback_query_expansion(&self, query: &str) -> SearchSpec {
		fallback_parse(query)
	}

	/// Expand context for search results
	fn expand_context(&self, results: &[HybridSearchResult]) -> RetrievalResult<String> {
		let symbols: Vec<_> = results.iter().map(|r| r.symbol.clone()).collect();

		if let Some(ref graph) = self.graph {
			let config = ContextConfig::default();
			let expander = ContextExpander::with_config(graph, config, self.config.max_tokens);
			Ok(expander.expand_to_xml(&symbols))
		} else {
			Ok(self.format_simple(results))
		}
	}

	/// Simple formatting without context expansion
	fn format_simple(&self, results: &[HybridSearchResult]) -> String {
		let mut output = String::from("<results>\n");

		for result in results {
			output.push_str(&format!(
				"  <symbol kind=\"{}\" name=\"{}\" file=\"{}\" line=\"{}\"/>\n",
				result.symbol.kind,
				result.symbol.name,
				result.symbol.location.file.display(),
				result.symbol.location.line
			));
		}

		output.push_str("</results>");
		output
	}

	/// Get more context for specific symbols
	pub fn get_more_context(&self, symbol_names: &[String]) -> RetrievalResult<String> {
		let symbols = self.symbols.as_ref().ok_or_else(|| {
			RetrievalError::Embedding("Pipeline not initialized".to_string())
		})?;

		// Find matching symbols
		let matching: Vec<_> = symbols
			.iter()
			.filter(|s| symbol_names.contains(&s.name))
			.cloned()
			.collect();

		if let Some(ref graph) = self.graph {
			let config = ContextConfig {
				max_callers: 10,
				max_callees: 10,
				max_usages_per_symbol: 30,
				context_lines_before: 5,
				context_lines_after: 20,
				include_parent: true,
				include_related_types: true,
			};
			let expander = ContextExpander::with_config(graph, config, self.config.max_tokens * 2);
			Ok(expander.expand_to_xml(&matching))
		} else {
			Err(RetrievalError::Embedding("No semantic graph".to_string()))
		}
	}
}

impl Default for RetrievalPipeline {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::retrieval::daemon::protocol::QueryIntent;

	#[test]
	fn test_config_default() {
		let config = PipelineConfig::default();
		assert_eq!(config.max_results, 20);
		assert!(config.expand_query);
		assert!(config.semantic_search);
		assert!(config.use_daemon_cache);
		assert!(config.include_usage_counts);
	}

	#[test]
	fn test_config_persistence_enabled_by_default() {
		let config = PipelineConfig::default();
		assert!(config.enable_persistence, "Persistence should be enabled by default");
	}

	#[test]
	fn test_config_with_persistence_disabled() {
		let config = PipelineConfig {
			enable_persistence: false,
			..PipelineConfig::default()
		};
		assert!(!config.enable_persistence);
	}

	#[test]
	fn test_fallback_query_expansion_extracts_symbols() {
		// Test that fallback parses CamelCase and snake_case symbols correctly
		let pipeline = RetrievalPipeline::new();

		// Test CamelCase extraction
		let spec = pipeline.fallback_query_expansion("find AuthService");
		assert!(
			spec.symbol_names.contains(&"AuthService".to_string()),
			"Should extract CamelCase symbol"
		);

		// Test snake_case extraction
		let spec2 = pipeline.fallback_query_expansion("where is parse_config");
		assert!(
			spec2.symbol_names.contains(&"parse_config".to_string()),
			"Should extract snake_case symbol"
		);

		// Test multiple symbols
		let spec3 = pipeline.fallback_query_expansion("RetrievalPipeline and HybridSearch");
		assert!(
			spec3.symbol_names.contains(&"RetrievalPipeline".to_string()),
			"Should extract first CamelCase symbol"
		);
		assert!(
			spec3.symbol_names.contains(&"HybridSearch".to_string()),
			"Should extract second CamelCase symbol"
		);
	}

	#[test]
	fn test_fallback_query_expansion_detects_intent() {
		// Test intent detection for definition queries
		let pipeline = RetrievalPipeline::new();

		// Test definition intent
		let spec_def = pipeline.fallback_query_expansion("where is AuthService defined");
		assert!(
			matches!(spec_def.intent, QueryIntent::FindDefinition),
			"Should detect FindDefinition intent"
		);

		// Test usage intent
		let spec_usage = pipeline.fallback_query_expansion("how is parse_config used");
		assert!(
			matches!(spec_usage.intent, QueryIntent::FindUsages),
			"Should detect FindUsages intent"
		);

		// Test understand intent
		let spec_understand = pipeline.fallback_query_expansion("how does the pipeline work");
		assert!(
			matches!(spec_understand.intent, QueryIntent::Understand),
			"Should detect Understand intent"
		);

		// Test debug intent
		let spec_debug = pipeline.fallback_query_expansion("debug error in AuthService");
		assert!(
			matches!(spec_debug.intent, QueryIntent::Debug),
			"Should detect Debug intent"
		);

		// Test default search intent
		let spec_search = pipeline.fallback_query_expansion("AuthService");
		assert!(
			matches!(spec_search.intent, QueryIntent::Search),
			"Should default to Search intent"
		);
	}
}
