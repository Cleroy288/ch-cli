//! BGE Cross-Encoder Reranker (XLM-RoBERTa based)
//!
//! Scores (query, document) pairs using the BGE reranker model.
//! Unlike bi-encoders, cross-encoders process both inputs together
//! for more accurate relevance scoring.
//!
//! Note: BGE-reranker-base uses XLM-RoBERTa architecture, not BERT.

use std::path::Path;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::xlm_roberta::{
	Config as XLMRobertaConfig,
	XLMRobertaForSequenceClassification,
};
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

use crate::retrieval::models::{download_model, get_device, load_tokenizer};
use crate::retrieval::models::{ModelError, ModelResult};

/// Default BGE reranker model
pub const DEFAULT_RERANKER: &str = "BAAI/bge-reranker-base";

/// Maximum sequence length for reranker
pub const MAX_SEQ_LEN: usize = 512;

/// BGE Reranker for scoring query-document pairs
pub struct BgeReranker {
	/// the XLM-RoBERTa model with classification head
	model: XLMRobertaForSequenceClassification,
	/// tokenizer for input processing
	tokenizer: Tokenizer,
	/// device (CPU/GPU)
	device: Device,
}

impl BgeReranker {
	/// Load the default BGE reranker
	pub fn new() -> ModelResult<Self> {
		Self::from_model_id(DEFAULT_RERANKER)
	}

	/// Load a specific reranker model by ID
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

	/// Load from local paths
	pub fn from_paths(
		weights_paths: &[std::path::PathBuf],
		tokenizer_path: &Path,
		config_path: &Path,
		device: Device,
	) -> ModelResult<Self> {
		// load config
		let config_str = std::fs::read_to_string(config_path)?;
		let config: XLMRobertaConfig = serde_json::from_str(&config_str)
			.map_err(|e| ModelError::WeightLoad(format!("config: {}", e)))?;

		// load weights (supports multiple files for sharded models)
		let vb = unsafe {
			VarBuilder::from_mmaped_safetensors(weights_paths, DType::F32, &device)?
		};

		// create XLM-RoBERTa model with classification head (num_labels=1 for reranking)
		let model = XLMRobertaForSequenceClassification::new(1, &config, vb)?;

		// load tokenizer
		let mut tokenizer = load_tokenizer(&tokenizer_path.to_path_buf())?;
		configure_tokenizer(&mut tokenizer);

		Ok(Self {
			model,
			tokenizer,
			device,
		})
	}

	/// Score a single (query, document) pair
	pub fn score(&self, query: &str, document: &str) -> ModelResult<f32> {
		let scores = self.score_batch(query, &[document.to_string()])?;
		Ok(scores.into_iter().next().unwrap_or(0.0))
	}

	/// Score multiple documents against a single query
	pub fn score_batch(&self, query: &str, documents: &[String]) -> ModelResult<Vec<f32>> {
		if documents.is_empty() {
			return Ok(Vec::new());
		}

		// create pairs: (query, doc) for each document
		let pairs: Vec<(String, String)> = documents
			.iter()
			.map(|doc| (query.to_string(), doc.clone()))
			.collect();

		self.score_pairs(&pairs)
	}

	/// Score multiple (query, document) pairs
	pub fn score_pairs(&self, pairs: &[(String, String)]) -> ModelResult<Vec<f32>> {
		if pairs.is_empty() {
			return Ok(Vec::new());
		}

		// tokenize pairs (query [SEP] document)
		let texts: Vec<(String, String)> = pairs.to_vec();
		let tokens = self
			.tokenizer
			.encode_batch(texts, true)
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

		let batch_size = pairs.len();
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

		// forward pass through XLM-RoBERTa (returns logits directly)
		let logits = self.model.forward(
			&token_ids_tensor,
			&attention_mask_tensor,
			&token_type_ids_tensor,
		)?;

		// apply sigmoid for probability score
		let scores = candle_nn::ops::sigmoid(&logits)?;

		// convert to Vec<f32> - squeeze the last dimension
		let scores_squeezed = scores.squeeze(1)?;
		let result = scores_squeezed.to_vec1::<f32>()?;
		Ok(result)
	}
}

/// Configure tokenizer for pair encoding
fn configure_tokenizer(tokenizer: &mut Tokenizer) {
	// XLM-RoBERTa uses <pad> token (id=1)
	let padding = PaddingParams {
		strategy: tokenizers::PaddingStrategy::BatchLongest,
		pad_id: 1,
		pad_token: "<pad>".to_string(),
		..Default::default()
	};
	tokenizer.with_padding(Some(padding));

	let truncation = TruncationParams {
		max_length: MAX_SEQ_LEN,
		..Default::default()
	};
	let _ = tokenizer.with_truncation(Some(truncation));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore] // requires model download
	fn test_score_single() {
		let reranker = BgeReranker::new().unwrap();
		let score = reranker
			.score("What is machine learning?", "Machine learning is a subset of AI.")
			.unwrap();
		assert!(score >= 0.0 && score <= 1.0);
	}

	#[test]
	#[ignore] // requires model download
	fn test_score_batch() {
		let reranker = BgeReranker::new().unwrap();
		let docs = vec![
			"Machine learning is a subset of AI.".to_string(),
			"The weather is nice today.".to_string(),
		];
		let scores = reranker.score_batch("What is machine learning?", &docs).unwrap();
		assert_eq!(scores.len(), 2);
		// relevant doc should score higher
		assert!(scores[0] > scores[1]);
	}
}
