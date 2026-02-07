//! BgeReranker Builder Functions
//!
//! Factory methods for loading BGE reranker models.

use std::path::Path;

use candle_core::Device;
use candle_nn::VarBuilder;
use candle_transformers::models::xlm_roberta::{
	Config as XLMRobertaConfig,
	XLMRobertaForSequenceClassification,
};

use crate::retrieval::models::{
	download_model, get_device, load_tokenizer,
};
use crate::retrieval::models::{ModelError, ModelResult};

use super::cross_encoder::{
	configure_tokenizer, BgeReranker, DEFAULT_RERANKER,
};

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
		let config_str = std::fs::read_to_string(config_path)?;
		let config: XLMRobertaConfig =
			serde_json::from_str(&config_str).map_err(|e| {
				let msg = format!("config: {}", e);
				ModelError::WeightLoad(msg)
			})?;

		let vb = load_weights_for_reranker(
			weights_paths, &device,
		)?;

		let model =
			XLMRobertaForSequenceClassification::new(
				1, &config, vb,
			)?;

		let mut tokenizer =
			load_tokenizer(&tokenizer_path.to_path_buf())?;
		configure_tokenizer(&mut tokenizer);

		Ok(Self {
			model,
			tokenizer,
			device,
		})
	}
}

/// Load weights for reranker model
fn load_weights_for_reranker(
	weights_paths: &[std::path::PathBuf],
	device: &Device,
) -> ModelResult<VarBuilder<'static>> {
	use candle_core::DType;

	let vb = unsafe {
		VarBuilder::from_mmaped_safetensors(
			weights_paths,
			DType::F32,
			device,
		)?
	};
	Ok(vb)
}
