//! Default implementation of RetrievalService.

use crate::domain::errors::retrieval::RetrievalError;
use crate::retrieval::{
	PipelineConfig, PipelineFlags, RetrievalOutput,
	RetrievalPipeline, StructuredOutput,
};

use super::types::RetrievalRequest;
use super::RetrievalService;

/// Default retrieval service backed by pipeline
pub struct DefaultRetrievalService;

impl Default for DefaultRetrievalService {
	fn default() -> Self {
		Self
	}
}

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
	let flags = &req.flags;
	PipelineConfig {
		max_results: req.limit,
		max_tokens: req.max_tokens,
		project_path: req.project_path.clone(),
		score_threshold: req.threshold,
		min_results_per_type: req.min_results,
		flags: PipelineFlags {
			expand_query: !flags.no_expand,
			rerank: !flags.no_rerank,
			expand_context: !flags.no_context,
			..PipelineFlags::default()
		},
		..PipelineConfig::default()
	}
}
