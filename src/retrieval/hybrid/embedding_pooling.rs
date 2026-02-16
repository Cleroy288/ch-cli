//! Embedding Pooling and Normalization
//!
//! Helper functions for mean pooling and L2 normalization
//! of BERT embeddings.

use candle_core::Tensor;
use crate::retrieval::models::ModelResult;

/// Mean pooling: average token embeddings weighted by attention mask
pub fn mean_pooling(
	embeddings: &Tensor,
	attention_mask: &Tensor,
) -> ModelResult<Tensor> {
	// embeddings: [batch, seq_len, hidden]
	// attention_mask: [batch, seq_len]

	// expand attention mask to hidden dim
	let mask = attention_mask.unsqueeze(2)?.to_dtype(embeddings.dtype())?;
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
pub fn l2_normalize(embeddings: &Tensor) -> ModelResult<Tensor> {
	let norm = embeddings.sqr()?.sum_keepdim(1)?.sqrt()?;
	let norm = norm.clamp(1e-12, f64::MAX)?;
	let normalized = embeddings.broadcast_div(&norm)?;
	Ok(normalized)
}
