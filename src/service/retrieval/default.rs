//! Default implementation of RetrievalService.

use crate::domain::errors::retrieval::RetrievalError;
use crate::retrieval::{
	PipelineConfig, RetrievalOutput,
	RetrievalPipeline, StructuredOutput,
};

use super::types::RetrievalRequest;
use super::RetrievalService;

/// Default retrieval service backed by pipeline
pub struct DefaultRetrievalService;

impl DefaultRetrievalService {
	/// Create a new default retrieval service
	pub fn new() -> Self {
		Self
	}
}

impl RetrievalService for DefaultRetrievalService {
	fn retrieve(
		&self,
		req: &RetrievalRequest,
	) -> Result<RetrievalOutput, RetrievalError> {
		let config = build_config(req);
		let mut pipeline =
			RetrievalPipeline::with_config(config);
		pipeline.initialize()?;
		pipeline.retrieve(&req.query)
	}

	fn retrieve_structured(
		&self,
		req: &RetrievalRequest,
	) -> Result<StructuredOutput, RetrievalError> {
		let config = build_config(req);
		let mut pipeline =
			RetrievalPipeline::with_config(config);
		pipeline.initialize()?;
		pipeline.retrieve_structured(&req.query)
	}
}

/// Build PipelineConfig from a RetrievalRequest
fn build_config(
	req: &RetrievalRequest,
) -> PipelineConfig {
	PipelineConfig {
		max_results: req.limit,
		max_tokens: req.max_tokens,
		expand_query: !req.no_expand,
		rerank: !req.no_rerank,
		expand_context: !req.no_context,
		project_path: req.project_path.clone(),
		rrf_score_threshold: req.threshold,
		min_results_per_type: req.min_results,
		..PipelineConfig::default()
	}
}
