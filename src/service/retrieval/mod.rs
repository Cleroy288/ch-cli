//! Retrieval service — agentic pipeline use cases.

mod default;
pub mod types;

pub use default::DefaultRetrievalService;

use crate::domain::errors::retrieval::RetrievalError;
use crate::retrieval::{RetrievalOutput, StructuredOutput};

use types::RetrievalRequest;

/// Service trait for agentic retrieval operations
pub trait RetrievalService {
	/// Run the full retrieval pipeline
	fn retrieve(
		&self,
		req: &RetrievalRequest,
	) -> Result<RetrievalOutput, RetrievalError>;

	/// Run pipeline with structured output
	fn retrieve_structured(
		&self,
		req: &RetrievalRequest,
	) -> Result<StructuredOutput, RetrievalError>;
}
