//! BGE Cross-Encoder Reranker (XLM-RoBERTa based)
//!
//! Scores (query, document) pairs using the BGE reranker model.
//! Unlike bi-encoders, cross-encoders process both inputs together
//! for more accurate relevance scoring.
//!
//! Note: BGE-reranker-base uses XLM-RoBERTa architecture, not BERT.

use candle_core::{Device, Tensor};
use candle_transformers::models::xlm_roberta::{
	XLMRobertaForSequenceClassification,
};
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

use crate::retrieval::models::{ModelError, ModelResult};

/// Default BGE reranker model
pub const DEFAULT_RERANKER: &str = "BAAI/bge-reranker-base";

/// Maximum sequence length for reranker
pub const MAX_SEQ_LEN: usize = 512;

/// BGE Reranker for scoring query-document pairs
pub struct BgeReranker {
	/// the XLM-RoBERTa model with classification head
	pub(crate) model: XLMRobertaForSequenceClassification,
	/// tokenizer for input processing
	pub(crate) tokenizer: Tokenizer,
	/// device (CPU/GPU)
	pub(crate) device: Device,
}

impl BgeReranker {
	/// Score a single (query, document) pair
	pub fn score(&self, query: &str, document: &str) -> ModelResult<f32> {
		let scores = self.score_batch(query, &[document.to_string()])?;
		Ok(scores.into_iter().next().unwrap_or(0.0))
	}

	/// Score multiple documents against a single query
	pub fn score_batch(
		&self,
		query: &str,
		documents: &[String],
	) -> ModelResult<Vec<f32>> {
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
	pub fn score_pairs(
		&self,
		pairs: &[(String, String)],
	) -> ModelResult<Vec<f32>> {
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

		let batch_size = pairs.len();
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
		let attention_mask_tensor =
			Tensor::from_vec(attention_mask_flat, dims, &self.device)?;
		let token_type_ids_tensor =
			Tensor::from_vec(token_type_ids_flat, dims, &self.device)?;

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
pub(super) fn configure_tokenizer(tokenizer: &mut Tokenizer) {
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

