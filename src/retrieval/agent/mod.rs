//! Agentic Execution Module
//!
//! Orchestrates the full retrieval pipeline:
//! 1. Query Expansion (LLM interprets natural language)
//! 2. Hybrid Search (keyword + semantic with RRF fusion)
//! 3. Context Expansion (graph traversal for context)
//! 4. Semantic Reranking (cross-encoder for relevance)
//! 5. Output Formatting (XML for LLM consumption)

pub mod feedback;
mod feedback_accessors;
pub mod feedback_actions;
mod feedback_handlers;
mod feedback_internal;
mod feedback_parsers;
pub mod feedback_scoring;
pub mod output;
mod output_json;
pub mod output_structured;
pub mod output_xml;
pub mod pipeline;

pub use feedback::FeedbackLoop;
pub use feedback_actions::{FeedbackAction, parse_feedback_action};
pub use output::StructuredOutput;
pub use output_structured::{CodeResult, DocResult, NotesResult};
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
			"Query: '{}'\nIntent: {:?}\n\
			Symbols: {:?}\nResults: {} ({} tokens)",
			self.query,
			self.search_spec.intent,
			self.search_spec.symbol_names,
			self.result_count,
			self.token_count,
		)
	}
}

