//! DTOs for the retrieval service.

/// Request parameters for retrieval
#[derive(Debug, Clone)]
pub struct RetrievalRequest {
	/// The natural language query
	pub query: String,
	/// Maximum results
	pub limit: usize,
	/// Maximum tokens in output
	pub max_tokens: usize,
	/// Disable query expansion
	pub no_expand: bool,
	/// Disable reranking
	pub no_rerank: bool,
	/// Disable context expansion
	pub no_context: bool,
	/// Output XML format
	pub xml: bool,
	/// Use structured output
	pub structured: bool,
	/// RRF score threshold
	pub threshold: f32,
	/// Minimum results per type
	pub min_results: usize,
	/// Project path
	pub project_path: String,
}

impl Default for RetrievalRequest {
	fn default() -> Self {
		Self {
			query: String::new(),
			limit: 10,
			max_tokens: 8000,
			no_expand: false,
			no_rerank: false,
			no_context: false,
			xml: false,
			structured: false,
			threshold: 0.015,
			min_results: 1,
			project_path: ".".to_string(),
		}
	}
}
