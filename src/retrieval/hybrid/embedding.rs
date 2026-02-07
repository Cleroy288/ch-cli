//! BGE Embedding Model
//!
//! Provides sentence embeddings using the BGE model. Uses
//! candle-transformers BERT with mean pooling.

use std::path::Path;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{
	BertModel, Config as BertConfig,
};
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

use crate::retrieval::models::{
	download_model, get_device, load_tokenizer, ModelError,
	ModelResult,
};

/// Default BGE model for embeddings
pub const DEFAULT_MODEL: &str = "BAAI/bge-small-en-v1.5";

/// Embedding dimension for bge-small
pub const EMBEDDING_DIM: usize = 384;

/// Maximum sequence length
pub const MAX_SEQ_LEN: usize = 512;

/// BGE Embedder for generating sentence embeddings
pub struct BgeEmbedder {
	/// the BERT model
	pub(crate) model: BertModel,
	/// tokenizer for input processing
	pub(crate) tokenizer: Tokenizer,
	/// device (CPU/GPU)
	pub(crate) device: Device,
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

	/// Load from local paths (supports sharded/multi-file weights)
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
			VarBuilder::from_mmaped_safetensors(
				weights_paths, DType::F32, &device,
			)?
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
}

/// Configure tokenizer with padding and truncation
pub(crate) fn configure_tokenizer(tokenizer: &mut Tokenizer) {
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

