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

/// Bundled input tensors for the cross-encoder
struct InputTensors {
	/// token ID tensor
	ids: Tensor,
	/// attention mask tensor
	mask: Tensor,
	/// token type ID tensor
	type_ids: Tensor,
}

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

		let tokens = self.tokenize_pairs(pairs)?;
		let tensors =
			self.build_tensors(&tokens, pairs.len())?;

		self.forward_pass(&tensors)
	}

	/// Tokenize (query, document) pairs for the model
	fn tokenize_pairs(
		&self,
		pairs: &[(String, String)],
	) -> ModelResult<Vec<tokenizers::Encoding>> {
		let texts: Vec<(String, String)> = pairs.to_vec();
		self.tokenizer
			.encode_batch(texts, true)
			.map_err(|err| {
				ModelError::Tokenizer(err.to_string())
			})
	}

	/// Build input tensors from tokenized encodings
	fn build_tensors(
		&self,
		tokens: &[tokenizers::Encoding],
		batch_size: usize,
	) -> ModelResult<InputTensors> {
		let token_ids = extract_ids(tokens);
		let masks = extract_masks(tokens);
		let type_ids = extract_type_ids(tokens);

		let seq_len = token_ids[0].len();
		let dims = (batch_size, seq_len);

		Ok(InputTensors {
			ids: flatten_to_tensor(
				token_ids, dims, &self.device,
			)?,
			mask: flatten_to_tensor(
				masks, dims, &self.device,
			)?,
			type_ids: flatten_to_tensor(
				type_ids, dims, &self.device,
			)?,
		})
	}

	/// Run forward pass and extract scores
	fn forward_pass(
		&self,
		tensors: &InputTensors,
	) -> ModelResult<Vec<f32>> {
		let logits = self.model.forward(
			&tensors.ids,
			&tensors.mask,
			&tensors.type_ids,
		)?;
		let scores = candle_nn::ops::sigmoid(&logits)?;
		let squeezed = scores.squeeze(1)?;
		Ok(squeezed.to_vec1::<f32>()?)
	}
}

/// Extract token IDs from encodings
fn extract_ids(
	tokens: &[tokenizers::Encoding],
) -> Vec<Vec<u32>> {
	tokens
		.iter()
		.map(|enc| enc.get_ids().to_vec())
		.collect()
}

/// Extract attention masks from encodings
fn extract_masks(
	tokens: &[tokenizers::Encoding],
) -> Vec<Vec<u32>> {
	tokens
		.iter()
		.map(|enc| enc.get_attention_mask().to_vec())
		.collect()
}

/// Extract token type IDs from encodings
fn extract_type_ids(
	tokens: &[tokenizers::Encoding],
) -> Vec<Vec<u32>> {
	tokens
		.iter()
		.map(|enc| enc.get_type_ids().to_vec())
		.collect()
}

/// Flatten 2D vec and create a tensor
fn flatten_to_tensor(
	data: Vec<Vec<u32>>,
	dims: (usize, usize),
	device: &Device,
) -> ModelResult<Tensor> {
	let flat: Vec<u32> =
		data.into_iter().flatten().collect();
	Ok(Tensor::from_vec(flat, dims, device)?)
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

