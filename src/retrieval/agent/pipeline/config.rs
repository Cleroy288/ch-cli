//! Pipeline configuration for retrieval.

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
	pub search_spec: crate::retrieval::daemon::protocol::SearchSpec,
	/// raw search results
	pub search_results: Vec<crate::retrieval::hybrid::HybridSearchResult>,
	/// formatted XML output
	pub xml_output: String,
	/// token count
	pub token_count: usize,
}
