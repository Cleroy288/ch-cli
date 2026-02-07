//! Model Loading Utilities
//!
//! Provides utilities for loading ML models from HuggingFace Hub
//! with caching and memory-mapping support.
//! Supports both single-file and sharded model weights.

pub mod cache;
mod cache_default;
mod cache_ops;
pub mod device;
mod device_platform;
mod weights;

use std::collections::HashSet;
use std::path::PathBuf;

use hf_hub::api::sync::Api;
use serde::Deserialize;

pub use cache::ModelCache;
pub use device::{get_device, get_device_info, DeviceInfo, DeviceType};
pub use weights::{load_weights, load_weights_multi};

// Re-export errors from domain for backward compat
pub use crate::domain::errors::model::{
	ModelError, ModelResult,
};

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
fn download_weights(
	repo: &hf_hub::api::sync::ApiRepo,
) -> ModelResult<Vec<PathBuf>> {
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
			.map_err(|e| {
				let msg = format!("index parse: {}", e);
				ModelError::WeightLoad(msg)
			})?;

		// get unique shard filenames
		let shard_names: HashSet<&String> = index.weight_map.values().collect();
		let mut shard_paths = Vec::new();

		for shard_name in &shard_names {
			eprintln!("[models] Downloading shard: {}", shard_name);
			let path = repo
				.get(shard_name)
				.map_err(|e| {
				let msg = format!("shard {}: {}", shard_name, e);
				ModelError::FileNotFound(msg)
			})?;
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

	let msg = "No model weights found \
		(tried model.safetensors, sharded index, \
		pytorch_model.bin)";
	Err(ModelError::FileNotFound(msg.to_string()))
}

/// Load tokenizer from path
pub fn load_tokenizer(
	tokenizer_path: &PathBuf,
) -> ModelResult<tokenizers::Tokenizer> {
	tokenizers::Tokenizer::from_file(tokenizer_path)
		.map_err(|e| ModelError::Tokenizer(e.to_string()))
}
