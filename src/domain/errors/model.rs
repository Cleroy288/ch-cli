//! Error types for ML model loading operations.

/// Errors that can occur during model loading
#[derive(Debug, thiserror::Error)]
pub enum ModelError {
	#[error("HuggingFace Hub error: {0}")]
	Hub(String),

	#[error("Model file not found: {0}")]
	FileNotFound(String),

	#[error("Failed to load weights: {0}")]
	WeightLoad(String),

	#[error("Tokenizer error: {0}")]
	Tokenizer(String),

	#[error("IO error: {0}")]
	IoError(#[from] std::io::Error),

	#[error("Candle error: {0}")]
	Candle(#[from] candle_core::Error),

	#[error("Model not loaded: {0}")]
	NotLoaded(String),
}

/// Result type for model operations
pub type ModelResult<T> = Result<T, ModelError>;
