//! Model Loading Utilities
//!
//! Provides utilities for loading ML models from HuggingFace Hub
//! with caching and memory-mapping support.
//! Supports both single-file and sharded model weights.

pub mod cache;
pub mod device;

use std::collections::HashSet;
use std::path::PathBuf;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use hf_hub::api::sync::Api;
use serde::Deserialize;
use thiserror::Error;

pub use cache::ModelCache;
pub use device::{DeviceInfo, DeviceType};

/// Errors that can occur during model loading
#[derive(Error, Debug)]
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
	Io(#[from] std::io::Error),

	#[error("Candle error: {0}")]
	Candle(#[from] candle_core::Error),

	#[error("Model not loaded: {0}")]
	NotLoaded(String),
}

/// Result type for model operations
pub type ModelResult<T> = Result<T, ModelError>;

/// Information about a downloaded model
#[derive(Debug, Clone)]
pub struct ModelInfo {
	/// model identifier (e.g., "BAAI/bge-small-en-v1.5")
	pub model_id: String,
	/// paths to weights files (one for single-file, multiple for sharded)
	pub weights_paths: Vec<PathBuf>,
	/// path to tokenizer file
	pub tokenizer_path: PathBuf,
	/// path to config file
	pub config_path: PathBuf,
}

/// Sharded model index structure (model.safetensors.index.json)
#[derive(Deserialize)]
struct ShardIndex {
	/// metadata about the sharded model (unused but part of format)
	#[serde(default)]
	#[allow(dead_code)]
	metadata: Option<ShardMetadata>,
	/// mapping from tensor name to shard filename
	weight_map: std::collections::HashMap<String, String>,
}

/// Metadata in shard index
#[derive(Deserialize)]
#[allow(dead_code)]
struct ShardMetadata {
	/// total size in bytes
	#[serde(default)]
	total_size: u64,
}

/// Download model files from HuggingFace Hub
/// Handles both single-file and sharded models
pub fn download_model(model_id: &str) -> ModelResult<ModelInfo> {
	let api = Api::new().map_err(|e| ModelError::Hub(e.to_string()))?;
	let repo = api.model(model_id.to_string());

	// download tokenizer
	let tokenizer_path = repo
		.get("tokenizer.json")
		.map_err(|e| ModelError::FileNotFound(format!("tokenizer: {}", e)))?;

	// download config
	let config_path = repo
		.get("config.json")
		.map_err(|e| ModelError::FileNotFound(format!("config: {}", e)))?;

	// download weights (handles sharded models)
	let weights_paths = download_weights(&repo)?;

	Ok(ModelInfo {
		model_id: model_id.to_string(),
		weights_paths,
		tokenizer_path,
		config_path,
	})
}

/// Download model weights, handling both single-file and sharded models
fn download_weights(repo: &hf_hub::api::sync::ApiRepo) -> ModelResult<Vec<PathBuf>> {
	// Try 1: Single safetensors file
	if let Ok(path) = repo.get("model.safetensors") {
		eprintln!("[models] Found single model.safetensors");
		return Ok(vec![path]);
	}

	// Try 2: Sharded safetensors (check for index file)
	if let Ok(index_path) = repo.get("model.safetensors.index.json") {
		eprintln!("[models] Found sharded model, downloading shards...");
		let index_content = std::fs::read_to_string(&index_path)?;
		let index: ShardIndex = serde_json::from_str(&index_content)
			.map_err(|e| ModelError::WeightLoad(format!("index parse: {}", e)))?;

		// get unique shard filenames
		let shard_names: HashSet<&String> = index.weight_map.values().collect();
		let mut shard_paths = Vec::new();

		for shard_name in &shard_names {
			eprintln!("[models] Downloading shard: {}", shard_name);
			let path = repo
				.get(shard_name)
				.map_err(|e| ModelError::FileNotFound(format!("shard {}: {}", shard_name, e)))?;
			shard_paths.push(path);
		}

		// sort for deterministic ordering
		shard_paths.sort();
		eprintln!("[models] Downloaded {} shards", shard_paths.len());
		return Ok(shard_paths);
	}

	// Try 3: Single pytorch file (fallback)
	if let Ok(path) = repo.get("pytorch_model.bin") {
		eprintln!("[models] Found pytorch_model.bin");
		return Ok(vec![path]);
	}

	Err(ModelError::FileNotFound(
		"No model weights found (tried model.safetensors, sharded index, pytorch_model.bin)".to_string()
	))
}

/// Create a VarBuilder from downloaded model weights (single file)
pub fn load_weights(weights_path: &PathBuf, device: &Device) -> ModelResult<VarBuilder<'static>> {
	load_weights_multi(&[weights_path.clone()], device)
}

/// Create a VarBuilder from multiple weight files (for sharded models)
pub fn load_weights_multi(weights_paths: &[PathBuf], device: &Device) -> ModelResult<VarBuilder<'static>> {
	let vb = unsafe {
		VarBuilder::from_mmaped_safetensors(weights_paths, DType::F32, device)
			.map_err(|e| ModelError::WeightLoad(e.to_string()))?
	};
	Ok(vb)
}

/// Load tokenizer from path
pub fn load_tokenizer(tokenizer_path: &PathBuf) -> ModelResult<tokenizers::Tokenizer> {
	tokenizers::Tokenizer::from_file(tokenizer_path)
		.map_err(|e| ModelError::Tokenizer(e.to_string()))
}

/// Get the default device with runtime detection and fallback
pub fn get_device() -> Device {
	device::get_device_with_fallback()
}

/// Get information about the detected compute device
pub fn get_device_info() -> DeviceInfo {
	device::detect_device()
}
