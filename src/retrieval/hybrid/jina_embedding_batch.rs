//! Jina Embedding Model — Batch Operations
//!
//! Text embedding generation for single and batch inputs.
//! JinaBERT forward() only takes input_ids (ALiBi handles
//! position encoding internally).

use candle_core::{Device, Tensor};
use candle_nn::Module;

use crate::retrieval::hybrid::embedding_pooling::{
	l2_normalize, mean_pooling,
};
use crate::retrieval::hybrid::jina_embedding::JinaEmbedder;
use crate::retrieval::models::{ModelError, ModelResult};

impl JinaEmbedder {
	/// Generate embedding for a single text
	pub fn embed_text(
		&self,
		text: &str,
	) -> ModelResult<Vec<f32>> {
		let embeddings =
			self.embed_batch(&[text.to_string()])?;
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
		let tokens = self
			.tokenizer
			.encode_batch(texts.to_vec(), true)
			.map_err(|err| {
				ModelError::Tokenizer(err.to_string())
			})?;
		let (input_tensor, mask_tensor) =
			build_jina_tensors(
				&tokens, texts.len(), &self.device,
			)?;
		jina_pool_normalize(
			&self.model, &input_tensor, &mask_tensor,
		)
	}
}

/// Build input/mask tensors from encoded batch
fn build_jina_tensors(
	tokens: &[tokenizers::Encoding],
	batch_size: usize,
	device: &Device,
) -> ModelResult<(Tensor, Tensor)> {
	let seq_len = tokens[0].get_ids().len();
	let dims = (batch_size, seq_len);
	let ids: Vec<u32> = tokens
		.iter()
		.flat_map(|enc| enc.get_ids().to_vec())
		.collect();
	let mask: Vec<u32> = tokens
		.iter()
		.flat_map(|enc| enc.get_attention_mask().to_vec())
		.collect();
	let input =
		Tensor::from_vec(ids, dims, device)?;
	let mask_tensor =
		Tensor::from_vec(mask, dims, device)?;
	Ok((input, mask_tensor))
}

/// Forward pass + mean pooling + L2 normalize for Jina
fn jina_pool_normalize(
	model: &super::jina_code_model::JinaCodeBert,
	input: &Tensor,
	mask: &Tensor,
) -> ModelResult<Vec<Vec<f32>>> {
	let embeddings = model.forward(input)?;
	let pooled = mean_pooling(&embeddings, mask)?;
	let normalized = l2_normalize(&pooled)?;
	let result = normalized
		.to_dtype(candle_core::DType::F32)?
		.to_vec2::<f32>()?;
	Ok(result)
}
