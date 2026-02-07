//! BGE Embedding Model — Batch Operations
//!
//! Text embedding generation for single and batch inputs.

use candle_core::Tensor;

use crate::retrieval::hybrid::embedding::BgeEmbedder;
use crate::retrieval::hybrid::embedding_pooling::{
	l2_normalize, mean_pooling,
};
use crate::retrieval::models::{ModelError, ModelResult};

impl BgeEmbedder {
	/// Generate embedding for a single text
	pub fn embed_text(&self, text: &str) -> ModelResult<Vec<f32>> {
		let embeddings = self.embed_batch(&[text.to_string()])?;
		Ok(embeddings.into_iter().next().unwrap())
	}

	/// Generate embeddings for multiple texts
	pub fn embed_batch(
		&self,
		texts: &[String],
	) -> ModelResult<Vec<Vec<f32>>> {
		if texts.is_empty() {
			return Ok(Vec::new());
		}

		// tokenize
		let tokens = self
			.tokenizer
			.encode_batch(texts.to_vec(), true)
			.map_err(|e| ModelError::Tokenizer(e.to_string()))?;

		// prepare tensors
		let token_ids: Vec<Vec<u32>> = tokens
			.iter()
			.map(|t| t.get_ids().to_vec())
			.collect();
		let attention_mask: Vec<Vec<u32>> = tokens
			.iter()
			.map(|t| t.get_attention_mask().to_vec())
			.collect();
		let token_type_ids: Vec<Vec<u32>> = tokens
			.iter()
			.map(|t| t.get_type_ids().to_vec())
			.collect();

		let batch_size = texts.len();
		let seq_len = token_ids[0].len();

		// flatten and create tensors
		let token_ids_flat: Vec<u32> =
			token_ids.into_iter().flatten().collect();
		let attention_mask_flat: Vec<u32> =
			attention_mask.into_iter().flatten().collect();
		let token_type_ids_flat: Vec<u32> =
			token_type_ids.into_iter().flatten().collect();

		let dims = (batch_size, seq_len);
		let token_ids_tensor =
			Tensor::from_vec(token_ids_flat, dims, &self.device)?;
		let attention_mask_tensor = Tensor::from_vec(
			attention_mask_flat,
			dims,
			&self.device,
		)?;
		let token_type_ids_tensor = Tensor::from_vec(
			token_type_ids_flat,
			dims,
			&self.device,
		)?;

		// forward pass
		let embeddings = self.model.forward(
			&token_ids_tensor,
			&token_type_ids_tensor,
			Some(&attention_mask_tensor),
		)?;

		// mean pooling over sequence dimension
		let pooled = mean_pooling(&embeddings, &attention_mask_tensor)?;

		// normalize embeddings
		let normalized = l2_normalize(&pooled)?;

		// convert to Vec<Vec<f32>>
		let result = normalized.to_vec2::<f32>()?;
		Ok(result)
	}
}
