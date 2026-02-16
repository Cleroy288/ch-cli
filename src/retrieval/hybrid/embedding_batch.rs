//! BGE Embedding Model — Batch Operations
//!
//! Text embedding generation for single and batch inputs.

use candle_core::{Device, Tensor};

use crate::retrieval::hybrid::embedding::BgeEmbedder;
use crate::retrieval::hybrid::embedding_pooling::{
	l2_normalize, mean_pooling,
};
use crate::retrieval::models::{ModelError, ModelResult};

impl BgeEmbedder {
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
		let tensors =
			build_tensors(&tokens, texts.len(), &self.device)?;
		pool_and_normalize(&self.model, &tensors)
	}
}

/// Encoded token data as device tensors
struct BatchTensors {
	/// token ID tensor [batch, seq_len]
	token_ids: Tensor,
	/// attention mask tensor [batch, seq_len]
	attention_mask: Tensor,
	/// token type ID tensor [batch, seq_len]
	token_type_ids: Tensor,
}

/// Build device tensors from encoded token batch
fn build_tensors(
	tokens: &[tokenizers::Encoding],
	batch_size: usize,
	device: &Device,
) -> ModelResult<BatchTensors> {
	let seq_len = tokens[0].get_ids().len();
	let dims = (batch_size, seq_len);

	let ids = flatten_field(tokens, |enc| enc.get_ids());
	let mask =
		flatten_field(tokens, |enc| enc.get_attention_mask());
	let types =
		flatten_field(tokens, |enc| enc.get_type_ids());

	Ok(BatchTensors {
		token_ids: Tensor::from_vec(ids, dims, device)?,
		attention_mask: Tensor::from_vec(
			mask, dims, device,
		)?,
		token_type_ids: Tensor::from_vec(
			types, dims, device,
		)?,
	})
}

/// Extracts a u32 slice from a tokenizer Encoding
type TokenFieldExtractor =
	fn(&tokenizers::Encoding) -> &[u32];

/// Extract and flatten a u32 field from encodings
fn flatten_field(
	tokens: &[tokenizers::Encoding],
	extractor: TokenFieldExtractor,
) -> Vec<u32> {
	tokens
		.iter()
		.flat_map(|enc| extractor(enc).to_vec())
		.collect()
}

/// Forward pass + mean pooling + L2 normalize
fn pool_and_normalize(
	model: &candle_transformers::models::bert::BertModel,
	tensors: &BatchTensors,
) -> ModelResult<Vec<Vec<f32>>> {
	let embeddings = model.forward(
		&tensors.token_ids,
		&tensors.token_type_ids,
		Some(&tensors.attention_mask),
	)?;
	let pooled =
		mean_pooling(&embeddings, &tensors.attention_mask)?;
	let normalized = l2_normalize(&pooled)?;
	let result = normalized
		.to_dtype(candle_core::DType::F32)?
		.to_vec2::<f32>()?;
	Ok(result)
}
