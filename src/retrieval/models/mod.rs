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
mod dtype_select;
mod gguf_download;
mod weights;

use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;

use hf_hub::api::sync::Api;
use serde::Deserialize;

pub use cache::ModelCache;
pub use device::{get_device, get_device_info, DeviceInfo, DeviceType};
pub use dtype_select::embedding_dtype;
pub use gguf_download::{download_gguf_model, GgufModelInfo};
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
	/// metadata about the sharded model (part of JSON format)
	#[serde(default)]
	_metadata: Option<ShardMetadata>,
	/// mapping from tensor name to shard filename
	weight_map: std::collections::HashMap<String, String>,
}

/// Metadata in shard index
#[derive(Deserialize)]
struct ShardMetadata {
	/// total size in bytes
	#[serde(default)]
	_total_size: u64,
}

/// Download model files from HuggingFace Hub
/// Handles both single-file and sharded models
pub fn download_model(
	model_id: &str,
) -> ModelResult<ModelInfo> {
	let api = Api::new()
		.map_err(|err| ModelError::Hub(err.to_string()))?;
	let repo = api.model(model_id.to_string());

	let tokenizer_path = download_file(
		&repo, "tokenizer.json", "tokenizer",
	)?;
	let config_path = download_file(
		&repo, "config.json", "config",
	)?;
	let weights_paths = download_weights(&repo)?;

	Ok(ModelInfo {
		model_id: model_id.to_string(),
		weights_paths,
		tokenizer_path,
		config_path,
	})
}

/// Download a single file from a HuggingFace repo
fn download_file(
	repo: &hf_hub::api::sync::ApiRepo,
	filename: &str,
	label: &str,
) -> ModelResult<PathBuf> {
	repo.get(filename).map_err(|err| {
		ModelError::FileNotFound(format!(
			"{}: {}",
			label, err
		))
	})
}

/// Download model weights, handling single/sharded/pytorch
fn download_weights(
	repo: &hf_hub::api::sync::ApiRepo,
) -> ModelResult<Vec<PathBuf>> {
	// Try single safetensors file
	if let Ok(path) = repo.get("model.safetensors") {
		log_model_msg("Found single model.safetensors");
		return Ok(vec![path]);
	}

	// Try sharded safetensors
	if let Some(paths) = try_sharded_download(repo)? {
		return Ok(paths);
	}

	// Try pytorch file (fallback)
	if let Ok(path) = repo.get("pytorch_model.bin") {
		log_model_msg("Found pytorch_model.bin");
		return Ok(vec![path]);
	}

	let msg = "No model weights found \
		(tried safetensors, sharded, pytorch)";
	Err(ModelError::FileNotFound(msg.to_string()))
}

/// Try downloading sharded safetensors model
fn try_sharded_download(
	repo: &hf_hub::api::sync::ApiRepo,
) -> ModelResult<Option<Vec<PathBuf>>> {
	let index_file = "model.safetensors.index.json";
	let index_path = match repo.get(index_file) {
		Ok(path) => path,
		Err(_) => return Ok(None),
	};

	log_model_msg("Found sharded model, downloading...");
	let index = parse_shard_index(&index_path)?;
	let paths = download_shards(repo, &index)?;
	Ok(Some(paths))
}

/// Parse shard index from JSON file
fn parse_shard_index(
	index_path: &PathBuf,
) -> ModelResult<ShardIndex> {
	let content = std::fs::read_to_string(index_path)?;
	serde_json::from_str(&content).map_err(|err| {
		ModelError::WeightLoad(format!(
			"index parse: {}",
			err
		))
	})
}

/// Download individual shard files from the index
fn download_shards(
	repo: &hf_hub::api::sync::ApiRepo,
	index: &ShardIndex,
) -> ModelResult<Vec<PathBuf>> {
	let names: HashSet<&String> =
		index.weight_map.values().collect();
	let mut paths = Vec::new();

	for name in &names {
		log_model_msg(&format!("Downloading: {}", name));
		let path = repo.get(name).map_err(|err| {
			ModelError::FileNotFound(format!(
				"shard {}: {}",
				name, err
			))
		})?;
		paths.push(path);
	}

	paths.sort();
	log_model_msg(&format!(
		"Downloaded {} shards",
		paths.len()
	));
	Ok(paths)
}

/// Log a model-related message to stderr
fn log_model_msg(msg: &str) {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[models] {}",
		msg,
	);
}

/// Load tokenizer from path
pub fn load_tokenizer(
	tokenizer_path: &PathBuf,
) -> ModelResult<tokenizers::Tokenizer> {
	tokenizers::Tokenizer::from_file(tokenizer_path)
		.map_err(|err| ModelError::Tokenizer(err.to_string()))
}
