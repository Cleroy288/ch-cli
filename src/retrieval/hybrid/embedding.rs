//! BGE Embedding Model
//!
//! Provides sentence embeddings using the BGE (BAAI General Embedding) model.
//! Uses candle-transformers BERT implementation with mean pooling.

use std::path::Path;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config as BertConfig};
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

use crate::retrieval::models::{download_model, get_device, load_tokenizer, ModelError, ModelResult};

/// Default BGE model for embeddings
pub const DEFAULT_MODEL: &str = "BAAI/bge-small-en-v1.5";

/// Embedding dimension for bge-small
pub const EMBEDDING_DIM: usize = 384;

/// Maximum sequence length
pub const MAX_SEQ_LEN: usize = 512;

/// BGE Embedder for generating sentence embeddings
pub struct BgeEmbedder {
	/// the BERT model
	model: BertModel,
	/// tokenizer for input processing
	tokenizer: Tokenizer,
	/// device (CPU/GPU)
	device: Device,
	/// embedding dimension
	dim: usize,
}

impl BgeEmbedder {
	/// Load the default BGE model
	pub fn new() -> ModelResult<Self> {
		Self::from_model_id(DEFAULT_MODEL)
	}

	/// Load a specific BGE model by ID
	pub fn from_model_id(model_id: &str) -> ModelResult<Self> {
		let device = get_device();
		let model_info = download_model(model_id)?;

		Self::from_paths(
			&model_info.weights_paths,
			&model_info.tokenizer_path,
			&model_info.config_path,
			device,
		)
	}

	/// Load from local paths (supports sharded models with multiple weight files)
	pub fn from_paths(
		weights_paths: &[std::path::PathBuf],
		tokenizer_path: &Path,
		config_path: &Path,
		device: Device,
	) -> ModelResult<Self> {
		// load config
		let config_str = std::fs::read_to_string(config_path)?;
		let config: BertConfig = serde_json::from_str(&config_str)
			.map_err(|e| ModelError::WeightLoad(format!("config: {}", e)))?;

		let dim = config.hidden_size;

		// load weights (supports multiple files for sharded models)
		let vb = unsafe {
			VarBuilder::from_mmaped_safetensors(weights_paths, DType::F32, &device)?
		};

		// create model
		let model = BertModel::load(vb, &config)?;

		// load tokenizer
		let mut tokenizer = load_tokenizer(&tokenizer_path.to_path_buf())?;
		configure_tokenizer(&mut tokenizer);

		Ok(Self {
			model,
			tokenizer,
			device,
			dim,
		})
	}

	/// Get embedding dimension
	pub fn dim(&self) -> usize {
		self.dim
	}

	/// Generate embedding for a single text
	pub fn embed_text(&self, text: &str) -> ModelResult<Vec<f32>> {
		let embeddings = self.embed_batch(&[text.to_string()])?;
		Ok(embeddings.into_iter().next().unwrap())
	}

	/// Generate embeddings for multiple texts
	pub fn embed_batch(&self, texts: &[String]) -> ModelResult<Vec<Vec<f32>>> {
		if texts.is_empty() {
			return Ok(Vec::new());
		}

		// tokenize
		let tokens = self
			.tokenizer
			.encode_batch(texts.to_vec(), true)
			.map_err(|e| ModelError::Tokenizer(e.to_string()))?;

		// prepare tensors
		let token_ids: Vec<Vec<u32>> = tokens.iter().map(|t| t.get_ids().to_vec()).collect();
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
		let token_ids_flat: Vec<u32> = token_ids.into_iter().flatten().collect();
		let attention_mask_flat: Vec<u32> = attention_mask.into_iter().flatten().collect();
		let token_type_ids_flat: Vec<u32> = token_type_ids.into_iter().flatten().collect();

		let token_ids_tensor =
			Tensor::from_vec(token_ids_flat, (batch_size, seq_len), &self.device)?;
		let attention_mask_tensor =
			Tensor::from_vec(attention_mask_flat, (batch_size, seq_len), &self.device)?;
		let token_type_ids_tensor =
			Tensor::from_vec(token_type_ids_flat, (batch_size, seq_len), &self.device)?;

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

/// Configure tokenizer with padding and truncation
fn configure_tokenizer(tokenizer: &mut Tokenizer) {
	let padding = PaddingParams {
		strategy: tokenizers::PaddingStrategy::BatchLongest,
		pad_id: 0,
		pad_token: "[PAD]".to_string(),
		..Default::default()
	};
	tokenizer.with_padding(Some(padding));

	let truncation = TruncationParams {
		max_length: MAX_SEQ_LEN,
		..Default::default()
	};
	let _ = tokenizer.with_truncation(Some(truncation));
}

/// Mean pooling: average token embeddings weighted by attention mask
fn mean_pooling(embeddings: &Tensor, attention_mask: &Tensor) -> ModelResult<Tensor> {
	// embeddings: [batch, seq_len, hidden]
	// attention_mask: [batch, seq_len]

	// expand attention mask to hidden dim
	let mask = attention_mask.unsqueeze(2)?.to_dtype(DType::F32)?;
	let mask_expanded = mask.broadcast_as(embeddings.shape())?;

	// apply mask and sum
	let masked = embeddings.mul(&mask_expanded)?;
	let sum_embeddings = masked.sum(1)?;

	// count non-masked tokens
	let sum_mask = mask_expanded.sum(1)?;
	let sum_mask = sum_mask.clamp(1e-9, f64::MAX)?;

	// mean
	let mean = sum_embeddings.div(&sum_mask)?;
	Ok(mean)
}

/// L2 normalize embeddings
fn l2_normalize(embeddings: &Tensor) -> ModelResult<Tensor> {
	let norm = embeddings.sqr()?.sum_keepdim(1)?.sqrt()?;
	let norm = norm.clamp(1e-12, f64::MAX)?;
	let normalized = embeddings.broadcast_div(&norm)?;
	Ok(normalized)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore] // requires model download
	fn test_embed_single() {
		let embedder = BgeEmbedder::new().unwrap();
		let embedding = embedder.embed_text("Hello, world!").unwrap();
		assert_eq!(embedding.len(), EMBEDDING_DIM);
	}

	#[test]
	#[ignore] // requires model download
	fn test_embed_batch() {
		let embedder = BgeEmbedder::new().unwrap();
		let texts = vec![
			"Hello, world!".to_string(),
			"How are you?".to_string(),
		];
		let embeddings = embedder.embed_batch(&texts).unwrap();
		assert_eq!(embeddings.len(), 2);
		assert_eq!(embeddings[0].len(), EMBEDDING_DIM);
	}
}
