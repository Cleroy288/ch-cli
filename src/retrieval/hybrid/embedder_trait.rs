//! Embedder Trait
//!
//! Common interface for embedding models. Allows swapping
//! between BGE, Jina, or future models without changing
//! downstream code.

use crate::retrieval::models::ModelResult;

/// Trait for sentence embedding models
pub trait Embedder: Send + Sync {
	/// Generate embeddings for multiple texts
	fn embed_batch(
		&self,
		texts: &[String],
	) -> ModelResult<Vec<Vec<f32>>>;

	/// Get embedding dimension
	fn dim(&self) -> usize;
}
