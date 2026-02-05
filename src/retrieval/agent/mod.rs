//! Agentic Execution Module
//!
//! Orchestrates the full retrieval pipeline:
//! 1. Query Expansion (LLM interprets natural language)
//! 2. Hybrid Search (keyword + semantic with RRF fusion)
//! 3. Context Expansion (graph traversal for context)
//! 4. Semantic Reranking (cross-encoder for relevance)
//! 5. Output Formatting (XML for LLM consumption)

pub mod feedback;
pub mod output;
pub mod pipeline;

pub use feedback::{FeedbackAction, FeedbackLoop};
pub use output::{CodeResult, DocResult, NotesResult, StructuredOutput};
pub use pipeline::{PipelineConfig, PipelineResult, RetrievalPipeline};

use crate::retrieval::daemon::protocol::SearchSpec;

/// Output from the agentic retrieval pipeline
#[derive(Debug, Clone)]
pub struct RetrievalOutput {
	/// the original query
	pub query: String,
	/// expanded search specification
	pub search_spec: SearchSpec,
	/// formatted XML output for LLM consumption
	pub xml_output: String,
	/// number of results found
	pub result_count: usize,
	/// total tokens in output
	pub token_count: usize,
	/// whether more context is available
	pub has_more: bool,
}

impl RetrievalOutput {
	/// Check if results were found
	pub fn has_results(&self) -> bool {
		self.result_count > 0
	}

	/// Get a summary of the retrieval
	pub fn summary(&self) -> String {
		format!(
			"Query: '{}'\nIntent: {:?}\nSymbols: {:?}\nResults: {} ({} tokens)",
			self.query,
			self.search_spec.intent,
			self.search_spec.symbol_names,
			self.result_count,
			self.token_count
		)
	}
}
