//! Jina Embeddings v2 Model
//!
//! Code-specialized embeddings using custom JinaCodeBert
//! with ALiBi attention. 768-dim, 8192 token context,
//! optimized for code retrieval across 31 languages.

use std::path::Path;

use candle_core::Device;
use candle_nn::VarBuilder;
use tokenizers::{
	PaddingParams, Tokenizer, TruncationParams,
};

use super::jina_code_model::{
	JinaCodeBert, JinaCodeConfig,
};
use crate::retrieval::models::{
	download_model, embedding_dtype, get_device,
	get_device_info, load_tokenizer, ModelError,
	ModelResult,
};

/// Default Jina code embeddings model
pub const JINA_MODEL: &str =
	"jinaai/jina-embeddings-v2-base-code";

/// Embedding dimension for jina-embeddings-v2-base-code
pub const JINA_EMBEDDING_DIM: usize = 768;

/// Maximum sequence length (ALiBi supports up to 8192)
pub const JINA_MAX_SEQ_LEN: usize = 8192;

/// Jina Embedder for code-specialized sentence embeddings
pub struct JinaEmbedder {
	/// the JinaCodeBert model (ALiBi + QK post-norm)
	pub(crate) model: JinaCodeBert,
	/// tokenizer for input processing
	pub(crate) tokenizer: Tokenizer,
	/// device (CPU/GPU)
	pub(crate) device: Device,
	/// embedding dimension
	dim: usize,
}

impl JinaEmbedder {
	/// Load the default Jina code model
	pub fn new() -> ModelResult<Self> {
		Self::from_model_id(JINA_MODEL)
	}

	/// Load a specific Jina model by ID
	pub fn from_model_id(
		model_id: &str,
	) -> ModelResult<Self> {
		let device = get_device();
		let info = download_model(model_id)?;
		Self::from_paths(
			&info.weights_paths,
			&info.tokenizer_path,
			&info.config_path,
			device,
		)
	}

	/// Load from local file paths
	pub fn from_paths(
		weights_paths: &[std::path::PathBuf],
		tokenizer_path: &Path,
		config_path: &Path,
		device: Device,
	) -> ModelResult<Self> {
		let config = parse_jina_config(config_path)?;
		let dim = config.hidden;
		let var_builder =
			load_weights(weights_paths, &device)?;
		let model = JinaCodeBert::new(
			var_builder, &config,
		)
		.map_err(|err| {
			ModelError::WeightLoad(err.to_string())
		})?;
		let tokenizer =
			build_tokenizer(tokenizer_path)?;
		Ok(Self { model, tokenizer, device, dim })
	}

	/// Get embedding dimension
	pub fn dim(&self) -> usize {
		self.dim
	}
}

/// Load safetensor weights with optimal dtype
fn load_weights(
	paths: &[std::path::PathBuf],
	device: &Device,
) -> ModelResult<VarBuilder<'static>> {
	let dtype =
		embedding_dtype(get_device_info().device_type);
	let var_builder = unsafe {
		VarBuilder::from_mmaped_safetensors(
			paths, dtype, device,
		)?
	};
	Ok(var_builder)
}

/// Load and configure tokenizer with padding/truncation
fn build_tokenizer(
	path: &Path,
) -> ModelResult<Tokenizer> {
	let mut tokenizer =
		load_tokenizer(&path.to_path_buf())?;
	let padding = PaddingParams {
		strategy:
			tokenizers::PaddingStrategy::BatchLongest,
		pad_id: 0,
		pad_token: "[PAD]".to_string(),
		..Default::default()
	};
	tokenizer.with_padding(Some(padding));
	let trunc = TruncationParams {
		max_length: JINA_MAX_SEQ_LEN,
		..Default::default()
	};
	let _ = tokenizer.with_truncation(Some(trunc));
	Ok(tokenizer)
}

/// Parse model config from JSON file
fn parse_jina_config(
	path: &Path,
) -> ModelResult<JinaCodeConfig> {
	let text = std::fs::read_to_string(path)?;
	let json: serde_json::Value =
		serde_json::from_str(&text).map_err(|err| {
			ModelError::WeightLoad(
				format!("config: {}", err),
			)
		})?;
	let get_field = |key: &str, default: u64| -> usize {
		json[key].as_u64().unwrap_or(default) as usize
	};
	Ok(JinaCodeConfig {
		num_layers: get_field("num_hidden_layers", 12),
		num_heads: get_field("num_attention_heads", 12),
		hidden: get_field("hidden_size", 768),
		intermediate: get_field("intermediate_size", 3072),
		vocab_size: get_field("vocab_size", 61056),
	})
}
