//! Embedder Trait — BGE Implementation
//!
//! Implements the Embedder trait for BgeEmbedder,
//! allowing it to be used via dynamic dispatch.

use crate::retrieval::hybrid::embedder_trait::Embedder;
use crate::retrieval::hybrid::embedding::BgeEmbedder;
use crate::retrieval::models::ModelResult;

impl Embedder for BgeEmbedder {
	fn embed_batch(
		&self,
		texts: &[String],
	) -> ModelResult<Vec<Vec<f32>>> {
		self.embed_batch(texts)
	}

	fn dim(&self) -> usize {
		self.dim()
	}
}
