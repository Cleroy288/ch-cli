//! Pipeline configuration for retrieval.

#[allow(clippy::struct_excessive_bools)]
/// Boolean feature toggles for the pipeline
#[derive(Debug, Clone)]
pub struct PipelineFlags {
	/// enable query expansion via LLM
	pub expand_query: bool,
	/// enable tiered expansion (fast-path)
	pub tiered_expansion: bool,
	/// enable semantic search
	pub semantic_search: bool,
	/// enable reranking
	pub rerank: bool,
	/// enable context expansion
	pub expand_context: bool,
	/// persistent index caching
	pub enable_persistence: bool,
	/// daemon-side project caching
	pub use_daemon_cache: bool,
	/// include usage count per symbol
	pub include_usage_counts: bool,
	/// use generated doc for context
	pub use_doc_context: bool,
	/// structured pipeline (code/doc/notes)
	pub use_triple_pipeline: bool,
}

impl Default for PipelineFlags {
	fn default() -> Self {
		Self {
			expand_query: true,
			tiered_expansion: true,
			semantic_search: true,
			rerank: true,
			expand_context: true,
			enable_persistence: true,
			use_daemon_cache: true,
			include_usage_counts: true,
			use_doc_context: true,
			use_triple_pipeline: true,
		}
	}
}

/// Configuration for the retrieval pipeline
#[derive(Debug, Clone)]
pub struct PipelineConfig {
	/// max results to return
	pub max_results: usize,
	/// max tokens in output
	pub max_tokens: usize,
	/// confidence threshold for fast-path
	pub fast_path_threshold: f32,
	/// importance threshold for validated symbols
	pub importance_threshold: f32,
	/// path to index (defaults to ".")
	pub project_path: String,
	/// max code results for structured pipeline
	pub max_code_results: usize,
	/// max doc results for structured pipeline
	pub max_doc_results: usize,
	/// max notes results for structured pipeline
	pub max_notes_results: usize,
	/// max lines per file (full content)
	pub max_lines_per_file: usize,
	/// Fusion score threshold for filtering
	pub score_threshold: f32,
	/// min results per content type
	pub min_results_per_type: usize,
	/// boolean feature toggles
	pub flags: PipelineFlags,
}

impl Default for PipelineConfig {
	fn default() -> Self {
		Self {
			max_results: 20,
			max_tokens: 8000,
			fast_path_threshold: 0.7,
			importance_threshold: 0.5,
			project_path: ".".to_string(),
			max_code_results: 10,
			max_doc_results: 5,
			max_notes_results: 3,
			max_lines_per_file: 500,
			score_threshold: 0.05,
			min_results_per_type: 1,
			flags: PipelineFlags::default(),
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
